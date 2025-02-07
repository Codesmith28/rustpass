use log::debug;
use std::fs::File;
use std::io::Write;

pub fn init_logger() {
    use std::env;
    env::set_var("RUST_LOG", "debug");

    // Create debug.log with explicit error handling
    let log_file = File::create("debug.log").expect("Failed to create log file");

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .filter(None, log::LevelFilter::Debug) // Explicitly set debug level
        .init();

    debug!("Logger initialized"); // Test log message
}
