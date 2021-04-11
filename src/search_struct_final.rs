#![allow(warnings)]

use std::fs;

use hyper_tls::HttpsConnector;
use hyper::{Body, Response};
use hyper::Client;
use url::{Url};
use tokio;
use serde_json;
use std::borrow::Borrow;

fn read_key(path: String) -> String{

    let api_key: String = fs::read_to_string(path).unwrap();

    return api_key;

}

#[derive(Debug)]
struct FinalJsonResponse {
    title: String,
    description: String,
    thumbnail: String

}

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

pub async fn search(query: &str) {
    let key = read_key("src\\your_api_key.txt".parse().unwrap());

    let  endpoint = Url::parse("https://www.googleapis.com/youtube/v3/search").unwrap();

    let params_vec = vec![("q", query), ("part", "snippet"), ("key", &key)];

    let get_url = params(endpoint, params_vec);

    let request = hyper::Request::get(&get_url)
    .body(Body::empty()).unwrap();

    let https = HttpsConnector::new();

    // Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Getting response of the request
    let response = client.request(request).await.unwrap();

    let status = (&response).status();



    // Getting Json
    let body = response.into_body();
    let body_vec = hyper::body::to_bytes(body).await.unwrap();

    let res = std::str::from_utf8(&*body_vec).unwrap();

    //converting str to json obj
    let json_obj: serde_json::Value = serde_json::from_str(&res).unwrap();

    let objs = &json_obj["items"].as_array().unwrap().to_owned();

    let mut objects_of_struct = Vec::new();

    for obj in objs {

        let snippet = &obj["snippet"];
        let title = &snippet["title"];
        let description = &snippet["description"];
        let thumbnail_link = &(&(snippet["thumbnails"])["high"]);

        let struc_obj = FinalJsonResponse{
            title: title.to_string(),
            description: description.to_string(),
            thumbnail: thumbnail_link.to_string()
        };

        objects_of_struct.push(struc_obj);

    }

    for object in objects_of_struct {
        println!("{:?}\n\n", object);
    }



}

