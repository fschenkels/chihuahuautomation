#![allow(dead_code)]
//#![feature(unboxed_closures)]

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
struct Event<F: Fn(&mut String) -> String> {
    context: String,
    etype: EventType,
    vendor: Vendor,
    routine: F
}

trait Executable {
    fn execute(&mut self) -> String;
}

impl<F: Fn(&mut String) -> String> Executable for Event<F> {
    fn execute(&mut self) -> String {
        (self.routine)(&mut self.context)
    }
}

fn routine_1(context: &mut String) -> String {
    println!("routine for context '{}'", context);
    String::new()
}


fn main() {
    let mut event = Event {
        context: String::from("some context here"),
        etype: EventType::Push,
        vendor: Vendor::Github,
        routine: routine_1
    };
    
    event.execute();
}
