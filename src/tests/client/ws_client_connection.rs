use std::time::Duration;

use futures::FutureExt;
use smol::Timer;

use crate::{
    client::prelude::ws::connect, protocol::prelude::common::utils::stop,
    server::prelude::ws::listen,
};

#[test]
fn test_server() {
    let result = smol::block_on(async {
        let timeout_duration = Duration::from_secs(2);
        let listen_future = start_server();

        futures::select! {
            result = listen_future.fuse() => result,
            _ = Timer::after(timeout_duration).fuse() => {
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

#[test]
fn test_client() {
    let result = smol::block_on(async {
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
