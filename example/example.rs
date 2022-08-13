use carrot_utils::logger::Logger;
use log::LevelFilter;

fn main() {
    let logger = Logger::new(
        "./data/{TIME_SEC}",
        "log_{TIME_SEC}.txt",
        LevelFilter::Debug,
        LevelFilter::Debug,
    );
}
