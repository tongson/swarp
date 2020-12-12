#![deny(warnings)]
use std::env;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
  let bind = env::var("BIND").unwrap_or("127.0.0.1".to_string());
  let port = env::var("PORT").unwrap_or("3030".to_string()).parse::<u16>().unwrap();
  let cwd = env::var("CWD").unwrap_or(".".to_string());
  warp::serve(warp::fs::dir(cwd))
    .run((bind.parse::<IpAddr>().unwrap(), port))
    .await;
}
