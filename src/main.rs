use std::net::TcpListener;

use financial_monitor::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Tightly tied to this IP need FIX"
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address)?;
    println!("Server running on {}", address);
    run(listener)?.await
}
