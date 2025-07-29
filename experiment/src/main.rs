
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
    queued: VecDeque<Event>,
    alive: VecDeque<Event>,
}

#[derive(Debug)]
struct Routines {
    push: Vec<Callback>,
    pull_request: Vec<Callback>,
    scheduled: Vec<Callback>,
}

fn main() {
    print!("fue");
}
