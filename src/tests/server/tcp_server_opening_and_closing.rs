use std::time::Duration;

use tokio::time::timeout;

use crate::{protocol::prelude::common::utils::stop, server::prelude::tcp::listen};

#[tokio::test]
async fn test() {
    let test_result = {
        let timeout_duration = Duration::from_secs(2);
        let listen_future = start();

        match timeout(timeout_duration, listen_future).await {
            Ok(result) => result,
            Err(_) => {
                println!("TCPS: Timeout reached, stopping the listener");
                stop(String::from("127.0.0.1"), String::from("9999")).await;
                None
            }
        }
    };

    assert!(test_result.is_none())
}

async fn start() -> Option<()> {
    match listen(String::from("9999")).await {
        Ok(_) => {
            println!("TCPS: Listen succeeded");
            Some(())
        }
        Err(e) => {
            println!("TCPS: Error: {:?}", e);
            stop(String::from("127.0.0.1"), String::from("9999")).await;
            assert!(false);
            None
        }
    }
}
