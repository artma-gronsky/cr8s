use diesel::{
    dsl::{now, IntervalDsl},
    ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};

use crate::{
    models::{Crate, NewCrate},
    schema::crates,
};

pub struct CrateRepository;

impl CrateRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(c)
    }

    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.offset(0).limit(limit).load(c)
    }

    pub fn create(c: &mut PgConnection, new: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table).values(new).get_result(c)
    }

    pub fn update(c: &mut PgConnection, id: i32, update: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(update.rustacean_id),
                crates::name.eq(update.name),
                crates::code.eq(update.code),
                crates::version.eq(update.version),
                crates::description.eq(update.description),
            ))
            .get_result(c)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c)
    }

    pub(crate) fn find_since(c: &mut PgConnection, hours_since: i32) -> QueryResult<Vec<Crate>> {
        crates::table
            .filter(crates::created_at.ge(now - hours_since.hours()))
            .load(c)
    }
}
