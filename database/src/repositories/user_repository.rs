use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    entities::{prelude::User, user},
    models::create_user::CreateUser,
};

pub async fn find_first_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<user::Model>, sea_orm::error::DbErr> {
    let user = User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn save<'r>(
    db: &DatabaseConnection,
    data: CreateUser<'r>,
) -> Result<user::Model, sea_orm::error::DbErr> {
    let new_user = user::ActiveModel {
        fpl_id: Set(data.fpl_id),
        email: Set(data.email.to_owned()),
        google_id: Set(data.google_id),
        facebook_id: Set(data.facebook_id),
        ..Default::default()
    };

    let new_user = User::insert(new_user).exec_with_returning(db).await?;

    Ok(new_user)
}
