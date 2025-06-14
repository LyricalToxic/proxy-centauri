use encoding::all::UTF_8;
use encoding::{DecoderTrap, Encoding};
use std::error::Error;
use std::fmt::{write, Display, Formatter};

pub(crate) trait HTTPParser {
    type Output;
    fn parse(&self) -> Self::Output;
}

#[derive(Eq, PartialEq, Debug)]
pub struct HTTPUrl {
    url: String,
}

#[derive(Eq, PartialEq, Debug)]
pub struct HTTPBody {
    body: [u8; 4096],
}
// ===== HTTPRequest ====
#[derive(Debug)]
pub struct HTTPRequest {
    pub url: HTTPUrl,
    pub headers: HTTPHeaders,
    pub body: HTTPBody,
}
impl Display for HTTPRequest{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HTTP response")?;
        writeln!(f, "\tUrl: {:?}", self.url)?;
        writeln!(f, "\tHeaders: \n{}", self.headers)?;
        // writeln!(f, "\tBody: {:<20?}", self.body)
        Ok(())
    }
}
impl PartialEq for HTTPRequest {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.headers == other.headers && self.body == other.body
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// HTTPHeaders
#[derive(Clone, Debug, PartialEq)]
pub struct HTTPHeaders(Vec<HTTPHeader>);

impl Display for HTTPHeaders{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for header in self.0.iter(){
            write!(f, "{header}")?;
        }
        Ok(())
    }
}

// ===== HTTPRequestParser ====
pub struct HTTPRequestParser {
    buf: [u8; 4096],
}
impl HTTPRequestParser {
    pub(crate) fn new(buf: &[u8; 4096]) -> Self {
        HTTPRequestParser { buf: *buf }
    }
}
impl HTTPParser for HTTPRequestParser {
    type Output = HTTPRequest;

    fn parse(&self) -> Self::Output {
        HTTPRequest {
            url: HTTPUrl {
                url: String::from(""),
            },
            headers: HTTPHeaderParser::new(&self.buf).parse(),
            body: HTTPBodyParser::new(&self.buf).parse(),
        }
    }
}

// ===== HTTPHeaderParser ====
struct HTTPHeaderParser {
    buf: [u8; 4096],
}

impl HTTPHeaderParser {
    pub(crate) fn new(buf: &[u8; 4096]) -> Self {
        HTTPHeaderParser { buf: *buf }
    }
}
impl HTTPParser for HTTPHeaderParser {
    type Output = HTTPHeaders;

    fn parse(&self) -> Self::Output {
        let mut headers: Vec<HTTPHeader> = vec![];
        let skip_lines: u32 = 1;
        let mut line_number: u32 = 0;
        let separator = ":";
        let text = UTF_8.decode(&self.buf, DecoderTrap::Strict).unwrap();
        for line in text.split("\n") {
            line_number += 1;
            if line_number <= skip_lines {
                continue;
            }
            if line == "" {
                break;
            }
            if let Some((name, value)) = line.split_once(separator) {
                headers.push(HTTPHeader {
                    name: String::from(name.trim()),
                    value: String::from(value.trim()),
                })
            }
        }
        HTTPHeaders(headers)
    }
}

// ===== HTTPBodyParser ====
struct HTTPBodyParser {
    buf: [u8; 4096],
}

impl HTTPBodyParser {
    pub fn new(buf: &[u8; 4096]) -> Self {
        HTTPBodyParser { buf: *buf }
    }
}

impl HTTPParser for HTTPBodyParser{

    type Output = HTTPBody;
    fn parse(&self) -> Self::Output {

        HTTPBody{
            body: self.buf
        }
    }
}

// ===== HTTPHeader ====
#[derive(PartialEq, Clone, Debug)]
struct HTTPHeader {
    name: String,
    value: String,
}

impl Display for HTTPHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::http::http_parser::{
        HTTPBody, HTTPHeader, HTTPHeaderParser, HTTPHeaders, HTTPRequestParser, HTTPUrl,
    };
    use crate::http::http_parser::{HTTPParser, HTTPRequest};

    #[test]
    fn parse_valid_http_headers() {
        let http_request_raw = String::from(
            "
            GET / HTTP/1.1
            Content-Type: application/json
            User-Agent: PostmanRuntime/7.39.0
            Accept: */*
            Host: 127.0.0.1:8081
            Accept-Encoding: gzip, deflate, br
            Connection: keep-alive
            Content-Length: 3

            END
            "
            .trim(),
        );
        let mut http_request_buf: [u8; 4096] = [0; 4096];
        http_request_buf[..http_request_raw.len()].copy_from_slice(http_request_raw.as_bytes());

        let expected_headers = HTTPHeaders(vec![
            HTTPHeader {
                name: String::from("Content-Type"),
                value: String::from("application/json"),
            },
            HTTPHeader {
                name: String::from("User-Agent"),
                value: String::from("PostmanRuntime/7.39.0"),
            },
            HTTPHeader {
                name: String::from("Accept"),
                value: String::from("*/*"),
            },
            HTTPHeader {
                name: String::from("Host"),
                value: String::from("127.0.0.1:8081"),
            },
            HTTPHeader {
                name: String::from("Accept-Encoding"),
                value: String::from("gzip, deflate, br"),
            },
            HTTPHeader {
                name: String::from("Connection"),
                value: String::from("keep-alive"),
            },
            HTTPHeader {
                name: String::from("Content-Length"),
                value: String::from("3"),
            },
        ]);

        let result_headers = HTTPHeaderParser::new(&http_request_buf).parse();
        assert_eq!(result_headers, expected_headers)
    }
    #[test]
    fn parse_valid_http_request() {
        let http_request_raw = String::from(
            "
            GET / HTTP/1.1
            Content-Type: application/json
            User-Agent: PostmanRuntime/7.39.0
            Accept: */*
            Host: 127.0.0.1:8081
            Accept-Encoding: gzip, deflate, br
            Connection: keep-alive
            Content-Length: 3

            END
            "
            .trim(),
        );
        let mut http_request_buf: [u8; 4096] = [0; 4096];
        http_request_buf[..http_request_raw.len()].copy_from_slice(http_request_raw.as_bytes());
        let parsed_request: HTTPRequest = HTTPRequestParser::new(&http_request_buf).parse();

        let body_str = String::from("END");
        let mut body_buf: [u8; 4096] = [0; 4096];
        body_buf[..body_str.len()].copy_from_slice(body_str.as_bytes());
        let expected_request = HTTPRequest {
            url: HTTPUrl {
                url: String::from(""),
            },
            headers: HTTPHeaders(vec![
                HTTPHeader {
                    name: String::from("Content-Type"),
                    value: String::from("application/json"),
                },
                HTTPHeader {
                    name: String::from("User-Agent"),
                    value: String::from("PostmanRuntime/7.39.0"),
                },
                HTTPHeader {
                    name: String::from("Accept"),
                    value: String::from("*/*"),
                },
                HTTPHeader {
                    name: String::from("Host"),
                    value: String::from("127.0.0.1:8081"),
                },
                HTTPHeader {
                    name: String::from("Accept-Encoding"),
                    value: String::from("gzip, deflate, br"),
                },
                HTTPHeader {
                    name: String::from("Connection"),
                    value: String::from("keep-alive"),
                },
                HTTPHeader {
                    name: String::from("Content-Length"),
                    value: String::from("3"),
                },
            ]),
            body: HTTPBody { body: body_buf },
        };
        assert_eq!(parsed_request, expected_request);
    }
}
