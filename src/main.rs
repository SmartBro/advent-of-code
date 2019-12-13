extern crate hyper;
extern crate hyper_tls;

use hyper::{Client, Body, Method, Request};
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://adventofcode.com/2019/day/1/input")
        .header("cookie", "session=<--YOUR_SESSION-->")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

//    let uri = "https://adventofcode.com/2019/day/1/input".parse()?;
    let resp = client.request(req).await?;

    println!("Response: {:#?}", resp.body());

    Ok(())
}