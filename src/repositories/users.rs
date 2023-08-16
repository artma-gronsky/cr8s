use crate::models::{NewUser, NewUserRole, Role, User, UserRole};
use crate::repositories::roles::RoleRepository;
use crate::schema::users;
use crate::schema::users_roles;
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};

pub struct UserRepository;

impl UserRepository {
    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for code in role_codes {
            let role: Role = {
                if let Ok(role_by_code) = RoleRepository::find_by_code(c, &code) {
                    role_by_code
                } else {
                    RoleRepository::create(c, code.clone(), code.clone())?
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
        diesel::delete(users::table.find(id)).execute(c)
    }

    pub fn get_all(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load(c)
    }
}
