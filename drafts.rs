#![allow(dead_code)]

#[derive(Debug)]
enum EventType {
    Push,
    PullRequest,
    Scheduled
}

#[derive(Debug)]
enum Vendor {
    Github,
    Gitlab,
    Azure
}

#[derive(Debug)]
struct Event {
    context: String,
    etype: EventType,
    vendor: Vendor,
    routine: fn(&mut Event)
}

fn a_routine(e: &mut Event) {
    println!("Hello, world!");
    println!("Processing a {:?} event", e.etype);
}

fn main() {
    let mut event = Event {
        context: String::from("some context here"),
        etype: EventType::Push,
        vendor: Vendor::Github,
        routine: a_routine,
    };
    
    (event.routine)(&mut event);
}
