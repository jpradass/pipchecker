use std::time::{Duration, Instant};

use ansi_term::Colour;
use clap::{arg, Command};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{sync::mpsc, task, time::sleep};

use crate::{
    api::pypi,
    info,
    menu::display,
    pip::{
        self,
        functions::{compare_versions, PipPkg},
    },
};

#[derive(Debug)]
pub enum PM {
    UV,
    Pip,
}

#[derive(Debug)]
pub struct Cmd {
    root_cmd: Command,
    package_manager: PM,
}

impl Cmd {
    pub fn new(pm: PM) -> Cmd {
        Cmd {
            root_cmd: Command::new("pipchecker")
                .about("Pip checker tool")
                .subcommand_required(false)
                .arg_required_else_help(true)
                .subcommand(
                    clap::Command::new("inspect")
                        .about("Get info of some package in Pypi")
                        .arg(arg!(<PKG_NAME> "package name"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    clap::Command::new("check")
                        .about("Check for new version of installed packages")
                        .arg_required_else_help(false),
                )
                .subcommand(
                    clap::Command::new("update")
                        .about("Updates local packages for new ones")
                        .arg_required_else_help(false),
                ),
            package_manager: pm,
        }
    }

    pub async fn execute(&self) {
        match self.root_cmd.clone().get_matches().subcommand() {
            Some(("inspect", sub_matches)) => {
                let pkg_name = sub_matches.get_one::<String>("PKG_NAME").expect("required");
                // println!("Getting info of {}...", pkg_name);
                // info!(format!("Getting info of {}...", pkg_name));
                if let Some(pkg_info) = pypi::get_pkg_info(pkg_name).await {
                    display::display_pkg_info(pkg_info);
                }
            }
            Some(("check", _sub_matches)) => {
                // info!("Checking version of packages...");
                if let Ok(pip_pkgs) =
                    pip::functions::get_installed_pkgs_info(&self.package_manager).await
                {
                    let start_time = Instant::now();
                    let spinner = ProgressBar::new_spinner();
                    spinner.set_message("Checking packages...");
                    spinner.set_style(
                        ProgressStyle::with_template("{spinner:.grey} {msg}")
                            .unwrap()
                            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
                    );

                    let (tx, mut rx) = mpsc::channel(64);

                    let mut handles = vec![];
                    for pkg in pip_pkgs {
                        let tx = tx.clone();

                        let handle = task::spawn(async move {
                            if let Some(pkg_info) = pypi::get_pkg_info(pkg.name.as_str()).await {
                                let version = pkg_info["version"].as_str().unwrap_or("");
                                if compare_versions(pkg.version.as_str(), version).await == 1 {
                                    // println!(" {green_cross} {name}: {} <{version}", pkg.version);
                                    let data_to_send =
                                        send_pkg_data(pkg, version.to_string()).await;
                                    let _ = tx.send(data_to_send).await;
                                }
                            }
                        });
                        handles.push(handle);
                    }

                    let spinner_clone = spinner.clone();
                    let spinner_handler = tokio::spawn(async move {
                        loop {
                            spinner.tick();
                            sleep(Duration::from_millis(100)).await;
                        }
                    });

                    drop(tx);
                    for handle in handles {
                        handle.await.unwrap();
                    }

                    let duration = start_time.elapsed();
                    let duration_millis = format!("{}ms", duration.as_millis());

                    spinner_handler.abort();
                    spinner_clone.finish_with_message(format!(
                        "Checks took {}",
                        Colour::RGB(169, 169, 169).paint(duration_millis)
                    ));

                    while let Some(tuple) = rx.recv().await {
                        let green_cross = "+".green();
                        println!(" {green_cross} {}: {} <{}", tuple.0, tuple.1, tuple.2);
                    }
                }
            }
            Some(("update", _sub_matches)) => {}
            _ => unreachable!(),
        }
    }
}

async fn send_pkg_data(pkg: PipPkg, pip_version: String) -> (String, String, String) {
    (pkg.name, pkg.version, pip_version)
}
