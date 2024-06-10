use async_std::future::timeout;
use async_std::task;
use std::time::Duration;

use crate::protocol::prelude::common::utils::stop;
use crate::server::prelude::ws::listen;

#[test]
fn test() {
    let test_result = task::block_on(async {
        let timeout_duration = Duration::from_secs(2);
        let listen_future = start();

        match timeout(timeout_duration, listen_future).await {
            Ok(result) => result,
            Err(_) => {
                println!("WSS: Timeout reached, stopping the listener");
                stop(String::from("127.0.0.1"), String::from("9998")).await;
                None
            }
        }
    });

    assert!(test_result.is_none())
}

async fn start() -> Option<()> {
    match listen(String::from("9998")).await {
        Ok(_) => {
            println!("WSS: Listen succeeded");
            Some(())
        }
        Err(e) => {
            println!("WSS: Error: {:?}", e);
            stop(String::from("127.0.0.1"), String::from("9998")).await;
            assert!(false);
            None
        }
    }
}
