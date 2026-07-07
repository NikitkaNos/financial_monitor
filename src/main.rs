use financial_monitor::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
