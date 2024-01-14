pub struct CreateUser<'r> {
    pub email: &'r str,
    pub fpl_id: Option<i32>,
    pub google_id: Option<String>,
    pub facebook_id: Option<String>,
}
