use env_logger::Builder;

fn main() {
    let mut builder = Builder::new();
    builder.filter_level(log::LevelFilter::Info);
    
    if cfg!(debug_assertions) {
        builder.filter_module("arwain", log::LevelFilter::Trace);
    }
    builder.init();

    log::info!("Starting arwaind...");
    log::trace!("Verbose tracing is enabled");
}
