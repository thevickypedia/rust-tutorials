use std::time;
use chrono;

pub fn datetime() {
    // represents the Unix epoch (January 1, 1970) as a SystemTime object
    let epoch = time::UNIX_EPOCH;
    println!("{:?}", epoch);
    println!("{}", chrono::offset::Local::now());
}
