#![allow(dead_code)]

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
    fn get_type(&self) -> EventType;
    fn get_routines(&self) -> Vec<Routine>;

    fn execute(&self) -> String {
        let to_run: Vec<Routine> = self.get_routines().iter().filter(
            |&f| std::mem::discriminant(&self.get_type()) == std::mem::discriminant(f)
        ).collect();
    }
}

impl Executable for Event {
    fn get_type(&self) -> EventType {
        self.etype
    }
    
    fn get_routines(&self) -> Vec<Routine> {
        self.routines
    }
}

fn routine_for_push(context: String) -> String {
    println!("routine for push with context '{}'", context);
    String::new()
}

fn routine_for_prs(context: String) -> String {
    println!("routine for PRs with context '{}'", context);
    String::new()
}

struct Platform {
    vendor: Vendor
}

trait EventsFactory {
    fn get_vendor(&self) -> Vendor;

    fn generate_event(
        &self,
        context: String,
    ) -> Event {
        Event {
            context: context,
            etype: EventType::Push,
            vendor: self.get_vendor(),
            routines: vec![Routine::Push(routine_for_push), Routine::Push(routine_for_prs)]
        }
    }
}

impl EventsFactory for Platform {
    fn get_vendor(&self) -> Vendor {
        self.vendor
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

