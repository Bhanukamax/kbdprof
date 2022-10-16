use log::{error, info};
use rdev::{listen, Event, EventType, Key};
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

fn main() {
    println!("starting kbd profiler!");
    create_table();
    let _ = WriteLogger::init(
        LevelFilter::Trace,
        Config::default(),
        File::create(".kbd_profile_log").expect("should create the log file"),
    );

    info!("Starting the kbd profiler");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
        error!("{:?}", error);
    }
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        write_to_db(key);
    }
}

fn write_to_db(key_code: Key) {
    let connection = sqlite::open(":keys").expect("should have open a db connection");
    let code = format!("INSERT INTO keys (name) VALUES ('{:?}');", key_code);

    if let Err(error) = connection.execute(code) {
        println!("error wirtting to db: {}", error);
        error!("error writting to db: {}", error);
    }
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
