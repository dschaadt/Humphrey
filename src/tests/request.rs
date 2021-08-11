#![allow(dead_code, unused_imports)]
use crate::http::headers::RequestHeader;
use crate::http::method::Method;
use crate::http::Request;
use std::{collections::BTreeMap, io::Read, net::SocketAddr};

struct MockStream {
    data: Vec<u8>,
}

impl MockStream {
    fn with_data(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut index = 0;
        for byte in buf {
            *byte = self.data[index];
            index += 1;
            if index == self.data.len() {
                break;
            }
        }

        std::io::Result::Ok(self.data.len())
    }
}

#[test]
fn test_request_from_stream() {
    let test_data = b"GET /testpath HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let mut stream = MockStream::with_data(test_data.to_vec());
    let request = Request::from_stream(&mut stream, "1.2.3.4:5678".parse().unwrap());

    assert!(request.is_ok());

    let request = request.unwrap();
    let expected_uri: String = "/testpath".into();
    assert_eq!(request.method, Method::Get);
    assert_eq!(request.uri, expected_uri);
    assert_eq!(request.version, "HTTP/1.1");
    assert_eq!(request.content, None);
    assert_eq!(request.address.to_string(), "1.2.3.4:5678");

    let mut expected_headers: BTreeMap<RequestHeader, String> = BTreeMap::new();
    expected_headers.insert(RequestHeader::Host, "localhost".to_string());
    assert_eq!(request.headers, expected_headers);
}
