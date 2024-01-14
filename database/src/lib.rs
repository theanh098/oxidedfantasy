pub mod repositories;
use entities::sea_orm_active_enums::MatchStatus;
pub use sea_orm;
pub mod entities;
pub mod models;

impl Default for MatchStatus {
    fn default() -> Self {
        MatchStatus::Next
    }
}
