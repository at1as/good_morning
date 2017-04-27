extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use hyper::{client, Client, status, Url};
use hyper::header::{Authorization, Basic, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use std::env;
use std::io::Read;
use std::fs::File;


fn main() {

  let account_sid = get_config_variable("account_sid".to_owned(), "src/twilio_conf.json".to_owned());
  let from_number = get_config_variable("from_number".to_owned(), "src/twilio_conf.json".to_owned());

  let auth_token: String = match env::args().nth(1) {
    Some(auth_token) => auth_token.to_owned(),
    None => {
      print_usage();
      return;
    }
  };

  let to_number: String = match env::args().nth(2) {
    Some(to_number) => str::replace(&to_number, "+", "%2B").to_owned(),
    None => {
      print_usage();
      return;
    }
  };

  let city: String = match env::args().nth(3) {
    Some(city) => city.to_owned(),
    None => {
      print_usage();
      return;
    }
  };

  let stock: String = "TWLO".to_owned();

  let message_prepend: String = match env::args().nth(4) {
    Some(message_prepend) => format!("{} ", message_prepend).to_owned(),
    None => "".to_owned()
  };

  
  let weather_report = get_weather(city);
  let stock_report = get_stock(stock);

  let url  = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_sid).to_owned();
  let data = format!("From={}&To={}&Body={}{}{}", from_number, to_number, message_prepend, weather_report, stock_report);
  
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

  println!("Server responded with : {}", s);
}


fn get_config_variable(key: String, filename: String) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents);

  let json: Value = serde_json::from_str(&contents).unwrap();
  let ref value = json[key];

  let stripped_val: String = format!("{}", value);

  str::replace(&stripped_val, "\"", "")
}


fn get_client() -> Client {
  let ssl = NativeTlsClient::new().unwrap();
  let connector = HttpsConnector::new(ssl);

  Client::with_connector(connector)
}


fn get_weather(city: String) -> String {
  let query_url = format!("http://query.yahooapis.com/v1/public/yql?q=select%20*%20from%20weather.forecast%20where%20woeid%20in%20(select%20woeid%20from%20geo.places(1)%20where%20text%3D%22{}%22)%20and%20u%3D'c'&format=json&env=store%3A%2F%2Fdatatables.org%2Falltableswithkeys", &city).to_owned();

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

  format_weather(json)
}


fn format_weather(json: Value) -> String {
  let ref sunset       = json["query"]["results"]["channel"]["astronomy"]["sunset"];
  let ref current_temp = json["query"]["results"]["channel"]["item"]["condition"]["temp"];
  let ref current_text = json["query"]["results"]["channel"]["item"]["condition"]["text"];
  let ref daily_high   = json["query"]["results"]["channel"]["item"]["forecast"][0]["high"];
  let ref daily_low    = json["query"]["results"]["channel"]["item"]["forecast"][0]["low"];
  let ref daily_text   = json["query"]["results"]["channel"]["item"]["forecast"][0]["text"];

  let text = format!("Currently {} C and {}. Today {} with a high of {} and low of {}. Today's sunset is at {}", current_temp, current_text, daily_text, daily_high, daily_low, sunset);

  str::replace(&text, "\"", "")
}


fn get_stock(ticker: String) -> String {
  let query_url = format!("http://finance.yahoo.com/d/quotes.csv?s={}&f=sm", ticker);
  
  let client  = Client::new();
  let mut res = client
                .get(&query_url)
                .send()
                .unwrap_or_else(|e| {
                  panic!("Error retrieving stock information : {}", e);
                });

  let mut s = String::new();
  res.read_to_string(&mut s).unwrap();

  /*
    Response body format:
      "TWLO","30.79 - 32.13"
      "GOOG","862.81 - 875.00"
  */

  let lines: Vec<&str> = s.split(|c| c == '\n').collect();
  
  let mut stock_text = "".to_owned();

  for stock_ticker in lines {
    let ticker_text: Vec<&str> = stock_ticker.split(|c| c == ',').collect();

    stock_text.push_str(&format!("{}", ticker_text.join(" range ")));
  }

  stock_text
}


fn print_usage() {
  println!(r#"
    Usage:
      cargo run <twilio_auth_token> <to_number> '<city_name>' '<message_prepended_text>'

    Example:
      cargo run 747d2bfff9e6c6e0b7b3c5b3866597db +15556667788 'San Francisco' 'Rise and shine!'

    Example Response:
      Text Message to +15556667788 reads "Rise and shine! Currently 10 C and cloudy. Today scattered showers ..."

    Note:
      <message_prepend_text> is an option field, all others are required.
  "#);
}

