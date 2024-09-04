use clap::{arg, Command};
use colored::*;

use crate::{api::pypi, info, menu::display, pip};

#[derive(Debug)]
pub struct Cmd {
    root_cmd: Command,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            root_cmd: Command::new("pipchecker")
                .about("Pip checker tool")
                .subcommand_required(false)
                .arg_required_else_help(true)
                .subcommand(
                    clap::Command::new("info")
                        .about("Get info of some package in Pypi")
                        .arg(arg!(<PKG_NAME> "package name"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    clap::Command::new("check")
                        .about("Check for new version of installed packages")
                        .arg_required_else_help(false),
                ),
        }
    }

    pub async fn execute(&self) {
        match self.root_cmd.clone().get_matches().subcommand() {
            Some(("info", sub_matches)) => {
                let pkg_name = sub_matches.get_one::<String>("PKG_NAME").expect("required");
                // println!("Getting info of {}...", pkg_name);
                info!(format!("Getting info of {}...", pkg_name));
                if let Some(pkg_info) = pypi::get_pkg_info(pkg_name).await {
                    display::display_pkg_info(pkg_info);
                }
            }
            Some(("check", sub_matches)) => {
                info!("Checking version of packages...");
                if let Ok(pip_pkgs) = pip::functions::get_installed_pkgs_info().await {
                    println!("{pip_pkgs:?}");
                }
            }
            _ => unreachable!(),
        }
    }
}
