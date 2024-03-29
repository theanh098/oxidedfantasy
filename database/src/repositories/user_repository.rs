use crate::entities::{prelude::User, sea_orm_active_enums::TransactionFlag, user};
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryTrait, Set,
};

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<user::Model>, sea_orm::error::DbErr> {
    User::find().filter(user::Column::Id.eq(id)).one(db).await
}

pub async fn save<'r>(
    db: &DatabaseConnection,
    data: user::ActiveModel,
) -> Result<user::Model, sea_orm::error::DbErr> {
    let new_user = User::insert(data).exec_with_returning(db).await?;

    Ok(new_user)
}

pub async fn find_first_by_platform_id<S: Into<String>>(
    db: &DatabaseConnection,
    google_id: Option<S>,
    facebook_id: Option<S>,
) -> Result<Option<user::Model>, sea_orm::error::DbErr> {
    let user = User::find()
        .apply_if(google_id, |query, google_id| {
            query.filter(user::Column::GoogleId.eq(google_id.into()))
        })
        .apply_if(facebook_id, |query, facebook_id| {
            query.filter(user::Column::GoogleId.eq(facebook_id.into()))
        })
        .one(db)
        .await?;

    Ok(user)
}

pub async fn update_fpl_id(
    db: &DatabaseConnection,
    user_id: i32,
    fpl_id: i32,
) -> Result<(), sea_orm::error::DbErr> {
    User::update(user::ActiveModel {
        fpl_id: Set(Some(fpl_id)),
        ..Default::default()
    })
    .filter(user::Column::Id.eq(user_id))
    .exec(db)
    .await
    .map(|_| ())
}

pub async fn update_d_coin(
    db: &DatabaseConnection,
    user_id: i32,
    d_coin: i32,
    kind: TransactionFlag,
) -> Result<(), sea_orm::error::DbErr> {
    let mut query = Query::update();

    query
        .table(user::Entity)
        .value(
            user::Column::DCoin,
            match kind {
                TransactionFlag::Down => Expr::col(user::Column::DCoin).sub(d_coin),
                TransactionFlag::Up => Expr::col(user::Column::DCoin).add(d_coin),
            },
        )
        .and_where(user::Column::Id.eq(user_id));

    db.execute(db.get_database_backend().build(&query)).await?;

    Ok(())
}
