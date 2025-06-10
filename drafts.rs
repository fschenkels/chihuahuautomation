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
enum Routine<F: Fn()> {
    Push(F),
    PullRequest(F),
    Scheduled(F)
}

#[derive(Debug)]
struct Event<F: Fn()> {
    context: String,
    etype: EventType,
    vendor: Vendor,
    routines: Vec<Routine<F>>
}

trait Executable {
    fn get_type(&self) -> EventType;

    fn execute(&mut self) {
        match self.get_type() {
            EventType::Push => {},
            EventType::PullRequest => {},
            EventType::Scheduled => {},
        }
    }
}

fn a_routine<F: Fn()>(e: &mut Event<F>) {
    println!("Hello, world!");
    println!("Processing a {:?} event", e.etype);
}

fn main() {
    let mut event = Event {
        context: String::from("some context here"),
        etype: EventType::Push,
        vendor: Vendor::Github,
        routines: vec![Routine::Push(a_routine)]
        //routine: a_routine,
    };
    
    //(event.routine)(&mut event);
}
