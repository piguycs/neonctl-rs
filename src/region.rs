use std::{
    sync::Arc,
    thread::{self, JoinHandle},
    time,
};

use crate::prelude::*;

pub fn neon_regions() -> Vec<JoinHandle<(String, u128)>> {
    let mut handles = vec![];

    for region in NEON_REGIONS {
        let region = Arc::new(region);
        let handle = thread::spawn(move || {
            let endpoint = f!("http://dynamodb.{region}.amazonaws.com");
            let start = time::Instant::now();
            ureq::get(&endpoint).call().unwrap();

            let time = start.elapsed();

            (f!("aws-{region}"), time.as_millis())
        });

        handles.push(handle);
    }

    handles
}
