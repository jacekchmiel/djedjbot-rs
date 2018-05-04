extern crate djedjbot;
extern crate diesel;

use djedjbot::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use djedjbot::schema::songs::dsl::*;

    let connection = establish_connection();
    let results = songs
        .limit(10000)
        .load::<Song>(&connection)
        .expect("Error loading songs");

    println!("Displaying {} songs", results.len());
    for song in results {
        println!("{:64}   {:3} bpm", song.title, song.tempo.map(|t| t.to_string()).unwrap_or(String::from("---")));
    }
}