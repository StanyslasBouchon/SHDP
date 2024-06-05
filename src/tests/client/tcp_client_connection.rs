use crate::{
    client::prelude::tcp::connect,
    protocol::prelude::common::utils::stop,
    server::prelude::tcp::listen,
};
use tokio::time::timeout;
use std::time::Duration;

#[tokio::test]
async fn test() {
    let timeout_duration = Duration::from_secs(2);
    let server_future = start_server();
    let client_future = start_client();

    let server_result = timeout(timeout_duration, server_future).await;
    let client_result = timeout(timeout_duration, client_future).await;

    match (server_result, client_result) {
        (Ok(sr), Ok(cr)) => {
            assert!(sr.is_some());
            assert!(cr.is_some());
        }
        _ => {
            println!("Timeout reached, stopping the listener");
            stop(String::from("127.0.0.1"), String::from("9999")).await;
            assert!(false);
        }
    }
}

async fn start_server() -> Option<()> {
    match listen(String::from("9999")).await {
        Ok(_) => {
            println!("Listen succeeded");
            Some(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            stop(String::from("127.0.0.1"), String::from("9999")).await;
            assert!(false);
            None
        }
    }
}

async fn start_client() -> Option<()> {
    match connect("127.0.0.1".to_string(), "9997".to_string()).await {
        Ok(_) => {
            println!("Connect succeeded");
            Some(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            stop(String::from("127.0.0.1"), String::from("9997")).await;
            assert!(false);
            None
        }
    }
}
