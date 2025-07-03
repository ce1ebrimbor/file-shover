use clap::Parser;
use env_logger;
use log::{debug, info, warn, error};
use std::io::{Cursor, ErrorKind, Read};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;

mod files;
mod message;
mod data;

use files::FileTree;
use message::{HttpStatus, Request, Response};
use data::get_mime_type;

/// A simple static file server
#[derive(Parser, Debug)]
#[command(name = "file-shover")]
#[command(about = "A static file server written in Rust")]
#[command(version = "1.0")]
struct Args {
    /// Root directory to serve files from
    #[arg(short, long, value_name = "PATH")]
    root: PathBuf,

    /// Port to listen on
    #[arg(short, long, default_value = "7878")]
    port: u16,
}

// parse request
fn handle_client(mut stream: TcpStream, file_tree: &FileTree) {
    debug!("New client connection");
    
    // Parse the request and handle parsing errors
    let req = match Request::from_bytes(&stream) {
        Ok(request) => request,
        Err(e) => {
            debug!("Failed to parse request: {}", e);
            let mut response = Response::new()
                .status(HttpStatus::BadRequest)
                .content_type("text/html")
                .body(Box::new(Cursor::new("<h1>400 Bad Request</h1>".as_bytes())));
            
            if let Err(write_err) = response.write(&mut stream) {
                debug!("Failed to write error response: {}", write_err);
            }

            if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
                debug!("Failed to shutdown stream: {}", e);
            }
            return;
        }
    };
    
    info!("Request: {} {}", req.method, req.path);

    let mut response = match file_tree.get_reader(&req.path) {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                info!("File not found: {}", req.path);
                Response::new()
                    .status(HttpStatus::NotFound)
                    .content_type("text/html")
                    .body(Box::new(Cursor::new("<h1>404 Not Found</h1>".as_bytes())))
            } else {
                info!("Server error for {}: {}", req.path, e);
                Response::new()
                    .status(HttpStatus::InternalServerError)
                    .content_type("text/html")
                    .body(Box::new(Cursor::new("<h1>500 Internal Server Error</h1>".as_bytes())))
            }
        }
        Ok(mut reader) => {
            let mut body = Vec::new();
            match reader.read_to_end(&mut body) {
                Ok(_) => {
                    info!("Successfully served: {}", req.path);
                    let mime_type = get_mime_type(&req.path);
                    Response::new()
                        .status(HttpStatus::Ok)
                        .content_type(mime_type.as_str())
                        .body(Box::new(Cursor::new(body)))
                }
                Err(e) => {
                    debug!("Error reading file {}: {}", req.path, e);
                    Response::new()
                        .status(HttpStatus::InternalServerError)
                        .content_type("text/html")
                        .body(Box::new(Cursor::new("<h1>500 Internal Server Error</h1>".as_bytes())))
                }
            }
        }
    };

    if let Err(e) = response.write(&mut stream) {
        debug!("Failed to write response: {}", e);
    }

    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
        debug!("Failed to shutdown stream: {}", e);
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let bind_address = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&bind_address)?;
    let file_tree = FileTree::new(args.root.clone());

    info!("🚀 File Shover server starting...");
    info!("📁 Serving files from: {}", args.root.display());
    info!("🌐 Listening on: http://{}", bind_address);
    info!("Press Ctrl+C to stop the server");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &file_tree);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
