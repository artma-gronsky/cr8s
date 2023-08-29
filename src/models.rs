use std::{io::Write, str::FromStr};

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, AsChangeset, Associations, Identifiable, expression::AsExpression, sql_types::{Text}, deserialize::{FromSql, self, FromSqlRow}, serialize::{ToSql, self, Output, IsNull}, pg::{PgValue, Pg}};
use serde::{Serialize, Deserialize};
use crate::schema::*;


#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Rustacean{
    #[serde(skip_deserializing)]
    pub id: i32, 
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean{
    pub name: String,
    pub email: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Crate{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=crates)]
pub struct NewCrate{
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>
}

#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct User{
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime    
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser{
    pub username: String,
    pub password: String,
}



#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct Role{
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime    
}

#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole{
    pub code: RoleCode,
    pub name: String,
}


#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}


#[derive(Insertable)]
#[diesel(table_name=users_roles)] 
pub struct NewUserRole{
    pub user_id:i32,
    pub role_id: i32
}

#[derive(AsExpression, Debug, Serialize, Deserialize, FromSqlRow, Clone, Default, PartialEq)]
#[diesel(sql_type=Text)]
  pub enum RoleCode {
    Admin,
    Editor,
    #[default] Viewer
}


impl FromStr for RoleCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err(())
        }
    }
}

impl ToString for RoleCode{
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => "admin".to_string(),
            RoleCode::Viewer => "viewer".to_string(),
            RoleCode::Editor => "editor".to_string(),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode
{
    fn from_sql(value: PgValue) -> deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer)
        }
    }
}

impl ToSql<Text, Pg> for RoleCode
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let _ = match self {
            RoleCode::Admin => out.write_all(b"admin"),
            RoleCode::Viewer => out.write_all(b"viewer"),
            RoleCode::Editor => out.write_all(b"editor")
        };

        Ok(IsNull::No) 
    }
}