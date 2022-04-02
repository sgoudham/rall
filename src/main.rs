use rall::{Level, SimpleLogger};

fn main() {
    let mut logger = SimpleLogger::default();
    logger.log(Level::TRACE, "My Best Friend Hazel :D");
    logger.log(Level::DEBUG, "My Best Friend Hazel :D");
    logger.log(Level::INFO, "My Best Friend Hazel :D");
    logger.log(Level::WARN, "My Best Friend Hazel :D");
    logger.log(Level::ERROR, "My Best Friend Hazel :D");
    logger.log(Level::FATAL, "My Best Friend Hazel :D");
}