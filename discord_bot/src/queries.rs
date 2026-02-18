use diesel::{RunQueryDsl, SelectableHelper};

use crate::error::AppError;
use crate::models::{
    NewReactionMessage, NewSelfAssignableRole, ReactionMessage, SelfAssignableRole,
};
use crate::schema::self_assignable_roles;

pub async fn create_self_assignable_role(
    pool: deadpool_diesel::postgres::Pool,
    new_role: NewSelfAssignableRole,
) -> Result<SelfAssignableRole, AppError> {
    let conn = pool.get().await.unwrap();
    let res = conn
        .interact(|conn| {
            diesel::insert_into(self_assignable_roles::table)
                .values(new_role)
                .returning(SelfAssignableRole::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();
    Ok(res)
}

pub async fn create_reaction_message(
    _pool: deadpool_diesel::postgres::Pool,
    _new_message: NewReactionMessage,
) -> Result<ReactionMessage, AppError> {
    todo!()
}

pub async fn delete_self_assignable_role(
    _pool: deadpool_diesel::postgres::Pool,
    _guild_id: String,
    _role_id: String,
) -> Result<(), AppError> {
    todo!()
}

pub async fn get_self_assignable_roles(
    _pool: deadpool_diesel::postgres::Pool,
    _guild_id: String,
) -> Result<Vec<SelfAssignableRole>, AppError> {
    todo!()
}

pub async fn get_reaction_message(
    _pool: deadpool_diesel::postgres::Pool,
    _guild_id: String,
    _message_id: String,
) -> Result<Option<ReactionMessage>, AppError> {
    todo!()
}
