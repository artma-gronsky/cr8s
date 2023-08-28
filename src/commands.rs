use std::str::FromStr;

use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::crates::CrateRepository;
use crate::repositories::roles::RoleRepository;
use crate::repositories::users::UserRepository;
use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport, Address};
use tera::{Context, Tera};

fn get_connection() -> PgConnection {
    let db_url = std::env::var("DATABASE_URL").expect("Cannot load db url from env");

    PgConnection::establish(&db_url).expect("Can not connect to postgres")
}

fn load_template_egine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parcing error(s):{}", e);
    })
}

pub fn crate_user(username: String, password: String, role_codes: Vec<String>) {
    let password_hash = auth::hash_password(&password).unwrap();

    let new_user = NewUser {
        username,
        password: password_hash,
    };
    let mut c = get_connection();

    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v).unwrap_or(Default::default()))
        .collect();
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();

    println!("User created {:?}", user);

    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    for role in roles {
        println!("Role assigned {:?}", role);
    }
}

pub fn list_users() {
    let mut c = get_connection();
    let users = UserRepository::find_all_with_roles(&mut c).unwrap();

    println!("User list:");
    for (idx, user) in users.iter().enumerate() {
        println!("{}) {:?}", idx + 1, user);
    }
}

pub fn delete_user(id: i32) {
    let mut c = get_connection();
    UserRepository::delete(&mut c, id).unwrap();
}

pub fn send_digest(receiver_email: String, hours_since: i32) {
    let mut c = get_connection();

    let crates = CrateRepository::find_since(&mut c, hours_since).unwrap();

    if crates.len() > 0 {
        println!("Send the digest for {} crates", crates.len());
        let tera = load_template_egine();
        let mut context = Context::new();
        context.insert("crates", &crates);
        let year: i32 = Utc::now().year();
        context.insert("year", &year);
        let html_body = tera.render("email/digest.html", &context).unwrap();

        let message = lettre::Message::builder()
            .subject("Cr8s digest")
            .from(lettre::message::Mailbox::new(Some("info@cr8s.com".to_string()), Address::new("info" ,"cr8s.com").unwrap()))
            .to(receiver_email.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();

        let smtp_host = 
            std::env::var("SMTP_HOST").expect("Cannot load smtp host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load smtp username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load smtp password from env");

        let creadentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&smtp_host)
            .unwrap()
            .credentials(creadentials)
            .build();

        mailer.send(&message).unwrap();
    }
}
