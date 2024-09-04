use std::error;

use tokio::{io::Error, process::Command};

#[derive(Debug)]
pub struct PipPkg {
    name: String,
    version: String,
}

pub async fn get_installed_pkgs_info() -> Result<Vec<PipPkg>, Error> {
    let output = Command::new("uv").args(["pip", "freeze"]).output().await?;
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
