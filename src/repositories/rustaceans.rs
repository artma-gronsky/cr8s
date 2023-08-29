use crate::models::{NewRustacean, Rustacean};
use crate::schema::rustaceans;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c)
    }

    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(c)
    }

    pub fn create(c: &mut PgConnection, new: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new)
            .get_result(c)
    }

    pub fn update(c: &mut PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}
