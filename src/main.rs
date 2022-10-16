use rdev::{listen, Event, EventType};

fn main() {
    println!("Hello, world agaaaain!");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
     if let EventType::KeyPress(key) = event.event_type {
        println!("this is the key press {:?}", key);
    }
}
