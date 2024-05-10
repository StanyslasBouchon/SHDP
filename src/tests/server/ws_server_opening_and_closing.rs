use async_std::future::timeout;
use async_std::task;
use std::time::Duration;

use crate::server::prelude::{ stop, ws::listen };

#[test]
fn test() {
    let test_result = task::block_on(async {
        let timeout_duration = Duration::from_secs(2);
        let listen_future = a();

        match timeout(timeout_duration, listen_future).await {
            Ok(result) => result,
            Err(_) => {
                println!("Timeout reached, stopping the listener");
                stop("9998".to_string());
                None
            }
        }
    });

    assert!(test_result.is_none())
}

async fn a() -> Option<()> {
    match listen(String::from("9998")).await {
        Ok(_) => {
            println!("Listen succeeded");
            Some(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    }
}
