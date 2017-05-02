extern crate serde_json;

use serde_json::Value;
use hyper::Client;
use std::io::Read;


pub fn get_weather(city: String) -> String {
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

  let text = format!("Currently {} C and {}. Today {} with a high of {} and low of {}. Today's sunset is at {}.", current_temp, current_text, daily_text, daily_high, daily_low, sunset);

  str::replace(&text, "\"", "")
}

