use crate::handle_surf_response;

#[derive(serde::Deserialize)]
pub struct Entry {
    pub id: i32,
    pub started_event: i32,
    pub favourite_team: i32,
    pub player_region_id: i32,
    pub last_deadline_total_transfers: i32,
    pub joined_time: String,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub name: String,
    pub player_region_iso_code_long: String,
    pub name_change_blocked: bool,
    pub summary_overall_points: Option<i32>,
    pub summary_overall_rank: Option<i32>,
    pub summary_event_points: Option<i32>,
    pub summary_event_rank: Option<i32>,
    pub current_event: Option<i32>,
    pub last_deadline_bank: Option<i32>,
    pub last_deadline_value: Option<i32>,
    pub kit: Option<String>,
}

pub async fn get_entry(fpl_id: i32) -> Result<Entry, surf::Error> {
    let mut response = surf::get(format!(
        "https://fantasy.premierleague.com/api/entry/{}",
        fpl_id
    ))
    .await?;

    handle_surf_response(&mut response).await
}
