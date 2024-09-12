use tokio::{io::Error, process::Command};

use crate::cmd::command::PM;

#[derive(Debug, Clone)]
pub struct PipPkg {
    pub name: String,
    pub version: String,
}

pub async fn get_installed_pkgs_info(package_manager: &PM) -> Result<Vec<PipPkg>, Error> {
    let output = match package_manager {
        PM::UV => Command::new("uv").args(["pip", "freeze"]).output().await?,
        PM::Pip => Command::new("pip").args(["freeze"]).output().await?,
    };
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut packages: Vec<&str> = stdout.split('\n').collect();

    // This is intentional because there is an empty line
    packages.pop();

    let pip_pkgs: Vec<PipPkg> = packages
        .iter()
        .map(|pkg| {
            let pkg_info: Vec<&str> = pkg.split("==").collect();
            let name = pkg_info.first().unwrap_or(&"");
            let version = pkg_info.get(1).unwrap_or(&"");

            PipPkg {
                name: name.to_string(),
                version: version.to_string(),
            }
        })
        .collect();

    Ok(pip_pkgs)
}

pub async fn compare_versions(v1: &str, v2: &str) -> i32 {
    let v1_splitted: Vec<i32> = v1.split(".").map(|v| v.parse::<i32>().unwrap()).collect();
    let v2_splitted: Vec<i32> = v2.split(".").map(|v| v.parse::<i32>().unwrap()).collect();

    for (version1, version2) in v1_splitted.iter().zip(v2_splitted.iter()) {
        if version1 > version2 {
            return -1;
        } else if version2 > version1 {
            return 1;
        }
    }

    0
}
