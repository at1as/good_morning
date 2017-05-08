extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
extern crate image;
extern crate imageproc;
extern crate multipart;
extern crate rusttype;
extern crate serde_json;

use chrono::prelude::*;
use hyper::{client, Client, status, Url};
use hyper::header::{Authorization, Basic, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use multipart::client::Multipart;
use serde_json::Value;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

/* Local Imports */
use cloudinary_api::upload_image_multipart;
use create_image::text_to_image;
use stocks::get_stocks;
use utils::print_usage;
use weather::get_weather;
mod cloudinary_api;
mod create_image;
mod stocks;
mod utils;
mod weather;


fn main() {

  let twilio_conf = "src/conf/twilio_conf.json";
  let message_conf = "src/conf/message_conf.json";

  let account_sid = get_config_variable("account_sid".to_owned(), twilio_conf.to_owned());
  let from_number = get_config_variable("from_number".to_owned(), twilio_conf.to_owned());
  
  let message_prepend = get_config_variable("message_prepend_text".to_owned(), message_conf.to_owned());
  let city = get_config_variable("city_location".to_owned(), message_conf.to_owned());
  let stocks = get_config_variable("stocks".to_owned(), message_conf.to_owned());

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

  let weather_report = get_weather(city);
  let stock_report = get_stocks(stocks);
  
  let url  = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_sid).to_owned();

  // Create and send SMS
  if (get_config_variable("send_sms".to_owned(), message_conf.to_owned()) == "true") {
    let body = [message_prepend.clone(), weather_report.clone(), stock_report.clone()].join(&" ");
    let data = format!("From={}&To={}&Body={}", from_number, to_number, body);

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

    println!("Sent SMS. Server responded with : {}", s);
  }


  // Create and send MMS
  if (get_config_variable("send_mms".to_owned(), message_conf.to_owned()) == "true") {
    
    // Cloudinary Image Upload Credentials
    let cloudinary_conf = "src/conf/cloudinary_conf.json";
    let cloudinary_upload_preset = get_config_variable("upload_preset".to_owned(), cloudinary_conf.to_owned());

    
    // Create Timestamp for Message
    let time = Local::now();
    let hour = time.hour();
    let min  = if time.minute() < 10 { 
                  format!("0{}", time.minute())
                } else {
                  format!("{}", time.minute())
                };

    let time_stamp = format!("{}:{} – ", hour, min);
    
    // Create and Upload Image
    let message_prepend_with_timestamp = format!("{}{}", time_stamp, message_prepend);
    let text_bodies = vec!("prepend".to_owned(), message_prepend_with_timestamp,
                           "Weather Report:".to_string(), weather_report,
                           "Stock Report:".to_owned(), stock_report);

    text_to_image(text_bodies, "./mms.jpeg".to_owned());
    let access_url = upload_image_multipart("mms.jpeg".to_owned(), cloudinary_upload_preset.to_owned());
    
    let body = access_url.clone();
    let data = format!("From={}&To={}&MediaUrl={}", from_number, to_number, body);

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

    println!("Sent MMS. Server responded with : {}", s);
  }
}


fn get_config_variable(key: String, filename: String) -> String {
  
  let path = Path::new(&filename);
  let mut file = File::open(&path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents);

  let json: Value = serde_json::from_str(&contents).unwrap();

  match json[key] {
    Value::String(ref v) => {
      let stripped_val: String = format!("{}", v);
      str::replace(&stripped_val, "\"", "")
    },
    Value::Array(ref v) => {
      let mut stocks = Vec::new();
      
      for x in v {
        let str_val = serde_json::Value::as_str(x).unwrap();
        stocks.push(str_val);
      }

      str::replace(&stocks.join(&"+"), "\"", "")
    },
    Value::Bool(ref v) => {
      v.to_string()
    },
    _ => "".to_owned()
  }
}


fn get_client() -> Client {
  let ssl = NativeTlsClient::new().unwrap();
  let connector = HttpsConnector::new(ssl);

  Client::with_connector(connector)
}

