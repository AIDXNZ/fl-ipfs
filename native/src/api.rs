use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub enum Platform {
    Unknown,
    Android,
    Ios,
    Windows,
    Unix,
    MacIntel,
    MacApple,
    Wasm,
}

pub fn platform() -> Platform {
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::Ios
    } else if cfg!(target_arch = "aarch64-apple-darwin") {
        Platform::MacApple
    } else if cfg!(target_os = "macos") {
        Platform::MacIntel
    } else if cfg!(target_family = "wasm") {
        Platform::Wasm
    } else if cfg!(unix) {
        Platform::Unix
    } else {
        Platform::Unknown
    }
}

pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}

pub fn handle_connection(mut stream: TcpStream) {
    if let Ok(val) = stream.read(&mut [0; 128]) {
        println!("{:?}", val)
    } else {
        println!("Connection Failed")
    }
}

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
    Ok(())
}