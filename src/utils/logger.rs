use log::debug;
use std::fs::File;
use std::io::Write;

pub fn init_logger() {
    use std::env;
    env::set_var("RUST_LOG", "debug");

    let log_file = File::create("debug.log").expect("Failed to create log file");

    env_logger::Builder::new()
        .format(|buf, record| {
            let level = format!("{:5}", record.level()); // Pad to align levels
            writeln!(
                buf,
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .filter(None, log::LevelFilter::Debug)
        .init();

    debug!("Logger initialized");
}
