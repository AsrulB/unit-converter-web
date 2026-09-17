use crate::unit_data::ConversionData;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod unit;
pub mod unit_data;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    let value_storage: Value = Default::default();
    match stream.read(&mut buffer) {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Request:\n{}\n", request);

            // Extract first line of HTTP request (e.g., "GET / HTTP/1.1")
            let mut lines = request.lines();
            let request_line = match lines.next() {
                None => return,
                Some(line) => line,
            };

            let parts: Vec<&str> = request_line.split_whitespace().collect();
            if parts.len() < 2 {
                return;
            }

            let method = parts[0];
            let path_and_query = parts[1];

            println!("method: {}, query: {}", method, path_and_query);

            // GET request - serve index.html
            if method == "GET" && path_and_query == "/" {
                display_home(&mut stream);
            } else if method == "POST" && path_and_query == "/submit-conversion" {
                process_data(&mut stream, &request, value_storage);
            } else if method == "GET" && path_and_query.starts_with("/conversion") {
                let template = fs::read_to_string("result.html").unwrap_or_default();

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    template.len(),
                    template
                );

                println!("why not render this");
                stream.write_all(response.as_bytes()).ok();
                stream.flush().ok();
            } else {
                let error_json = json!({
                    "success": false,
                    "error": "Invalid URL"
                });
                send_json_response(&mut stream, 404, &error_json);
            }

            // Check if it's a POST request with JSON body (form submission)
            // if request.starts_with("POST") {
            //     process_data(&mut stream, &request);
            // }
            // GET request - serve index.html
            // else if request.starts_with("GET") {
            //     display_home(&mut stream);
            // }
        }
        Err(e) => {
            eprintln!("Read error: {}", e);
        }
    }
}

fn process_data(stream: &mut TcpStream, request: &Cow<'_, str>, mut _storage: Value) {
    if let Some(body) = extract_body(&request) {
        println!("Body request {}", body);

        match serde_json::from_str::<Value>(&body) {
            Err(e) => {
                eprintln!("JSON parse error: {}", e);
                let error_json = json!({
                    "success": false,
                    "error": "Invalid JSON format"
                });
                send_json_response(stream, 400, &error_json);
            }
            Ok(value) => match ConversionData::run(value) {
                None => {
                    eprintln!("Unit conversion error!");
                    let error_json = json!({
                        "success": false,
                        "error": "Invalid Unit Conversion"
                    });

                    send_json_response(stream, 400, &error_json);
                }
                Some(data) => {
                    _storage = data.clone();

                    if let Some(datas) = data.get("data") {
                        println!("{}", datas.to_string());
                        let from = datas.get("from").unwrap();
                        let to = datas.get("to").unwrap();
                        let value = datas.get("value").unwrap();
                        // send_json_response(stream, 200, &data);

                        let redirect_path =
                            format!("/conversion?value={}&from={}&to={}", value, from, to);
                        let response = json!({
                            "success": true,
                            "redirect_url": redirect_path
                        });

                        send_json_response(stream, 200, &response);
                    }
                }
            },
        }
    }
}

fn display_home(stream: &mut TcpStream) {
    let content = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write_all(response.as_bytes()).ok();
    stream.flush().ok();
}

// Extract JSON body from HTTP request
fn extract_body(request: &str) -> Option<String> {
    let parts: Vec<&str> = request.split("\r\n\r\n").collect();
    if parts.len() > 1 {
        Some(parts[1].trim().to_string())
    } else {
        None
    }
}

// Send JSON response back to client
fn send_json_response(stream: &mut TcpStream, status_code: u16, data: &Value) {
    let status_text = match status_code {
        200 => "OK",
        400 => "Bad Request",
        _ => "Internal Server Error",
    };

    let json_body = data.to_string();
    let content_length = json_body.len();

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        status_code, status_text, content_length, json_body
    );

    stream.write_all(response.as_bytes()).ok();
    stream.flush().ok();
}
