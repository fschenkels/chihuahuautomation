#![allow(dead_code)]

use std::collections::VecDeque;

//fn main() {
//    let mut queue: VecDeque<i32> = VecDeque::new();

//    // Enqueue elements
//    queue.push_back(1);
//    queue.push_back(2);
//    queue.push_back(3);

//    // Dequeue elements
//    while let Some(item) = queue.pop_front() {
//        println!("Dequeued: {}", item);
//    }
//}

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
    push: VecDeque<Callback>,
    pull_request: VecDeque<Callback>,
    scheduled: VecDeque<Callback>,
}

fn main() {
    let mut plat = Platform {
        vendor: Vendor::Github
    };
    
    plat.generate_event(
        String::from("fuezito")
    ).execute();
}

