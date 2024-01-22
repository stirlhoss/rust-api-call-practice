use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    name: String,
    height: String,
    mass: String,
    hair_color: String,
    skin_color: String,
    eye_color: String,
    gender: String,
    homeworld: String,
    films: Vec<String>,
    species: Vec<String>,
    vehicles: Vec<String>,
    starships: Vec<String>,
    created: String,
    edited: String,
    url: String
}

#[tokio::main]
async fn main() {
    let url = "https://swapi.dev/api/people/1/";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    match response.status(){
        reqwest::StatusCode::OK => {
            println!("{:#?}", response);
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success {:#?}", parsed),
                Err(_) => println!("response does not match APIresponse"),
            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("need new token");
        },
        _ => {
            panic!("Something unexpected happened")
        }
    }
}
