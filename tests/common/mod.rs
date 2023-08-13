use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{Value, serde_json::json};

pub static CRATES_BASE_URL: &str = "http://127.0.0.1:8000/crates";

#[ignore]
pub fn crate_test_rustacean(client: &Client) -> Value{
    let response = client.post("http://127.0.0.1:8000/rustaceans/")
    .json(&json!({
        "name": "name",
        "email": "mail"
    }))
    .send().unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

#[ignore]
pub fn delete_test_rustacean(client: &Client, rustacean: Value){
    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}",rustacean["id"])).send().unwrap();

    assert_eq!(response.status(),StatusCode::NO_CONTENT);
}

#[ignore]
pub fn crate_test_crate(client: &Client) -> Value{

    let rustacean = crate_test_rustacean(client);

    let response = client.post(CRATES_BASE_URL).json(&json!({
        "rustacean_id": rustacean["id"],
        "name":"crate_new_name",
        "code": "crate_new_code",
        "version":"0.23.5"
    })).send().unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json::<Value>().unwrap()
}

#[ignore]
pub fn delete_test_crate(client: &Client, a_crate: Value){
    let response = client.delete(format!("{}/{}", CRATES_BASE_URL, a_crate["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", a_crate["rustacean_id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
