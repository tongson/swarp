#![deny(warnings)]
use std::env;

#[tokio::main]
async fn main() {
  let d = env::var("DIR").unwrap_or(".".to_string());
  let p: u16 = env::var("PORT").unwrap_or_else(|_| "3030".to_string()).parse::<u16>().unwrap();
  warp::serve(warp::fs::dir(d))
    .run(([127, 0, 0, 1], p))
    .await;
}
