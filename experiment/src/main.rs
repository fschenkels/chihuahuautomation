
#![allow(dead_code)]

use std::collections::VecDeque;
use serde_json::Value;
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

type Callback = fn(Value) -> Result<String, String>;

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
    routines: Vec<Callback>,
}

#[derive(Debug)]
struct Routines {
    push: Vec<Callback>,
    pull_request: Vec<Callback>,
    scheduled: Vec<Callback>,
}

#[derive(Debug)]
struct EventsEngine {
    registered_routines: Routines,
    queued: VecDeque<Event>,
    alive: VecDeque<Event>,
}

impl EventsEngine {
    fn new() -> EventsEngine {
        EventsEngine {
            registered_routines: Routines {
                push: Vec::new(),
                pull_request: Vec::new(),
                scheduled: Vec::new(),
            },
            queued: VecDeque::new(),
            alive: VecDeque::new(),
        }
    }
    
    fn intake(self, context: Value) -> Result<String, String> {
        println!("Execution id {}", context["run_id"]);
        Ok(String::from("It worked \\0/"))
    }
}

fn main() {
    let eng: EventsEngine = EventsEngine::new();

    let input_example = r###"
{
  "token": "***",
  "job": "dump_contexts_to_log",
  "ref": "refs/heads/my_branch",
  "sha": "c27d339ee6075c1f744c5d4b200f7901aad2c369",
  "repository": "octocat/hello-world",
  "repository_owner": "octocat",
  "repositoryUrl": "git://github.com/octocat/hello-world.git",
  "run_id": "1536140711",
  "run_number": "314",
  "retention_days": "90",
  "run_attempt": "1",
  "actor": "octocat",
  "workflow": "Context testing",
  "head_ref": "",
  "base_ref": "",
  "event_name": "push",
  "server_url": "https://github.com",
  "api_url": "https://api.github.com",
  "graphql_url": "https://api.github.com/graphql",
  "ref_name": "my_branch",
  "ref_protected": false,
  "ref_type": "branch",
  "secret_source": "Actions",
  "workspace": "/home/runner/work/hello-world/hello-world",
  "action": "github_step",
  "event_path": "/home/runner/work/_temp/_github_workflow/event.json",
  "action_repository": "",
  "action_ref": "",
  "path": "/home/runner/work/_temp/_runner_file_commands/add_path_b037e7b5-1c88-48e2-bf78-eaaab5e02602",
  "env": "/home/runner/work/_temp/_runner_file_commands/set_env_b037e7b5-1c88-48e2-bf78-eaaab5e02602"
}"###.to_string();

    eng.intake(
        serde_json::from_str(&input_example).unwrap()
    ).unwrap();
    ()
}
