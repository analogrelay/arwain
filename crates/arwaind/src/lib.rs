use std::{path::Path, os::unix::prelude::PermissionsExt};

use anyhow::Context;
use arwain_proto::meta::meta_service_server::MetaServiceServer;
use clap::Parser;
use tokio::{net::UnixListener, select};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tracing::{trace, info};

mod logging;
mod meta;

#[derive(Parser)]
struct ArwaindOptions {
    /// Server socket path, defaults to /var/run/arwain/arwain.sock
    #[arg(short, long, default_value = "/var/run/arwain/arwain.sock")]
    socket: String,

    /// Log verbosely
    #[arg(short, long)]
    verbose: bool
}

pub async fn server() -> i32 {
    let options = ArwaindOptions::parse();

    logging::start(options.verbose);
    tracing::info!("Arwaind server starting...");
    tracing::debug!("Verbose tracing enabled");

    let result = run(&options.socket).await;

    match result {
        Ok(_) => 0,
        Err(e) => {
            tracing::error!("{:?}", e);
            1
        },
    }
}

async fn run(socket_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Prep by cleaning up the old socket (ignoring errors), and creating the parent directory
    tokio::fs::remove_file(socket_path).await.ok();
    let socket_dir = Path::new(socket_path)
        .parent()
        .ok_or("Invalid socket path")?;

    tokio::fs::create_dir_all(socket_dir).await.with_context(|| {
        format!(
            "Unable to create socket directory: {}",
            socket_path
        )
    })?;
    trace!("Socket directory created");

    // Bind the socket
    let socket = UnixListener::bind(socket_path)?;
    let stream = UnixListenerStream::new(socket);

    // Start the server 🚀
    let server = tokio::spawn(async {
        Server::builder()
            .add_service(MetaServiceServer::new(meta::MetaService::default()))
            .serve_with_incoming_shutdown(stream, async {
                // When this future completes, the server will shutdown.
                let mut sigterm = tokio::signal::unix::signal(
                    tokio::signal::unix::SignalKind::terminate(),
                ).expect("Failed to register SIGTERM handler");
                let mut sigint = tokio::signal::unix::signal(
                    tokio::signal::unix::SignalKind::interrupt(),
                ).expect("Failed to register SIGINT handler");

                select! {
                    _ = sigterm.recv() => trace!("SIGTERM received"),
                    _ = sigint.recv() => trace!("SIGINT received"),
                }
                
                info!("Shutdown requested...");
            })
            .await
    });

    // Allow non-root users to connect to the socket
    tokio::fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o766)).await?;
    info!("Socket created, listening for connections: {}", socket_path);

    // Wait for the server to shut down
    server.await??;
    info!("Server shut down");
    
    Ok(())
}