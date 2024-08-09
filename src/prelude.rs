pub use crate::{api::Api, table::print_table};
pub use std::format as f;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const NEON_REGIONS: [&str; 6] = [
    "us-east-1",
    "us-east-2",
    "us-west-2",
    "eu-central-1",
    "ap-southeast-1",
    "ap-southeast-2",
];

pub fn print_bold(text: &str) {
    if let Some(mut t) = term::stdout() {
        let _ = t.attr(term::Attr::Bold);
        let _ = writeln!(t, "{text}");
        let _ = t.reset();
    } else {
        println!("{text}");
    }
}
