#![allow(dead_code)]

type Callback = fn(String) -> String;

#[derive(Debug)]
enum EventType {
    Push,
    PullRequest,
    Scheduled
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
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
    routines: Vec<Callback>
}

#[derive(Debug)]
struct EventsEngine {
    registered_routines: Routines,
    queued: Vec<Event>
    alive: Vec<Event>,
}

#[derive(Debug)]
struct Routines {
    push: Vec<Callback>,
    pull_request: Vec<Callback>
    scheduled: Vec<Callback>,
}

fn main() {
    let mut plat = Platform {
        vendor: Vendor::Github
    };
    
    plat.generate_event(
        String::from("fuezito")
    ).execute();
}

