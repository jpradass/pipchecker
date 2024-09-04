use std::io;

mod api;
mod cmd;
mod log;
mod menu;
mod pip;
mod read_req;

#[tokio::main]
async fn main() -> io::Result<()> {
    match read_req::exists_requirements()? {
        Some(_) => {
            // println!("Found at {:?}", file.path());
            // let pkgs: Vec<String> =
            //     read_req::read_local_requirements(file.path().to_str().unwrap()).await;
            // println!("{:?}", pkgs);
            // let pypi = api::pypi::PypiApi::new();
            // match pypi.get_pkg_info("requests").await {
            //     Some(pkg) => {
            //         // println!("{:?}", pkg.as_object().unwrap())
            //         menu::display::display_pkg_info(pkg);
            //     }
            //     None => exit(1),
            // };
            let cmmd = cmd::command::Cmd::new();
            cmmd.execute().await;
        }
        None => println!("File not found"),
    }
    Ok(())
}
