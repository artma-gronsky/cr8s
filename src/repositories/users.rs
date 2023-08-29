use crate::models::{NewUser, NewUserRole, Role, RoleCode, User, UserRole};
use crate::repositories::roles::RoleRepository;
use crate::schema::{roles, users, users_roles};
use diesel::*;

pub struct UserRepository;

impl UserRepository {
    pub fn find_all_with_roles(
        c: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users: Vec<User> = users::table.load(c)?;
        let results = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);

        Ok(users.into_iter().zip(results).collect())
    }

    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for code in role_codes {
            let role: Role = {
                if let Ok(role_by_code) = RoleRepository::find_by_code(c, &code) {
                    role_by_code
                } else {
                    RoleRepository::create(c, code.clone(), code.to_string())?
                }
            };

            let new_user_role = NewUserRole {
                user_id: user.id,
                role_id: role.id,
            };
            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)?;
        }

        Ok(user)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id))).execute(c)?;
        diesel::delete(users::table.find(id)).execute(c)
    }

    pub fn get_all(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load(c)
    }

    pub fn get_by_name(c: &mut PgConnection, username: &str) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).first(c)
    }
    pub fn get_by_id(c: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).first(c)
    }
}
