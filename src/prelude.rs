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
