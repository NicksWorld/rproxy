pub extern crate url;

use url::{Url, ParseError};

use std::error;
use std::fmt;

use std::collections::BTreeMap;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RequestParseError;

impl fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "Invalid http request sent to Request::FromStr")
    }
}

impl error::Error for RequestParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	None
    }
}


#[derive(Debug)]
pub struct Request {
    pub url: Url,
    pub method: String,
    pub headers: BTreeMap<String, String>,
    pub body: String
}


/*
Example data:

POST http://example.com/ HTTP/1.1
Host: example.com
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:68.0) Gecko/20100101 Firefox/68.0
Accept: text/html
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Content-Type: text/plain;charset=UTF-8
Content-Length: 17
Origin: http://www.example.com
Connection: keep-alive
Referer: http://www.example.com/

{"hello":"world"}
*/
impl FromStr for Request {
    type Err = RequestParseError;

    fn from_str(req_str: &str) -> Result<Self, Self::Err> {
	// Seperate the data by \r\n
	let req_lines: Vec<&str> = req_str.split("\r\n").collect();

	// Parse the first line: POST http://example.com/ HTTP/1.1
	let first_line: Vec<&str> = req_lines[0].split(" ").collect();
	if first_line.len() != 3 {
	    return Err(RequestParseError)
	}
	let method = String::from(first_line[0]);
	let url = Url::parse(first_line[1]).unwrap();

	// Capture the headers, and store the line the body starts
	let mut headers = BTreeMap::new();
	let mut body_start = 0;
	for line_num in 1..req_lines.len() {
	    if req_lines[line_num] == "" {
		body_start = line_num + 1;
		break;
	    } else {
		let header: Vec<&str> = req_lines[line_num]
		    .split(": ")
		    .collect();
		headers.insert(String::from(header[0]), String::from(header[1..].join(": ")));
	    }
	}

	// Format the body (if available) and remove null characters (\u{0})
	let mut body = String::from("");
	if body_start != 0 {
	    body = String::from(
		req_lines[body_start..]
		    .join("\r\n")
		    .trim_matches(char::from(0))
	    );
	}
	
	Ok(Request {
	    url,
	    method,
	    body,
	    headers
	})
    }
}
