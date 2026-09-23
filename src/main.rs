use crate::unit_data::ConversionData;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub mod unit;
pub mod unit_data;

type Db = Arc<Mutex<HashMap<String, String>>>;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&db);
                handle_connection(stream, db_clone);
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, db: Db) {
    let mut buffer = [0; 4096];
    // let mut value_storage: Value = Default::default();
    match stream.read(&mut buffer) {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Request:\n{}\n", request);

            // Extract first line of HTTP request (e.g., "GET / HTTP/1.1")
            let (method, path_and_query) = parse_request_line(&request).unwrap_or_default();

            // GET request - serve index.html
            if method == "GET" && path_and_query == "/" {
                display_home(&mut stream);
            } else if method == "POST" && path_and_query == "/submit-conversion" {
                process_data(&mut stream, &request, db);
            } else if method == "GET" && path_and_query.starts_with("/conversion") {
                display_result_page(&mut stream, db);
            } else {
                let error_json = json!({
                    "success": false,
                    "error": "Invalid URL"
                });
                send_json_response(&mut stream, 404, &error_json);
            }
        }
        Err(e) => {
            eprintln!("Read error: {}", e);
        }
    }
}

fn display_result_page(stream: &mut TcpStream, db: Db) {
    let db_guard = db.lock().unwrap();

    let result = db_guard
        .get(&"result".to_string())
        .cloned()
        .unwrap_or_default();
    let input = db_guard
        .get(&"input".to_string())
        .cloned()
        .unwrap_or_default();
    let from = db_guard
        .get(&"from".to_string())
        .cloned()
        .unwrap_or_default();
    let to = db_guard.get(&"to".to_string()).cloned().unwrap_or_default();

    println!("result: {}", result);

    let args = format!("Input : {} {},<br>Result: {} {}", input, from, result, to);
    let template = fs::read_to_string("result.html")
        .map(|s| s.replace("RUST", &args))
        .unwrap_or_default();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        template.len(),
        template,
    );

    stream.write_all(response.as_bytes()).ok();
    stream.flush().ok();
}

fn parse_request_line(request: &Cow<'_, str>) -> Option<(String, String)> {
    let mut lines = request.lines();
    let request_line = match lines.next() {
        None => return None,
        Some(line) => line,
    };

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let method = parts[0].to_string();
    let path_and_query = parts[1].to_string();

    Some((method, path_and_query))
}

fn process_data(stream: &mut TcpStream, request: &Cow<'_, str>, db: Db) {
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
            Ok(json_value) => match ConversionData::run(json_value) {
                None => {
                    eprintln!("Unit conversion error!");
                    let error_json = json!({
                        "success": false,
                        "error": "Invalid Unit Conversion"
                    });

                    send_json_response(stream, 400, &error_json);
                }
                Some(data) => {
                    let from = data.get("from").and_then(|v| v.as_str()).unwrap();
                    let to = data.get("to").and_then(|v| v.as_str()).unwrap();
                    let result = data
                        .get("result")
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    let input = data.get("input").map(|s| s.to_string()).unwrap_or_default();

                    db.lock()
                        .unwrap()
                        .insert("from".to_string(), from.to_string());
                    db.lock().unwrap().insert("to".to_string(), to.to_string());
                    db.lock()
                        .unwrap()
                        .insert("input".to_string(), input.to_string());
                    db.lock()
                        .unwrap()
                        .insert("result".to_string(), result.clone());

                    let redirect_path = format!(
                        "/conversion?input={}&result={}&from={}&to={}",
                        input, result, from, to
                    );
                    let response = json!({
                        "success": true,
                        "redirect_url": redirect_path
                    });

                    send_json_response(stream, 200, &response);
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
