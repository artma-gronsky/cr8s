use std::process::Command;

use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};


mod common;

#[test]
fn test_login(){
    let output = Command::new("cargo")
    .arg("run")
    .arg("--bin")
    .arg("cli")
    .arg("users").arg("create").arg("testadmin").arg("1234").arg("admin")
    .output().unwrap();

    println!("{:?}", output);


    let client = Client::new();

    let request_body = json!({
        "username":"testadmin",
        "password": "1234"
    });

    let response = client.post(common::LOGIN_BASE_URL).json(&request_body).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json:Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(),128);
}


#[test]
fn test_login_wrong_password(){
    let output = Command::new("cargo")
    .arg("run")
    .arg("--bin")
    .arg("cli")
    .arg("users").arg("create").arg("testadmin").arg("1234").arg("admin")
    .output().unwrap();

    println!("{:?}", output);


    let client = Client::new();
    let request_body = json!({
        "username":"testadmin",
        "password": "12345"
    });

    let response = client.post(common::LOGIN_BASE_URL).json(&request_body).send().unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_login_wrong_username(){
    let client = Client::new();
    let request_body = json!({
        "username":"someuser",
        "password": "12345"
    });

    let response = client.post(common::LOGIN_BASE_URL).json(&request_body).send().unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}


#[test]
fn test_me(){
    let client = common::get_client_with_logged_in_viewer();

    let response = client.get(format!("{}/me",common::BASE_URL)).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json:Value = response.json().unwrap();
    assert!(json.get("id").is_some());
    assert!(json.get("username").is_some());
    assert!(json.get("created_at").is_some());
    assert!(json.get("password").is_none());
    assert_eq!(json["username"], "testviewer");
}