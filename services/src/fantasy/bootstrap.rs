use crate::handle_surf_response;

#[derive(serde::Deserialize, Debug)]
pub struct Event {
    pub average_entry_score: i32,
    pub data_checked: bool,
    pub deadline_time: String,
    pub deadline_time_epoch: i32,
    pub deadline_time_game_offset: i32,
    pub finished: bool,
    pub highest_score: Option<i32>,
    pub highest_scoring_entry: Option<i32>,
    pub id: i32,
    pub is_current: bool,
    pub is_next: bool,
    pub is_previous: bool,
    pub most_captained: Option<i32>,
    pub most_selected: Option<i32>,
    pub most_transferred_in: Option<i32>,
    pub most_vice_captained: Option<i32>,
    pub name: String,
    pub top_element: Option<i32>,
    pub transfers_made: i32,
    pub cup_leagues_created: bool,
    pub h2h_ko_matches_created: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct Bootstrap {
    pub events: Vec<Event>,
}

pub async fn get_bootstrap() -> Result<Bootstrap, surf::Error> {
    let mut response = surf::get("https://fantasy.premierleague.com/api/bootstrap-static/").await?;

    handle_surf_response(&mut response).await
}
