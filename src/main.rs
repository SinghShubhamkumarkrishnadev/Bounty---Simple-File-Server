use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    path::Path,
    fs,
    env,
};

use simple_http::http::{request, response};

fn create_socket() -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let buf_str = String::from_utf8_lossy(&buffer);
    let request = request::HttpRequest::new(&buf_str)?;

    let response = request.response()?;

    println!("{:?}", &response);

    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        response.content_length, response.content_type
    );

    stream.write(headers.as_bytes())?;
    stream.write(&response.response_body)?;
    stream.flush()?;

    Ok(())
}

fn serve(socket: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    let mut counter = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                match std::thread::spawn(move || handle_client(&mut stream)).join() {
                    Ok(_) => {
                        counter += 1;
                        println!("connected stream... {}", counter);
                    }
                    Err(_) => continue,
                };
            }
            Err(e) => {
                eprintln!("Failed to accept a client: {}", e);
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let socket = create_socket();
    serve(socket)?;
    Ok(())
}
