#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exit_code = arwaind::server().await;
    std::process::exit(exit_code);
}