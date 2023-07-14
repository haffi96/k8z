// use clap::Command;
use clap::Parser;
use core::fmt;
use duct::cmd;
use inquire::{Select, Text};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use std::{
    error::Error,
    io::{self, BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
};
use tracing::info;

#[derive(Parser, Debug)]
struct Cli {
    verb: String,
    resource: String,
}

#[derive(Debug)]
enum PodActions {
    Describe,
    Exec,
    PortForward,
    Delete,
    ClipboardCopy,
}

impl fmt::Display for PodActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PodActions::Describe => write!(f, "Describe pod in yaml"),
            PodActions::Exec => write!(f, "Create interactive shell"),
            PodActions::PortForward => write!(f, "Port forward"),
            PodActions::Delete => write!(f, "Delete pod"),
            PodActions::ClipboardCopy => write!(f, "Copy pod name to clipboard"),
        }
    }
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: could not read line");
    return line.trim().to_string();
}

fn pod_actions_handler(pod_choice: String) -> Result<(), Box<dyn Error>> {
    let pod_actions = vec![
        PodActions::Describe,
        PodActions::Exec,
        PodActions::PortForward,
        PodActions::Delete,
        PodActions::ClipboardCopy,
    ];
    let actions = Select::new("Pick an action to perform!", pod_actions).prompt();

    match actions {
        Ok(PodActions::Describe) => {
            let cmd_args = vec!["-n", "default", "describe", "pod", &pod_choice];
            let out = cmd("kubectl", cmd_args).stderr_null().read();
            match out {
                Ok(res) => println!("{}", res),
                Err(e) => println!("feck"),
            }
        }

        Ok(PodActions::Exec) => {
            let cmd_args = vec!["-n", "default", "exec", "-it", &pod_choice, "/bin/bash"];
            // let mut cmd = Command::new("kubectl")
            //     .args(cmd_args)
            //     .stdout(Stdio::piped())
            //     .stderr(Stdio::piped())
            //     .spawn()?;
            //
            // let mut stdout = cmd.stdout.take().unwrap();
            // let mut stderr = cmd.stderr.take().unwrap();
            //
            // //write
            // let stdin = std::io::stdin();
            // let mut input: Vec<String> = Vec::new();
            // let mut line = String::new();
            //
            // //read
            // for line in BufReader::new(stdout).lines() {
            //     println!("{}", line?);
            // }
        }

        Ok(PodActions::PortForward) => {
            info!("Selected PortForward to {}", pod_choice);
            let port_mapping = Text::new("Define port mapping. local_port:container_port >> ")
                .prompt()
                .unwrap();
            let cmd_args = vec!["-n", "default", "port-forward", &pod_choice, &port_mapping];
            let mut cmd = Command::new("kubectl")
                .args(cmd_args)
                .stdout(Stdio::piped())
                .spawn()?;

            for line in BufReader::new(cmd.stdout.take().ok_or("stdout missing")?).lines() {
                println!("{}", line?);
            }
            cmd.wait()?;
        }

        Ok(PodActions::Delete) => info!("Selected Delete for {}", pod_choice),

        Ok(PodActions::ClipboardCopy) => {
            info!("Selected ClipboardCopy for {}", pod_choice)
        }

        Err(_) => info!("err fuck"),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    match args.verb.as_str() {
        "get" => match args.resource.as_str() {
            "pods" => {
                // Manage pods
                let pods: Api<Pod> = Api::default_namespaced(client);

                let lp = ListParams::default();
                let pods = pods.list(&lp).await?;
                let mut options_to_render = vec![];
                for p in &pods {
                    options_to_render.push(p.name_unchecked())
                }
                let ans = Select::new("Pick a Pod!", options_to_render).prompt();

                match ans {
                    Ok(choice) => pod_actions_handler(choice),
                    Err(_) => Ok(()),
                };
            }
            _ => println!("Not implemented!"),
        },
        _ => println!("Not implemented!"),
    }

    Ok(())
}
