use sea_orm::{Linked, RelationTrait};

use super::{r#match, user};

pub struct MatchToOwner;
pub struct MatchToOpponent;
pub struct MatchToWinner;

impl Linked for MatchToOwner {
    type FromEntity = r#match::Entity;
    type ToEntity = user::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![r#match::Relation::User2.def()]
    }
}

impl Linked for MatchToOpponent {
    type FromEntity = r#match::Entity;
    type ToEntity = user::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![r#match::Relation::User3.def()]
    }
}

impl Linked for MatchToWinner {
    type FromEntity = r#match::Entity;
    type ToEntity = user::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![r#match::Relation::User1.def()]
    }
}
