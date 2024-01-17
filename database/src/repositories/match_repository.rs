use crate::{
    entities::{
        prelude::{Match, Transaction},
        r#match,
        sea_orm_active_enums::{MatchStatus, TransactionFlag, TransactionType},
        transaction,
    },
    models::{FindMatchesParams, MatchWithOwnerOpponentAndWinner},
    repositories::user_repository::update_d_coin,
};
use sea_orm::{
    sea_query::{Alias, Expr},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Iterable, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, RelationTrait, SelectColumns, Set,
    TransactionTrait,
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

    // collect d_coin
    update_d_coin(db, creator_id, total_d_coin, TransactionFlag::Down).await?;

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
        exclude_created_by,
        season,
    }: FindMatchesParams,
) -> Result<(Vec<MatchWithOwnerOpponentAndWinner>, u64), sea_orm::error::DbErr> {
    let query_builder = Match::find()
        // Select partial match
        .select_only()
        .columns(r#match::Column::iter().filter(|col| match col {
            &r#match::Column::IsPrivate
            | &r#match::Column::WinnerId
            | &r#match::Column::OpponentId
            | &r#match::Column::OwnerId => false,
            _ => true,
        }))
        // Select partial owner
        .select_column_as(Expr::cust("owner.id"), "owner_id")
        .select_column_as(Expr::cust("owner.fpl_id"), "owner_fpl_id")
        .select_column_as(Expr::cust("owner.name"), "owner_name")
        .select_column_as(
            Expr::cust("owner.player_first_name"),
            "owner_player_first_name",
        )
        .select_column_as(
            Expr::cust("owner.player_last_name"),
            "owner_player_last_name",
        )
        // Select partial winner
        .select_column_as(Expr::cust("winner.id"), "winner_id")
        .select_column_as(Expr::cust("winner.fpl_id"), "winner_fpl_id")
        .select_column_as(Expr::cust("winner.name"), "winner_name")
        .select_column_as(
            Expr::cust("winner.player_first_name"),
            "winner_player_first_name",
        )
        .select_column_as(
            Expr::cust("winner.player_last_name"),
            "winner_player_last_name",
        )
        // Select partial opponent
        .select_column_as(Expr::cust("opponent.id"), "opponent_id")
        .select_column_as(Expr::cust("opponent.fpl_id"), "opponent_fpl_id")
        .select_column_as(Expr::cust("opponent.name"), "opponent_name")
        .select_column_as(
            Expr::cust("opponent.player_first_name"),
            "opponent_player_first_name",
        )
        .select_column_as(
            Expr::cust("opponent.player_last_name"),
            "opponent_player_last_name",
        )
        // Join owner, opponent, winner
        .join_as(
            sea_orm::JoinType::LeftJoin,
            r#match::Relation::User3.def(),
            Alias::new("opponent"),
        )
        .join_as(
            sea_orm::JoinType::LeftJoin,
            r#match::Relation::User2.def(),
            Alias::new("owner"),
        )
        .join_as(
            sea_orm::JoinType::LeftJoin,
            r#match::Relation::User1.def(),
            Alias::new("winner"),
        )
        // Filter and more
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
        .apply_if(exclude_created_by, |query, exclude_created_by| {
            query.filter(r#match::Column::OwnerId.ne(exclude_created_by))
        })
        .apply_if(season.clone(), |query, season| {
            query.filter(r#match::Column::Season.eq(season))
        })
        .filter(r#match::Column::Status.eq(status));

    let matches = query_builder
        .to_owned()
        .offset((page - 1) * take)
        .limit(take)
        .order_by(r#match::Column::Id, Order::Asc)
        // If there are any modifier on this query, the QueryResult implementation for MatchWithOwnerOpponentAndWinner should be modified too.
        .into_model::<MatchWithOwnerOpponentAndWinner>()
        .all(db)
        .await?;

    let total = query_builder.count(db).await?;

    Ok((matches, total))
}
