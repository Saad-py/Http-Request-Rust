// Warnings are annoying rn
#![allow(warnings)]
// Imports
extern crate hyper;


use hyper_tls::HttpsConnector;
use hyper::{Body, Response};
use hyper::Client;
use url::{Url};
use tokio;
use serde_json;
use std::borrow::Borrow;
use futures::{Future, Stream};




// defining a function to make and produce a url with the query parameters needed by the endpoint
fn params(mut url:Url, key_value: Vec<(&str, &str)>) -> String{

    // for every element in vector
    for i in key_value {
        // it is appended as a query parameter to the url
        url.query_pairs_mut().append_pair(i.0, i.1);

    }

    // makng a reference to the url to ignore the error
    let str = &url;

    // returning the final url in the form of a string
    return str.as_str().to_string()
}


async fn a(){
    // Defining the endpoint and unwrapping it because Result<Url, ParseError> is annoying
    let  endpoint = Url::parse("https://www.googleapis.com/youtube/v3/search").unwrap();

    // Making a vector for the parameters
    let param_vector= vec![("part", "snippet")];

    // getting the final url with the query parameters
    let final_url = params(endpoint, param_vector);


    // Now the request part

    // defining request
    let request = hyper::Request::get(&final_url)
    .body(Body::empty()).unwrap();

    // Https connector
    let https = HttpsConnector::new();

    // Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Getting response of the request
    let response = client.request(request).await;

    // Unwrapping request
    let mut ures = response.unwrap();

    // Getting Json
    let body = ures.into_body();
    let body_vec = hyper::body::to_bytes(body).await.unwrap();

    println!("{}", std::str::from_utf8(&body_vec).unwrap());


}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(a());

}