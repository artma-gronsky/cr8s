use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

mod common;

#[test]
fn test_create_crate() {
    let client = common::get_client_with_logged_in_admin();

    let rustacean = common::crate_test_rustacean(&client);

    let request_body = json!({
        "rustacean_id": rustacean["id"],
        "name":"crate_new_name",
        "code": "crate_new_code",
        "version":"0.23.5"
    });

    let response = client
        .post(common::CRATES_BASE_URL)
        .json(&request_body)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate["name"], request_body["name"]);

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "name":"crate_new_name",
            "code": "crate_new_code",
            "version":"0.23.5",
            "created_at": a_crate["created_at"],
            "description": a_crate["description"]
        })
    );

    common::delete_test_crate(&client, a_crate);
}

#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();

    let a_crate = common::crate_test_crate(&client);

    let update_crate_json = json!({
        "rustacean_id": a_crate["rustacean_id"],
        "name":"crate_modifyed_name",
        "code": "crate_modifyed_code",
        "version":"0.23.5",
        "description": a_crate["description"]
    });

    let response = client
        .put(format!("{}/{}", common::CRATES_BASE_URL, a_crate["id"]))
        .json(&update_crate_json)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let reponse_json: Value = response.json().unwrap();
    assert_eq!(
        reponse_json,
        json!({
            "id": a_crate["id"],
            "rustacean_id": a_crate["rustacean_id"],
            "name": update_crate_json["name"],
            "code": update_crate_json["code"],
            "version":"0.23.5",
            "created_at": a_crate["created_at"],
            "description": a_crate["description"]
        })
    );

    common::delete_test_crate(&client, a_crate);
}

#[test]
fn test_veiw_crates() {
    let client = common::get_client_with_logged_in_admin();

    let crate1 = common::crate_test_crate(&client);
    let crate2 = common::crate_test_crate(&client);

    let response = client.get(common::CRATES_BASE_URL).send().unwrap();
    let response_json: Value = response.json().unwrap();
    let crates_array = response_json.as_array().unwrap();

    assert!(crates_array.contains(&crate1));
    assert!(crates_array.contains(&crate2));

    common::delete_test_crate(&client, crate1);
    common::delete_test_crate(&client, crate2);
}

#[test]
fn get_one_crate() {
    let client = common::get_client_with_logged_in_admin();

    let a_crate = common::crate_test_crate(&client);

    let response = client
        .get(format!("{}/{}", common::CRATES_BASE_URL, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.json::<Value>().unwrap(), a_crate);

    common::delete_test_crate(&client, a_crate);
}

#[test]
fn delete_crate() {
    let client = common::get_client_with_logged_in_admin();

    let a_crate = common::crate_test_crate(&client);

    let response = client
        .delete(format!("{}/{}", common::CRATES_BASE_URL, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let body = response.json::<Value>();
    assert!(body.is_err());

    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            a_crate["rustacean_id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn delete_crate_wrong_user_role_failed() {
    let client = common::get_client_with_logged_in_viewer();

    let some_crate_id = 1;
    let response = client
        .delete(format!("{}/{}", common::CRATES_BASE_URL, some_crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let body = response.json::<Value>();
    assert!(body.is_err());
}
