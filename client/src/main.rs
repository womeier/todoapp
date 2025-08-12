use clap::CommandFactory;
use clap::{Parser, Subcommand};
use clap_complete::{generate, Shell};
use client_api::apis::client::APIClient;
use client_api::apis::configuration::Configuration;
use client_api::models::*;
use serde::Deserialize;
use std::env;
use std::io;
use std::process;

#[derive(Parser)]
#[command(name="todoclient", version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    List {
        #[arg(short, long)]
        label: Option<String>,
    },

    Create {
        #[arg(short, long)]
        title: String,
    },

    Close {
        #[arg(short, long)]
        tid: i32,
    },

    ReOpen {
        #[arg(short, long)]
        tid: i32,
    },

    SetLabel {
        #[arg(short, long)]
        tid: i32,

        #[arg(short, long)]
        label: String,
    },

    Completions {
        #[arg(long = "shell", value_enum)]
        shell: Shell,
    },
}

/// The config format for ./config/todoap/config.json
#[derive(Deserialize)]
struct ClientConfig {
    url: String,
    port: Option<i16>,
    api_token: String,
}

fn initial_configuration(config_path: Option<String>) -> Configuration {
    let default_config_path = format!(
        "{}/.config/todoapp/config.json",
        env::var_os("HOME").unwrap().to_str().unwrap()
    );

    let config_path = config_path.unwrap_or(default_config_path);
    let config_file = std::fs::read_to_string(config_path)
        .expect("Failed to read config file. It is expected at $HOME/.config/todoapp/config.json or at the provided path.");

    let config: ClientConfig =
        serde_json::from_str(&config_file).expect("Failed to parse config file.");

    let base_path = if let Some(port) = config.port {
        format!("{}:{}", config.url, port)
    } else {
        config.url
    };

    Configuration {
        oauth_access_token: Some(config.api_token),
        base_path,
        ..Default::default()
    }
}

async fn process_cmd(client: APIClient, command: Command) {
    match command {
        Command::List { label } => {
            let label = label.unwrap_or("Now".to_string());
            let tasks = client
                .default_api()
                .list_tasks()
                .await
                .expect("Failed to retrieve list of tasks.");

            let mut tasks: Vec<&Task> = tasks.iter().filter(|t| t.label == label).collect();
            tasks.sort_by(|t1, t2| t1.id.cmp(&t2.id));

            println!("Tasks({label}):");
            for t in tasks {
                println!("{:5}: {}", t.id, t.title);
            }
        }

        Command::Create { title } => {
            let t = NewTask::new(title.to_string());

            let task = client
                .default_api()
                .create_task(&title, None, None, t)
                .await
                .expect("Failed to create task.");

            println!("Created task {}.", task.id)
        }

        Command::Close { tid } => {
            let t = UpdateTask {
                done: Some(Some(true)),
                label: None,
            };
            let err_msg = format!("Failed to close task [{tid}].");

            client
                .default_api()
                .update_task(None, None, tid, t)
                .await
                .expect(&err_msg);

            println!("Closed task {tid}.")
        }

        Command::ReOpen { tid } => {
            let t = UpdateTask {
                done: Some(Some(false)),
                label: None,
            };
            let err_msg = format!("Failed to reopen task [{tid:?}].");

            client
                .default_api()
                .update_task(None, None, tid, t)
                .await
                .expect(&err_msg);

            println!("Reopened task {tid}.")
        }

        Command::SetLabel { tid, label } => {
            let t = UpdateTask {
                done: None,
                label: Some(Some(label.clone())),
            };
            let err_msg = format!("Failed to update [{tid:?}].");

            client
                .default_api()
                .update_task(None, None, tid, t)
                .await
                .expect(&err_msg);

            println!("Updated task {tid} to {label}.")
        }
        Command::Completions { shell: _ } => { /* unreachable clause, not handled here */ }
    };
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let api_config = initial_configuration(args.config);
    let client = APIClient::new(api_config);

    if let Command::Completions { shell } = args.command {
        let mut cmd = Cli::command();
        eprintln!("Generating completions file for {shell:?}.");
        let cmd_name = cmd.get_name().to_string();
        generate(shell, &mut cmd, cmd_name, &mut io::stdout());
        process::exit(0);
    }

    process_cmd(client, args.command).await;
}
