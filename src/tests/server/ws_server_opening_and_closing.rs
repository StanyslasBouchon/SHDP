use std::time::Duration;

use futures::FutureExt;
use smol::Timer;

use crate::protocol::prelude::common::utils::stop;
use crate::server::prelude::ws::listen;

#[test]
fn test() {
    let test_result = smol::block_on(async {
        let timeout_duration = Duration::from_secs(2);
        let listen_future = start();

        futures::select! {
            result = listen_future.fuse() => result,
            _ = Timer::after(timeout_duration).fuse() => {
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
