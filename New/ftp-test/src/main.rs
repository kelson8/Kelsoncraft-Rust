use std::{env, fs};
use libunftp::ServerBuilder;
use unftp_sbe_fs::Filesystem;

// use std::path::Path;

// This is a very basic FTP client written in Rust.
// Most of this came from this GitHub repo
// https://github.com/bolcom/libunftp/tree/master/crates/unftp-sbe-fs

// TODO Add user IP logging to this later, I guess it doesn't do that by default.

// TODO Move these into a .env later.
const SERVER_IP: &str = "192.168.1.108";
const SERVER_PORT: u16 = 2121;
// // TODO Fix this to work.
// // const SERVER_PORT_RANGE: RangeInclusive<u16> = 50000..50010;
// // const SERVER_PORT_RANGE: ops::Range<u16> = 50000..50010;
//
const FTP_GREETING: &str = "Welcome to the KCNet Rust FTP Server";
//
//
const FTP_FOLDER: &'static str = "/media/kelson/Game Drive/ftp_server";
// This can be used for a temporary folder.
// const FTP_FOLDER: &'static str = "/tmp/rust_ftp";

// TODO Move into .env before setting these up.
// const FTP_USERNAME: &str = "root";
// const FTP_PASSWORD: &str = "password";

/// Check if a path exists
///
/// https://stackoverflow.com/questions/32384594/how-to-check-whether-a-path-exists
pub fn path_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}


#[tokio::main]
pub async fn main() -> std::io::Result<()> {

    // Load the .env file for later use
    // There is a .env.example file in here which can be renamed to .env and used.
    // https://env.dev/guides/rust-env-variables
    dotenvy::dotenv().ok();

    // let server_ip = env::var("SERVER_IP").expect("Missing SERVER_IP environment variable");
    // let server_port = env::var("SERVER_PORT").expect("Missing SERVER_PORT environment variable");
    // let ftp_greeting = env::var("FTP_GREETING").expect("Missing FTP_GREETING environment variable");
    // let ftp_folder = env::var("FTP_FOLDER").expect("Missing FTP_FOLDER environment variable");

    // let ftp_folder = Path::new("/media/kelson/Game Drive/ftp_server");
    // let ftp_home = std::env::temp_dir();

    // Create the FTP folder if it doesn't exist
    // This places the FTP server files in the home folder of the FTP_FOLDER path.
    if !path_exists(&(FTP_FOLDER.to_owned() + "/home")) {
        // fs::create_dir_all(&ftp_folder)?;
        fs::create_dir_all(&(FTP_FOLDER.to_owned() + "/home"))?;
    } else {
        println!("FTP folder already exists, using existing folder for it.");
    }
    
    // let ftp_home_fs =  Filesystem::new(ftp_folder + "/home");
    let ftp_home_fs =  Filesystem::new(FTP_FOLDER.to_string() + "/home");


    // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // let server = ServerBuilder::new(Box::new(move || Filesystem::new(ftp_home.clone()).unwrap()))
    let server = ServerBuilder::new(Box::new(move || Filesystem::new(FTP_FOLDER).unwrap()))
        .greeting(FTP_GREETING)
        .passive_ports(50000..=50010)
        // .passive_ports(SERVER_PORT_RANGE)
        .build()
        .unwrap();

    println!("{}:{} FTP Server starting in directory: {:?}", SERVER_IP, SERVER_PORT, &ftp_home_fs);
    server.listen(format!("{}:{}", SERVER_IP, SERVER_PORT)).await.expect("FTP Server failed to start");

    Ok(())
}
