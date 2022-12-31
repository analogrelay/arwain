pub fn start(verbose: bool) {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(format!("arwaind={level}"))
        .try_init()
        .unwrap();
}