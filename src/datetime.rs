use chrono;

pub fn datetime() {
    let datetime_obj = chrono::offset::Local::now();
    println!("Date: {}", datetime_obj.date_naive());
    println!("Time: {}", datetime_obj.time());
    println!("UTF Offset: {:?}", datetime_obj.offset());  // Use format specifier
}
