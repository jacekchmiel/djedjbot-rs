use diesel::sql_types::*;

#[derive(Queryable)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub tempo: Option<i16>,
}