use std::{collections::HashMap, fmt::Display, io, str::FromStr};
use super::response::HttpResponse;

#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    pub resource: Resource,
    version: Version,
    headers: HttpHeader,
    pub request_body: String,
}

impl HttpRequest {
    pub fn response(&self) -> io::Result<HttpResponse> {
        HttpResponse::new(self)
    }

    pub fn new(request: &str) -> io::Result<HttpRequest> {
        let method = Method::new(request);
        let resource = Resource::new(request).unwrap_or_else(|| Resource {
            path: "".to_string(),
        });
        let version = Version::new(request)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.msg))?;
        let headers = HttpHeader::new(request).unwrap_or(HttpHeader {
            headers: HashMap::new(),
        });
        let request_body = request.split_once("\r\n\r\n").map_or(String::new(), |(_, body)| body.to_string());

        Ok(HttpRequest {
            method,
            resource,
            version,
            headers,
            request_body,
        })
    }
}

#[derive(Debug)]
struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    pub fn new(request: &str) -> Option<HttpHeader> {
        let mut httpheader = HttpHeader {
            headers: HashMap::new(),
        };
        let (_, header_str) = request.split_once("\r\n")?;
        for line in header_str.split_terminator("\r\n") {
            if line.is_empty() {
                break;
            }
            let (header, value) = line.split_once(":")?;
            httpheader.headers.insert(header.trim().to_string(), value.trim().to_string());
        }
        Some(httpheader)
    }
}

#[derive(Debug)]
pub enum Version {
    V1_1,
    V2_0,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Version::V1_1 => "HTTP/1.1",
            Version::V2_0 => "HTTP/2",
        };
        write!(f, "{}", msg)
    }
}

pub struct VersionError {
    msg: String,
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Version {
    pub fn new(request: &str) -> Result<Self, VersionError> {
        Version::from_str(request)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(request: &str) -> Result<Self, Self::Err> {
        let request_split = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let splits = method_line.split_ascii_whitespace();
            for split in splits {
                if split == "HTTP/1.1" {
                    return Ok(Version::V1_1);
                } else if split == "HTTP/2" || split == "HTTP/2.0" {
                    return Ok(Version::V2_0);
                };
            }
        }
        let invalid = format!("Unknown protocol version in {}", request);
        let version_error = VersionError { msg: invalid };
        Err(version_error)
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Uninitialized,
}

impl Method {
    pub fn new(request: &str) -> Method {
        if let Some((method_line, _)) = request.split_once("\r\n") {
            if let Some((method, _)) = method_line.split_once(' ') {
                return match method {
                    "GET" => Method::Get,
                    "POST" => Method::Post,
                    _ => Method::Uninitialized,
                };
            }
        }
        Method::Uninitialized
    }
}

#[derive(Debug)]
pub struct Resource {
    pub path: String,
}

impl Resource {
    pub fn new(request: &str) -> Option<Resource> {
        request.split_once("\r\n").and_then(|(method_line, _)| {
            method_line.split_once(' ').and_then(|(_, rest)| {
                rest.split_once(' ').map(|(resource, _)| {
                    Resource {
                        path: resource.trim_start_matches('/').to_string(),
                    }
                })
            })
        })
    }
}
