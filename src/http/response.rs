use super::request::HttpRequest;
use super::request::Version;
use infer;
use percent_encoding::percent_decode_str;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use url_escape::encode_component;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: Version,
    pub status: ResponseStatus,
    pub content_length: usize,
    pub accept_ranges: AcceptRanges,
    pub response_body: Vec<u8>,
    pub current_path: String,
    pub content_type: String,
}

impl HttpResponse {
    pub fn new(request: &HttpRequest) -> io::Result<HttpResponse> {
        let version: Version = Version::V1_1;
        let mut status: ResponseStatus = ResponseStatus::NotFound;
        let mut content_length: usize = 0;
        let mut accept_ranges: AcceptRanges = AcceptRanges::None;
        let mut content_type: String = String::new();
        let mut response_body: Vec<u8> = Vec::new();
        let current_path: String = request.resource.path.clone();

        let server_root_path = std::env::current_dir()?;
        let resource = percent_decode_str(&request.resource.path).decode_utf8_lossy();
        let new_path = server_root_path.join(&*resource);

        let rootcwd_len = server_root_path.canonicalize()?.components().count();
        let resource_len = new_path.canonicalize()?.components().count();

        if rootcwd_len > resource_len {
            status = ResponseStatus::NotFound;
            return Ok(HttpResponse {
                version,
                status,
                content_length: 0,
                accept_ranges,
                response_body: Vec::new(),
                current_path,
                content_type: "text/plain".to_string(),
            });
        }

        let base_url = "http://localhost:5500";

        if new_path.exists() {
            if new_path.is_file() {
                let mut file = File::open(&new_path)?;
                let mut content: Vec<u8> = Vec::new();
                file.read_to_end(&mut content)?;

                content_length = content.len();
                status = ResponseStatus::OK;
                accept_ranges = AcceptRanges::Bytes;

                if let Some(file_type) = infer::get(&content) {
                    content_type = file_type.mime_type().to_string();
                } else if new_path.extension().and_then(|ext| ext.to_str()) == Some("txt")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("rs")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("lock")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("png")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("json")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("TAG")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("toml")
                    || new_path.extension().and_then(|ext| ext.to_str()) == Some("md")
                {
                    content_type = "text/plain".to_string();
                } else {
                    content_type = "application/octet-stream".to_string();
                }

                response_body = content;
            } else if new_path.is_dir() {
                status = ResponseStatus::OK;
                content_type = "text/html".to_string();

                let mut begin_html = r#"
                <!DOCTYPE html> 
                <html> 
                <head> 
                    <meta charset="utf-8"> 
                </head> 
                <body>"#
                    .to_string();

                let decoded_path = resource.replace("%2F", "/");

                let one_step_back_path = {
                    let components: Vec<&str> = resource.split('/').collect();
                    if components.len() >= 1 {
                        components[..components.len() - 1].join("/")
                    } else {
                        String::from("/")
                    }
                };

                let go_back_link = format!(
                    "<a href=\"{}/{}\">Go back up a directory</a>",
                    base_url,
                    encode_component(&one_step_back_path)
                );

                let header = format!(
                    "<h1>Currently in {}</h1>{}<br><hr>",
                    decoded_path, go_back_link
                );
                begin_html.push_str(&header);

                for entry in WalkDir::new(new_path).max_depth(1).min_depth(1) {
                    let entry = entry.unwrap();
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let file_url = encode_component(&file_name);

                    if entry.path().is_dir() {
                        begin_html.push_str(&format!(
                            "<div><a href=\"{}/{}\">{}/</a></div>",
                            base_url,
                            resource.to_string() + "/" + &file_url,
                            file_name
                        ));
                    } else {
                        begin_html.push_str(&format!(
                            "<div><a href=\"{}/{}\">{}</a></div>",
                            base_url,
                            resource.to_string() + "/" + &file_url,
                            file_name
                        ));
                    }
                }

                let end_html = r#"
                </body>
                </html>"#
                    .to_string();

                let full_html = begin_html + &end_html;
                response_body = full_html.into_bytes();
                content_length = response_body.len();
            }
        } else {
            // Return 404 Not Found if the file or directory doesn't exist
            status = ResponseStatus::NotFound;
            content_type = "text/html".to_string();
            let not_found_body = format!(
                "<html><body><h1>404 Not Found</h1><p>The requested resource <strong>{}</strong> was not found on this server.</p></body></html>",
                request.resource.path
            );
            response_body = not_found_body.into_bytes();
            content_length = response_body.len();
        }

        Ok(HttpResponse {
            version,
            status,
            content_length,
            accept_ranges,
            response_body,
            current_path,
            content_type,
        })
    }
}

#[derive(Debug)]
enum ResponseStatus {
    OK = 200,
    NotFound = 404,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: &str = match self {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::NotFound => "404 Not Found",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
enum AcceptRanges {
    Bytes,
    None,
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: &str = match self {
            AcceptRanges::Bytes => "Accept-Ranges: bytes",
            AcceptRanges::None => "Accept-Ranges: none",
        };
        write!(f, "{}", msg)
    }
}
