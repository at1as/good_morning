extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;

use hyper::{client, Client, status, Url};
use hyper::header::{Authorization, Basic, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use std::env;
use std::io::Read;


fn main() {

  let account_sid = "AC67b1bb80dde5bbe63a0673736c5dcf26";
  let from_number = "%2B16473609044";

  let auth_token: String = match env::args().nth(1) {
    Some(auth_token) => auth_token.to_owned(),
    None => {
      println!("Usage : cargo run <auth_token> <to_number>");
      return;
    }
  };

  let to_number: String = match env::args().nth(2) {
    Some(to_number) => str::replace(&to_number, "+", "%2B").to_owned(),
    None => {
      println!("Usage: cargo run <auth_token> <to_number>");
      return;
    }
  };
  
  let weather_report = str::replace(&get_weather(), "\"", "");

  let url  = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_sid).to_owned();
  let data = format!("From={}&To={}&Body={}", from_number, to_number, weather_report);
  
  let mut res = get_client()
                .post(&url)
                .body(&data)
                .header(Authorization(Basic{
                  username: account_sid.to_string(),
                  password: Some(auth_token.to_owned())
                }))
                .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
                .send()
                .unwrap();
  
  let mut s = String::new();
  res.read_to_string(&mut s).unwrap();

  println!("{}", s);
}


fn get_client() -> Client {
  let ssl = NativeTlsClient::new().unwrap();
  let connector = HttpsConnector::new(ssl);

  Client::with_connector(connector)
}


fn get_weather() -> String {
  let city = "Montreal";
  let query_url = format!("http://query.yahooapis.com/v1/public/yql?q=select%20*%20from%20weather.forecast%20where%20woeid%20in%20(select%20woeid%20from%20geo.places(1)%20where%20text%3D%22{}%22)%20and%20u%3D'c'&format=json&env=store%3A%2F%2Fdatatables.org%2Falltableswithkeys", &city);

  let client  = Client::new();
  let mut res = client
                .get(&query_url)
                .send()
                .unwrap_or_else(|e| {
                  panic!("Error : {}", e);
                });

  let mut s = String::new();
  res.read_to_string(&mut s).unwrap();

  let json: Value = serde_json::from_str(&s).unwrap_or_else(|e| {
    panic!("Failed to parse json : {}", e)
  });


  let ref sunset       = json["query"]["results"]["channel"]["astronomy"]["sunset"];
  let ref current_temp = json["query"]["results"]["channel"]["item"]["condition"]["temp"];
  let ref current_text = json["query"]["results"]["channel"]["item"]["condition"]["text"];
  let ref daily_high   = json["query"]["results"]["channel"]["item"]["forecast"][0]["high"];
  let ref daily_low    = json["query"]["results"]["channel"]["item"]["forecast"][0]["low"];
  let ref daily_text   = json["query"]["results"]["channel"]["item"]["forecast"][0]["text"];

  let text = format!("Currently {} C and {}. Today {} with a high of {} and low of {}. Today's sunset is at {}", current_temp, current_text, daily_text, daily_high, daily_low, sunset);
  println!("Preparing weather report : {}", text);
  
  text
}

