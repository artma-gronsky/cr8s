use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

mod common;

#[test]
fn test_get_rustacens(){
    //Setup
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = common::crate_test_rustacean(&client);
    let rustacean2 = common::crate_test_rustacean(&client);

    //Test
    let response  = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
    assert_eq!(response.status(),StatusCode::OK);

    let body = response.json::<Value>().unwrap();
    let body_array = body.as_array().unwrap();
    assert!(body_array.contains(&rustacean1));
    assert!(body_array.contains(&rustacean2));

    // Cleanup 
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

 
#[test]
fn test_create_rustacean(){
    let client = common::get_client_with_logged_in_admin();
    let response = client.post("http://127.0.0.1:8000/rustaceans/")
    .json(&json!({
        "name": "name",
        "email": "mail"
    }))
    .send().unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id":  rustacean["id"],
        "name": "name",
        "email": "mail",
        "created_at": rustacean["created_at"]
    }));

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacens(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::crate_test_rustacean(&client);
    let response = client.get(format!("http://127.0.0.1:8000/rustaceans/{}",rustacean["id"])).send().unwrap();

    assert_eq!(response.status(),StatusCode::OK);


    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_update_rustacens(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::crate_test_rustacean(&client);
    let response = client.put(format!("http://127.0.0.1:8000/rustaceans/{}",rustacean["id"]))
    .json(&json!({  "name": "updated_name",
    "email": "updated_mail"}))
    .send()
    .unwrap();

    assert_eq!(response.status(),StatusCode::OK);

    let updated: Value = response.json().unwrap();
    assert_eq!(updated["name"], "updated_name");
    assert_eq!(updated["email"], "updated_mail");

    common::delete_test_rustacean(&client, rustacean);
}



#[test]
fn test_delete_rustacens(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::crate_test_rustacean(&client);

    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}",rustacean["id"])).send().unwrap();

    assert_eq!(response.status(),StatusCode::NO_CONTENT);

    let json_body_result = response.json::<Value>()
    .map_err(|_| "Some error");
    assert_eq!(json_body_result, Err("Some error"));
}