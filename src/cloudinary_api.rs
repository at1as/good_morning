extern crate serde_json;

use serde_json::Value;
use hyper::{Url, Result};
use hyper::method::Method;
use hyper::client::Request;
use hyper::net::Streaming;
use multipart::client::Multipart;
use std::io::Read;
use std::fs::File;
use std::path::Path;


/* Cloudinary API is used to temporarily store MMS image. Cloudinary returns a public URL
   with the JPEG that is then passed to Twilio to as a parameter in the MMS request */


pub fn upload_image_multipart(image_name: &str, upload_preset: &str) -> String {

  /* Build Request */
  let upload_url = Url::parse("http://api.cloudinary.com/v1_1/at1as/image/upload").unwrap();
  let req = Request::new(Method::Post, upload_url).unwrap();
  let mut multipart = Multipart::from_request(req).unwrap();

  write_body(&mut multipart, image_name, upload_preset).unwrap();

  let mut response = multipart.send().unwrap();
  
  /* Format Response */
  let mut s = String::new();
  response.read_to_string(&mut s).unwrap();

  let json: Value = serde_json::from_str(&s).unwrap_or_else(|e| {
    panic!("Failed to parse cloudinary json for image upload : {}", e)
  });

  extract_image_url(json)
}


/*
   Test URL as cURL with :
      curl -XPOST 'http://api.cloudinary.com/v1_1/at1as/image/upload' -F "file=@mms.jpeg" -F "upload_preset=omz6uupt" -v
   
   Note:
      Upload_preset flag allows uploading without authentication

   See:
      http://cloudinary.com/documentation/upload_images#unsigned_upload
      http://cloudinary.com/documentation/upload_images#upload_presets  */

fn write_body(multi: &mut Multipart<Request<Streaming>>, image_name: &str, upload_preset: &str) -> Result<()> {
  let path = Path::new(&image_name);
  let mut binary = File::open(&path).unwrap();

  multi.write_text("upload_preset", upload_preset)?;
  multi.write_file("file", "mms.jpeg")?;
  multi.write_stream("mms", &mut binary, Some("mms.jpeg"), None)
        .and(Ok(()))
}

fn extract_image_url(json: Value) -> String {
  let ref sunset = json["secure_url"];
  let image_url = format!("{}", sunset);

  str::replace(&image_url, "\"", "")
}

