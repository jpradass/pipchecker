use std::io;

mod api;
mod read_req;

#[tokio::main]
async fn main() -> io::Result<()> {
    match read_req::exists_requirements()? {
        Some(file) => {
            println!("Found at {:?}", file.path());
            // let pkgs: Vec<String> =
            //     read_req::read_local_requirements(file.path().to_str().unwrap()).await;
            // println!("{:?}", pkgs);
            let pypi = api::pypi::PypiApi::new();
            match pypi.get_pkg_info("requests").await {
                Some(pkg) => println!("{:?}", pkg.as_object().unwrap()),
                None => println!("Got none xdd"),
            };
        }
        None => println!("File not found"),
    }
    Ok(())
}
