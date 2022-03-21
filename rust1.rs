use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}


use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}

use error_chain::error_chain;
use serde::Deserialize;
use serde_json::json;
use std::env;
use reqwest::Client;

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

#[tokio::main]
async fn main() ->  Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    let request_url = "https://api.github.com/gists";
    let response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send().await?;

    let gist: Gist = response.json().await?;
    println!("Created {:?}", gist);

    let request_url = format!("{}/{}",request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send().await?;

    println!("Gist {} deleted! Status code: {}",gist.id, response.status());
    Ok(())
}

use reqwest;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}

use reqwest;
use reqwest::header;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut h = header::HeaderMap::new();
    h.insert("Accept", header::HeaderValue::from_static("application/json"));
    
    let client = reqwest::Client::builder()
        .default_headers(h)
        .build()?;

    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}


use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let api_key = "YOUR API KEY".to_string();
    let args: Vec<String> = env::args().collect();
    let mut symbol: String = "AAPL".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a company symbol, it has defaulted to AAPL.");
    } else {
        symbol = args[1].clone();
    }

    let res = CompanyQuote::get(&symbol, &api_key).await?;
    println!("{}'s current stock price: {}", symbol, res.c);

    Ok(())
}

use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}


use std::io::Read;
use curl::easy::Easy;

fn main() {
    let mut data = "this is the body".as_bytes();

    let mut easy = Easy::new();
    easy.url("http://www.example.com/upload").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
}

use curl::easy::{Easy, List};

fn main() {
    let mut easy = Easy::new();
    easy.url("http://www.example.com").unwrap();

    let mut list = List::new();
    list.append("Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==").unwrap();
    easy.http_headers(list).unwrap();
    easy.perform().unwrap();
}

use ureq::{Agent, AgentBuilder};
  use std::time::Duration;

  let agent: Agent = ureq::AgentBuilder::new()
      .timeout_read(Duration::from_secs(5))
      .timeout_write(Duration::from_secs(5))
      .build();
  let body: String = agent.get("http://example.com/page")
      .call()?
      .into_string()?;

  // Reuses the connection from previous request.
  let response: String = agent.put("http://example.com/upload")
      .set("Authorization", "example-token")
      .call()?
      .into_string()?;
      
      use isahc::prelude::*;

fn main() -> Result<(), isahc::Error> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.
    let mut response = isahc::get("https://example.org")?;

    // Print some basic info about the response to standard output.
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
    print!("{}", response.text()?);

    Ok(())
}

let obj = json!({
    "hello": "world",
});

let resp = attohttpc::post("https://my-api.org/do/something")
    .header("X-My-Header", "foo")   // set a header for the request
    .param("qux", "baz")            // set a query parameter
    .json(&obj)?                    // set the request body (json feature required)
    .send()?;                       // send the request

// Check if the status is a 2XX code.
if resp.is_success() {
    // Consume the response body as text and print it.
    println!("{}", resp.text()?);
}
