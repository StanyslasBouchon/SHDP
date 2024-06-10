use std::time::Duration;

use crate::{
    client::prelude::ws::connect, protocol::prelude::common::utils::stop,
    server::prelude::ws::listen,
};
use async_std::future::timeout;
use async_std::task;

#[tokio::test]
async fn test_server() {
    let result = task::block_on(async {
        let timeout_duration = Duration::from_secs(2);
        let listen_future = start_server();

        match timeout(timeout_duration, listen_future).await {
            Ok(result) => result,
            Err(_) => {
                println!("WSCS: Timeout reached, stopping the listener");
                stop(String::from("127.0.0.1"), String::from("9997")).await;
                None
            }
        }
    });

    assert!(result.is_none())
}

async fn start_server() -> Option<()> {
    match listen(String::from("9997")).await {
        Ok(_) => {
            println!("WSCS: Listen succeeded");
            Some(())
        }
        Err(e) => {
            println!("WSCS: Error: {:?}", e);
            stop(String::from("127.0.0.1"), String::from("9997")).await;
            assert!(false);
            None
        }
    }
}

#[tokio::test]
async fn test_client() {
    let result = task::block_on(async {
        match connect(String::from("127.0.0.1"), String::from("9997")).await {
            Ok(_) => {
                println!("WSCC: Listen succeeded");
                Some(())
            }
            Err(e) => {
                println!("WSCC: Error: {:?}", e);
                stop(String::from("127.0.0.1"), String::from("9997")).await;
                assert!(false);
                None
            }
        }
    });

    assert!(result.is_some())
}
