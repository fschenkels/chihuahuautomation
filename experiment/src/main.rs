
#![allow(dead_code)]

use std::collections::VecDeque;
use serde_json::Value;

type Callback = fn(Value) -> Result<String, String>;

#[derive(Debug)]
enum EventType {
    Push,
    PullRequest,
    Scheduled,
    Unknown
}

impl From<&str> for EventType {
    fn from(description: &str) -> Self {
        match description {
            "push" => EventType::Push,
            "pull_request" => EventType::PullRequest,
            "schedule" => EventType::Scheduled,
            _ => EventType::Unknown,
        }
    }
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
    context: Value,
    etype: EventType,
    vendor: Vendor,
}

impl From<Value> for Event {
    fn from(json: Value) -> Self {
        Event {
            etype: EventType::from(json["event_name"].as_str().unwrap()),
            vendor: Vendor::Github,
            context: json,
        }
    }
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
    
    fn intake(mut self, context: Value) -> Result<String, String> {
        self.queued.push_back(Event::from(context));
        Ok(String::from("It worked \\0/"))
    }
}

fn main() {
    let eng: EventsEngine = EventsEngine::new();

    let input_example: String = r###"
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

    //let json: Value = serde_json::from_str(&input_example).unwrap();
    eng.intake(
        serde_json::from_str(&input_example).unwrap()
    ).unwrap();
    println!("It worked \\0/");
    ()
}
