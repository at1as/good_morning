
pub fn print_usage() {
  println!(r#"
    Usage:
      cargo run <twilio_auth_token> <to_number>

    Example:
      cargo run 747d2bfff9e6c6e0b7b3c5b3866597db +15556667788

    Example Response:
      Text Message to +15556667788 reads "Rise and shine! Currently 10 C and cloudy. Today scattered showers ..."

    Set other variables in src/message_conf.json:
      <message_prepend_text> is an option field, all others are required.
      <city_location> name of city for weather report
      <stocks> list of stocks for stock report
  "#);
}

