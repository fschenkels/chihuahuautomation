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
enum Routine {
    Push(fn(String) -> String),
    PullRequest(fn(String) -> String),
    Scheduled(fn(String) -> String)
}

#[derive(Debug)]
struct Event {
    context: String,
    etype: EventType,
    vendor: Vendor,
    routines: Vec<Routine>
}

trait Executable {
    fn execute(self) -> String;
}

impl Executable for Event {
    fn execute(self) -> String {
        match self.routines[0] {
            Routine::Push(r) => (r)(self.context),
            Routine::PullRequest(r) => (r)(self.context),
            Routine::Scheduled(r) => (r)(self.context)
        }
    }
}

fn routine_1(context: String) -> String {
    println!("routine for context '{}'", context);
    String::new()
}

struct Platform {
    vendor: Vendor
}

trait EventsFactory {
    fn generate_event(
        self,
        context: String,
    ) -> Event;
}

impl EventsFactory for Platform {
    fn generate_event(
        self,
        context: String,
    ) -> Event {
        Event {
            context: context,
            etype: EventType::Push,
            vendor: self.vendor,
            routines: vec![Routine::Push(routine_1)]
        }
    }
    
}

fn main() {
    let mut plat = Platform {
        vendor: Vendor::Github
    };
    
    plat.generate_event(
        String::from("fuezito")
    ).execute();
}
