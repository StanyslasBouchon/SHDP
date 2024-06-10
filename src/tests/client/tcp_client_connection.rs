// use crate::{
//     client::prelude::tcp::connect,
//     protocol::prelude::common::{error::Error, utils::stop},
//     server::prelude::tcp::listen,
// };
// use async_std::task;

// #[tokio::test]
// async fn test_server() {
//     let result = task::block_on(async {
//         match listen(String::from("9999")).await {
//             Ok(_) => {
//                 println!("Listen succeeded");
//                 Some(())
//             }
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 stop(String::from("127.0.0.1"), String::from("9999")).await;
//                 assert!(false);
//                 None
//             }
//         }
//     });

//     assert!(result.is_none())
// }

// #[tokio::test]
// async fn test_client() {
//     let result = task::block_on(async {
//         match connect(String::from("127.0.0.1"), String::from("9999")).await {
//             Ok(_) => {
//                 println!("Listen succeeded");
//                 Some(())
//             }
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 stop(String::from("127.0.0.1"), String::from("9999")).await;
//                 assert!(false);
//                 None
//             }
//         }
//     });

//     assert!(result.is_none())
// }
