use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl, ExpressionMethods, dsl::IntervalDsl, BelongingToDsl};
use crate::models::{NewRole, Role, User, UserRole};
use crate::schema::roles;

pub struct RoleRepository;

impl RoleRepository {
    pub fn find_by_code(c: &mut PgConnection, code: &String) -> QueryResult<Role>{
        roles::table.filter(roles::code.eq(code)).first(c)
    }
    
    pub fn create(c: &mut PgConnection, code: String, name: String) ->  QueryResult<Role>{
        let new_role = NewRole{
            code,
            name
        };

        diesel::insert_into(roles::table).values(new_role).get_result(c)
    }

    pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).get_results(c)
    }

    pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user).get_results(c)?;

        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(c, role_ids)
    }

}