use std::{fs, io};

pub async fn read_local_requirements(path: &str) -> Vec<String> {
    let req_file = tokio::fs::read_to_string(path).await.unwrap();
    let packages = req_file.lines().map(str::to_string).collect();
    packages
}

pub fn exists_requirements() -> io::Result<Option<fs::DirEntry>> {
    let current_dir = std::env::current_dir()?;
    let directory = fs::read_dir(current_dir.as_path())?;

    let requirements = directory
        .filter_map(Result::ok)
        .find(|entry| entry.file_name() == "requirements.txt");

    Ok(requirements)
}
