use hyper::Client;
use std::io::Read;


pub fn get_stocks(ticker: &str) -> String {
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

    stock_text.push_str(&format!("{} ", str::replace(&ticker_text.join(" : "), "\"", "")));
  }

  stock_text
}
