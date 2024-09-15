use std::cell::Cell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

struct HTTPHeader<'a> {
    name: &'a str,
    value: &'a str,
}

impl Display for HTTPHeader<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

struct HTTPHeaders<'a>(Vec<HTTPHeader<'a>>);

impl HTTPHeaders<'_> {
    fn from_raw(payload: Rc<String>) -> Self {
        let mut headers: Vec<HTTPHeader> = vec![];
        let skip_lines: u32 = 2;
        let mut line_number: u32 = 0;
        let separator = ":";
        let split_payload = payload.split("\n").collect::<Vec<&str>>();
        // for line in split_payload{
        //     line_number += 1;
        //     if line_number <= skip_lines {
        //         continue;
        //     }
        //     if line == "" {
        //         break;
        //     }
        //     let split_line = line.split(separator).collect::<Vec<&str>>();
        //     headers.push(
        //         HTTPHeader {
        //             name: split_line[0],
        //             value: split_line[1],
        //         }
        //     )
        // }
        headers.push(
            HTTPHeader {
                name: &payload[1..10],
                value: &payload[10..20],
            }
        );
        HTTPHeaders(headers)
    }
}

impl Display for HTTPHeaders<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for header in &self.0 {
            write!(f, "{}", header)?
        }
        Ok(())
    }
}

pub struct HTTPResponseParser<'a> {
    payload: Box<[u8]>,
    payload_text: Rc<String>,
    headers: Rc<HTTPHeaders<'a>>,
    // body: Box<[u8]>,
    // text: Box<str>,
}

impl HTTPResponseParser<'_> {
    pub fn new(raw_response: Box<[u8]>) -> Result<Self, Box<dyn Error>> {
        let text_response = Rc::new(String::from_utf8(raw_response.to_vec()).unwrap());
        Ok(HTTPResponseParser {
            payload: raw_response,
            payload_text: text_response.clone(),
            headers: Rc::new(HTTPHeaders::from_raw(text_response.clone())),
        })
    }
    pub fn get_text(&self) -> &str {
        self.payload_text.clone().as_str()
    }

    pub fn get_headers(&self) -> Rc<HTTPHeaders> {
        self.headers.clone()
    }
}