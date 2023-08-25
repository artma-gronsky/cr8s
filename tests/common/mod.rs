use std::process::Command;
use reqwest::{blocking::{Client, ClientBuilder}, StatusCode, header::{HeaderMap, self, HeaderValue}};
use rocket::serde::json::{serde_json::json, Value};

pub static CRATES_BASE_URL: &str = "http://127.0.0.1:8000/crates";
pub static LOGIN_BASE_URL: &str = "http://127.0.0.1:8000/login";

#[ignore]
pub fn crate_test_rustacean(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans/")
        .json(&json!({
            "name": "name",
            "email": "mail"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

#[ignore]
pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[ignore]
pub fn crate_test_crate(client: &Client) -> Value {
    let rustacean = crate_test_rustacean(client);

    let response = client
        .post(CRATES_BASE_URL)
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "name":"crate_new_name",
            "code": "crate_new_code",
            "version":"0.23.5"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json::<Value>().unwrap()
}

#[ignore]
pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/{}", CRATES_BASE_URL, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            a_crate["rustacean_id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_client_with_logged_in_admin() -> Client {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("testadmin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();

    let request_body = json!({
        "username":"testadmin",
        "password": "1234"
    });

    let response = client
        .post(LOGIN_BASE_URL)
        .json(&request_body)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());


    let token = json["token"].as_str().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
