use crate::{
    entities::{
        prelude::{Match, MatchMonitor, MatchOpponent, Transaction, User},
        r#match,
        sea_orm_active_enums::{MatchStatus, TransactionFlag, TransactionType},
        transaction, user,
    },
    models::{FindMatchesParams, MatchWithOwnerAndOpponentAndMonitor},
};
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, LoaderTrait,
    QueryFilter, QuerySelect, QueryTrait, Set, TransactionTrait,
};

pub async fn update_all_next_round_to_live_by_gameweek(
    db: &DatabaseConnection,
    gameweek: i32,
) -> Result<(), sea_orm::error::DbErr> {
    let active_model = r#match::ActiveModel {
        status: Set(MatchStatus::Live),
        ..Default::default()
    };

    Match::update_many()
        .set(active_model)
        .filter(r#match::Column::Status.eq(MatchStatus::Next))
        .filter(r#match::Column::Gameweek.eq(gameweek))
        .exec_with_returning(db)
        .await?;

    Ok(())
}

pub async fn update_all_live_to_finished_by_gameweek(
    db: &DatabaseConnection,
    gameweek: i32,
) -> Result<(), sea_orm::error::DbErr> {
    let active_model = r#match::ActiveModel {
        status: Set(MatchStatus::Finished),
        ..Default::default()
    };

    Match::update_many()
        .set(active_model)
        .filter(r#match::Column::Status.eq(MatchStatus::Live))
        .filter(r#match::Column::Gameweek.eq(gameweek))
        .exec_with_returning(db)
        .await?;

    Ok(())
}

pub async fn create_matches(
    db: &DatabaseConnection,
    creator_id: i32,
    matches: Vec<r#match::ActiveModel>,
    game_week: i32,
    total_d_coin: i32,
) -> Result<(), sea_orm::error::DbErr> {
    let txn = db.begin().await?;
    let quantity = matches.len();

    // create matches
    Match::insert_many(matches)
        .exec_without_returning(db)
        .await?;

    // create monitors

    // collect d_coin
    let mut query = Query::update();
    query
        .table(user::Entity)
        .value(
            user::Column::DCoin,
            Expr::col(user::Column::DCoin).sub(total_d_coin),
        )
        .and_where(Expr::col(user::Column::Id).eq(creator_id));

    let stmt = db.get_database_backend().build(&query);
    db.execute(stmt).await?;

    // create transactions
    let metadata = serde_json::json!({
        "quantity": quantity,
        "on_gameweek": game_week,
    });
    Transaction::insert(transaction::ActiveModel {
        d_coin: Set(total_d_coin),
        message: Set(format!("You have created {} matches", { quantity })),
        flag: Set(TransactionFlag::Down),
        metadata: Set(metadata),
        owner_id: Set(creator_id),
        r#type: Set(TransactionType::CreateMatch),
        ..Default::default()
    })
    .exec_without_returning(db)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn find_matches(
    db: &DatabaseConnection,
    FindMatchesParams {
        created_by,
        joined_by_or_created,
        page,
        status,
        take,
    }: FindMatchesParams,
) -> Result<Vec<MatchWithOwnerAndOpponentAndMonitor>, sea_orm::error::DbErr> {
    let matches = Match::find()
        .apply_if(created_by, |query, owner_id| {
            query.filter(r#match::Column::OwnerId.eq(owner_id))
        })
        .apply_if(joined_by_or_created, |query, joined_by_or_created| {
            query.filter(
                Condition::any()
                    .add(r#match::Column::OpponentId.eq(joined_by_or_created))
                    .add(r#match::Column::OwnerId.eq(joined_by_or_created)),
            )
        })
        .filter(r#match::Column::Status.eq(status))
        .offset((page as u64 - 1) * take as u64)
        .limit(take as u64)
        .all(db)
        .await?;

    let owners = matches.load_one(User, db).await?;
    let monitors = matches.load_one(MatchMonitor, db).await?;
    dbg!(&monitors);
    let opponents = matches.load_one(MatchOpponent, db).await?;
    dbg!(&opponents);

    let matches = matches
        .into_iter()
        .zip(owners)
        .zip(opponents)
        .zip(monitors)
        .map(
            |(((r#match, owner), opponent), monitor)| MatchWithOwnerAndOpponentAndMonitor {
                r#match,
                monitor,
                opponent,
                owner,
            },
        )
        .collect();

    Ok(matches)
}
