use rall::{debug, error, fatal, info, trace, warn};

fn main() {
    trace!("My Best Friend Hazel {}", ":D");
    debug!("My Best Friend Hazel {}", ":D");
    info!("My Best Friend Hazel {}{}", ":", ")");
    warn!("My Best Friend Hazel {}", ":)");
    error!("My Best Friend Hazel {}", ":P");
    fatal!("My Best Friend Hazel {}", ":D");

    let very_important_value = String::from(";)");
    debug!("Look at this: {}", very_important_value);
}