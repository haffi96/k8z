// use clap::Command;
use clap::Parser;
use core::fmt;
use duct::cmd;
use inquire::Select;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use std::{
    io::{self, Write},
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
                    Ok(choice) => {
                        let pod_actions = vec![
                            PodActions::Describe,
                            PodActions::Exec,
                            PodActions::PortForward,
                            PodActions::Delete,
                            PodActions::ClipboardCopy,
                        ];
                        let actions =
                            Select::new("Pick an action to perform!", pod_actions).prompt();

                        match actions {
                            Ok(PodActions::Describe) => {
                                let cmd_args = vec!["-n", "default", "describe", "pod", &choice];
                                let out = cmd("kubectl", cmd_args).stderr_null().read();
                                match out {
                                    Ok(res) => println!("{}", res),
                                    Err(e) => println!("feck"),
                                }
                            }
                            Ok(PodActions::Exec) => {
                                // FIXME: This is facked
                                let cmd_args = vec![
                                    "-n",
                                    "default",
                                    "exec",
                                    &choice,
                                    "-t",
                                    "--tty",
                                    "--stdin",
                                    "--",
                                    "/bin/bash",
                                ];

                                let mut child = Command::new("kubectl")
                                    .args(cmd_args)
                                    .stdin(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .stderr(Stdio::piped())
                                    .spawn()
                                    .expect("Failed to spawn child process");

                                // .expect("sh command failed to start");
                                // let out = cmd("kubectl", cmd_args)
                                //     .stderr_capture()
                                //     .stdout_capture()
                                //     .run()
                                //     .unwrap();
                                // match out {
                                //     Ok(res) => println!("{}", res),
                                //     Err(e) => println!("feck"),
                                // }
                            }
                            Ok(PodActions::PortForward) => {
                                info!("Selected PortForward to {}", choice)
                            }
                            Ok(PodActions::Delete) => info!("Selected Delete for {}", choice),
                            Ok(PodActions::ClipboardCopy) => {
                                info!("Selected ClipboardCopy for {}", choice)
                            }
                            Err(_) => info!("err fuck"),
                        }
                    }
                    Err(_) => println!("There was an error, try again!"),
                }
            }
            _ => println!("Not implemented!"),
        },
        _ => println!("Not implemented!"),
    }

    // Create Pod blog
    // info!("Creating Pod instance blog");
    // let p: Pod = serde_json::from_value(json!({
    //     "apiVersion": "v1",
    //     "kind": "Pod",
    //     "metadata": { "name": "blog2" },
    //     "spec": {
    //         "containers": [{
    //           "name": "blog",
    //           "image": "clux/blog:0.1.0"
    //         }],
    //     }
    // }))?;

    // let pp = PostParams::default();
    // match pods.create(&pp, &p).await {
    //     Ok(o) => {
    //         let name = o.name_any();
    //         assert_eq!(p.name_any(), name);
    //         info!("Created {}", name);
    //     }
    //     Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
    //     Err(e) => return Err(e.into()),                        // any other case is probably bad
    // }

    // // Watch it phase for a few seconds
    // let establish = await_condition(pods.clone(), "blog", is_pod_running());
    // let _ = tokio::time::timeout(std::time::Duration::from_secs(15), establish).await?;

    // // Replace its spec
    // info!("Patch Pod blog");
    // let patch = json!({
    //     "metadata": {
    //         "resourceVersion": p1cpy.resource_version(),
    //     },
    //     "spec": {
    //         "activeDeadlineSeconds": 5
    //     }
    // });
    // let patchparams = PatchParams::default();
    // let p_patched = pods
    //     .patch("blog", &patchparams, &Patch::Merge(&patch))
    //     .await?;
    // assert_eq!(p_patched.spec.unwrap().active_deadline_seconds, Some(5));

    // let lp = ListParams::default().fields(&format!("metadata.name={}", "blog")); // only want results for our pod
    // for p in pods.list(&lp).await? {
    //     info!("Found Pod: {}", p.name_any());
    // }

    // // Delete it
    // let dp = DeleteParams::default();
    // pods.delete("blog", &dp).await?.map_left(|pdel| {
    //     assert_eq!(pdel.name_any(), "blog");
    //     info!("Deleting blog pod started: {:?}", pdel);
    // });

    Ok(())
}
