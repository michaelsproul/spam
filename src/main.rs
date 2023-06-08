use futures::future::join_all;
use rand::Rng;
use reqwest;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    loop {
        let mut handles = vec![];
        for _ in 0..1000 {
            let handle = tokio::spawn(async move {
                let url =
                    format!("http://127.0.0.1:5052/eth/v1/beacon/states/finalized/validators");
                reqwest::get(url).await.unwrap();
            });
            handles.push(handle);
            sleep(Duration::from_millis(50)).await;
        }
        join_all(handles).await;
    }
}
