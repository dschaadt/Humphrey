use crate::http::headers::RequestHeader;
use crate::http::request::{Request, RequestError};
use crate::http::response::Response;
use crate::http::status::StatusCode;
use crate::route::RouteHandler;

use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

/// Represents the Humphrey app.
///
/// The type parameter represents the app state, which is shared between threads.
/// It must implement the `Send` trait as well as `Default` for setting initial values.
/// The state is given to every request as an `Arc<Mutex<State>>`.
pub struct App<State>
where
    State: Send + Default + 'static,
{
    routes: Vec<RouteHandler<State>>,
    error_handler: ErrorHandler,
    state: Arc<Mutex<State>>,
}

/// Represents a function able to handle a request.
/// It is passed a reference to the request as well as the app's state, and must return a response.
///
/// ## Example
/// The most basic request handler would be as follows:
/// ```
/// fn handler(request: &Request, _: Arc<Mutex<()>>) -> Response {
///     Response::new(StatusCode::OK) // create the response
///         .with_bytes(b"<html><body><h1>Success</h1></body></html>".to_vec()) // add the body
///         .with_request_compatibility(request) // ensure compatibility with the request
///         .with_generated_headers() // generate required headers
/// }
/// ```
pub type RequestHandler<State> = fn(&Request, Arc<Mutex<State>>) -> Response;

/// Represents a function able to handle an error.
/// The first parameter of type `Option<&Request>` will be `Some` if the request could be parsed.
/// Otherwise, it will be `None` and the status code will be `StatusCode::BadRequest`.
///
/// Every app has a default error handler, which simply displays the status code.
/// The source code for this default error handler is copied below since it is a good example.
///
/// ## Example
/// ```
/// fn error_handler(request: Option<&Request>, status_code: StatusCode) -> Response {
///     let body = format!(
///         "<html><body><h1>{} {}</h1></body></html>",
///         Into::<u16>::into(status_code.clone()),
///         Into::<&str>::into(status_code.clone())
///     );
///
///     if let Some(request) = request {
///         Response::new(status_code)
///             .with_bytes(body.as_bytes().to_vec())
///             .with_request_compatibility(request)
///             .with_generated_headers()
///     } else {
///         Response::new(status_code)
///             .with_bytes(body.as_bytes().to_vec())
///             .with_generated_headers()
///     }
/// }
/// ```
pub type ErrorHandler = fn(Option<&Request>, StatusCode) -> Response;

/// Represents a generic error with the program.
pub type HumphreyError = Box<dyn std::error::Error>;

impl<State> App<State>
where
    State: Send + Default + 'static,
{
    /// Initialises a new Humphrey app.
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            error_handler,
            state: Arc::new(Mutex::new(State::default())),
        }
    }

    /// Runs the Humphrey app on the given socket address.
    /// This function will only return if a fatal error is thrown such as the port being in use.
    pub fn run(self, addr: &SocketAddr) -> Result<(), HumphreyError> {
        let socket = TcpListener::bind(addr)?;
        let routes = Arc::new(self.routes);
        let error_handler = Arc::new(self.error_handler);

        for stream in socket.incoming() {
            match stream {
                Ok(stream) => {
                    let cloned_routes = routes.clone();
                    let cloned_error_handler = error_handler.clone();
                    let cloned_state = self.state.clone();
                    spawn(move || {
                        client_handler(stream, cloned_routes, cloned_error_handler, cloned_state)
                    });
                }
                Err(_) => (),
            }
        }

        Ok(())
    }

    /// Adds a route and associated handler to the server.
    /// Routes can include wildcards, for example `/blog/*`.
    ///
    /// ## Panics
    /// This function will panic if the route string cannot be converted to a `Uri` object.
    pub fn with_route(mut self, route: &str, handler: RequestHandler<State>) -> Self {
        self.routes.push(RouteHandler {
            route: route.parse().unwrap(),
            handler,
        });
        self
    }

    /// Sets the error handler for the server.
    pub fn with_error_handler(mut self, handler: ErrorHandler) -> Self {
        self.error_handler = handler;
        self
    }
}

/// Handles a connection with a client.
/// The connection will be opened upon the first request and closed as soon as a request is
///   recieved without the `Connection: Keep-Alive` header.
fn client_handler<State>(
    mut stream: TcpStream,
    routes: Arc<Vec<RouteHandler<State>>>,
    error_handler: Arc<ErrorHandler>,
    state: Arc<Mutex<State>>,
) {
    loop {
        let request = Request::from_stream(&mut stream);
        let cloned_state = state.clone();

        if match &request {
            Ok(_) => false,
            Err(e) => e == &RequestError::Stream,
        } {
            break;
        }

        let response = match &request {
            Ok(request) => match routes.iter().find(|r| r.route.matches(&request.uri)) {
                Some(handler) => (handler.handler)(request, cloned_state),
                None => error_handler(Some(request), StatusCode::NotFound),
            },
            Err(_) => error_handler(None, StatusCode::BadRequest),
        };

        let response_bytes: Vec<u8> = response.into();
        if let Err(_) = stream.write(&response_bytes) {
            break;
        };

        if let Ok(request) = request {
            if let Some(connection) = request.headers.get(&RequestHeader::Connection) {
                if connection.to_ascii_lowercase() != "keep-alive" {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

/// The default error handler for every Humphrey app.
/// This can be overridden by using the `with_error_handler` method when building the app.
fn error_handler(request: Option<&Request>, status_code: StatusCode) -> Response {
    let body = format!(
        "<html><body><h1>{} {}</h1></body></html>",
        Into::<u16>::into(status_code.clone()),
        Into::<&str>::into(status_code.clone())
    );

    if let Some(request) = request {
        Response::new(status_code)
            .with_bytes(body.as_bytes().to_vec())
            .with_request_compatibility(request)
            .with_generated_headers()
    } else {
        Response::new(status_code)
            .with_bytes(body.as_bytes().to_vec())
            .with_generated_headers()
    }
}