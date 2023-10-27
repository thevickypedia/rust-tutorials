extern crate chrono;

mod args;
mod loops;
mod entry;
mod datetime;

fn main() {
    datetime::datetime();
    entry::entrypoint();
}
