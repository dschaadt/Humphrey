use humphrey::http::{Request, Response};
use humphrey::krauss::wildcard_match;
use humphrey::App;

#[cfg(feature = "plugins")]
use crate::plugins::manager::PluginManager;
#[cfg(feature = "plugins")]
use crate::plugins::plugin::PluginLoadResult;
#[cfg(feature = "plugins")]
use std::process::exit;

use crate::cache::Cache;
use crate::config::{BlacklistMode, Config, RouteConfig};
use crate::logger::Logger;
use crate::proxy::proxy_handler;
use crate::r#static::{directory_handler, file_handler, not_found, redirect_handler};
use crate::server::pipe::pipe;

use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

/// Represents the application state.
/// Includes the target directory, cache state, and the logger.
pub struct AppState {
    pub config: Config,
    pub cache: RwLock<Cache>,
    pub logger: Logger,
    #[cfg(feature = "plugins")]
    pub plugin_manager: RwLock<PluginManager>,
}

impl From<Config> for AppState {
    fn from(config: Config) -> Self {
        let cache = RwLock::new(Cache::from(&config));
        let logger = Logger::from(&config);
        Self {
            config,
            cache,
            logger,
            #[cfg(feature = "plugins")]
            plugin_manager: RwLock::new(PluginManager::default()),
        }
    }
}

/// Main function for the static server.
pub fn main(config: Config) {
    let app: App<AppState> = App::new_with_config(config.threads, AppState::from(config))
        .with_connection_condition(verify_connection)
        .with_websocket_handler(websocket_handler)
        .with_route("/*", request_handler);

    let state = app.get_state();

    let addr = format!("{}:{}", state.config.address, state.config.port);
    let logger = &state.logger;
    logger.info("Starting server");

    #[cfg(feature = "plugins")]
    if let Ok(plugins_count) = load_plugins(&state.config, state) {
        logger.info(&format!("Loaded {} plugins", plugins_count))
    } else {
        exit(1);
    };

    logger.info(&format!("Running at {}", addr));
    logger.debug(&format!("Configuration: {:?}", state.config));

    app.run(addr).unwrap();
}

/// Verifies that the client is allowed to connect by checking with the blacklist config.
fn verify_connection(stream: &mut TcpStream, state: Arc<AppState>) -> bool {
    if let Ok(address) = stream.peer_addr() {
        if state.config.blacklist.mode == BlacklistMode::Block
            && state.config.blacklist.list.contains(&address.ip())
        {
            state.logger.warn(&format!(
                "{}: Blacklisted IP attempted to connect",
                &address.ip()
            ));
            return false;
        }
    } else {
        state.logger.warn("Corrupted stream attempted to connect");
        return false;
    }

    true
}

#[cfg(feature = "plugins")]
fn request_handler(mut request: Request, state: Arc<AppState>) -> Response {
    let plugins = state.plugin_manager.read().unwrap();

    let mut response = plugins
        .on_request(&mut request, state.clone()) // If the plugin overrides the response, return it
        .unwrap_or_else(|| inner_request_handler(request, state.clone())); // If no plugin overrides the response, generate it in the normal way

    // Pass the response to plugins before it is sent to the client
    plugins.on_response(&mut response, state.clone());

    response
}

#[cfg(not(feature = "plugins"))]
fn request_handler(request: Request, state: Arc<AppState>) -> Response {
    inner_request_handler(request, state)
}

fn inner_request_handler(request: Request, state: Arc<AppState>) -> Response {
    for route in &state.config.routes {
        match route {
            RouteConfig::File { matches, file } => {
                if wildcard_match(matches, &request.uri) {
                    return file_handler(request, state.clone(), file);
                }
            }

            RouteConfig::Directory { matches, directory } => {
                if wildcard_match(matches, &request.uri) {
                    return directory_handler(request, state.clone(), directory, matches);
                }
            }

            RouteConfig::Proxy {
                matches,
                load_balancer,
            } => {
                if wildcard_match(matches, &request.uri) {
                    return proxy_handler(request, state.clone(), load_balancer, matches);
                }
            }

            RouteConfig::Redirect { matches, target } => {
                if wildcard_match(matches, &request.uri) {
                    return redirect_handler(request, state.clone(), target);
                }
            }
        }
    }

    not_found(&request)
}

fn websocket_handler(request: Request, mut source: TcpStream, state: Arc<AppState>) {
    let source_addr = request.address.origin_addr.to_string();

    if let Some(address) = &state.config.websocket_proxy {
        let bytes: Vec<u8> = request.into();

        if let Ok(mut destination) = TcpStream::connect(address) {
            // The target was successfully connected to

            destination.write_all(&bytes).unwrap();

            let mut source_clone = source.try_clone().unwrap();
            let mut destination_clone = destination.try_clone().unwrap();
            state.logger.info(&format!(
                "{}: WebSocket connected, proxying data",
                source_addr
            ));

            // Pipe data in both directions
            let forward = spawn(move || pipe(&mut source, &mut destination));
            let backward = spawn(move || pipe(&mut destination_clone, &mut source_clone));

            // Log any errors
            if forward.join().unwrap().is_err() {
                state.logger.error(&format!(
                    "{}: Error proxying WebSocket from client to target, connection closed",
                    source_addr
                ));
            }
            if backward.join().unwrap().is_err() {
                state.logger.error(&format!(
                    "{}: Error proxying WebSocket from target to client, connection closed",
                    source_addr
                ));
            }

            state.logger.info(&format!(
                "{}: WebSocket session complete, connection closed",
                source_addr
            ));
        } else {
            state
                .logger
                .error(&format!("{}: Could not connect to WebSocket", source_addr));
        }
    } else {
        state.logger.warn(&format!(
            "{}: WebSocket connection attempted but no handler provided",
            source_addr
        ))
    }
}

#[cfg(feature = "plugins")]
fn load_plugins(config: &Config, state: &Arc<AppState>) -> Result<usize, ()> {
    let mut manager = state.plugin_manager.write().unwrap();

    for plugin in &config.plugins {
        unsafe {
            let app_state = state.clone();
            match manager.load_plugin(&plugin.library, &plugin.config, app_state) {
                PluginLoadResult::Ok(name) => {
                    state.logger.info(&format!("Initialised plugin {}", name));
                }
                PluginLoadResult::NonFatal(e) => {
                    state
                        .logger
                        .warn(&format!("Non-fatal plugin error in {}", plugin.name));
                    state.logger.warn(&format!("Error message: {}", e));
                    state.logger.warn("Ignoring this plugin");
                }
                PluginLoadResult::Fatal(e) => {
                    state
                        .logger
                        .error(&format!("Could not initialise plugin {}", plugin.name));
                    state.logger.error(&format!("Error message: {}", e));

                    return Err(());
                }
            }
        }
    }

    Ok(manager.plugin_count())
}