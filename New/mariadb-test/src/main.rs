use std::env;
use mysql::*;
use mysql::prelude::*;

const PROGRAM_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Basic test for connecting to a Mysql/MariaDB serverin rust.
// TODO Fix this to work later.
// Currently I get this error
// Error: DriverError { Could not connect to address `localhost:3306': Connection refused (os error 111) }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Display the program info message on startup.
    println!("--------------------");
    println!("Running {} v{}", PROGRAM_NAME, VERSION);
    println!("--------------------\n");


    // Load the .env file for later use
    // There is a .env.example file in here which can be renamed to .env and used.
    // https://env.dev/guides/rust-env-variables
    dotenvy::dotenv().ok();

    let db_username = env::var("DB_USERNAME").unwrap_or_default();
    let db_password = env::var("DB_PASSWORD").unwrap_or_default();
    let db_host = env::var("DB_HOST").unwrap_or_default();
    let db_name = env::var("DB_NAME").unwrap_or_default();
    let db_port = env::var("DB_PORT").unwrap_or_default();

    // println!("DB username: {}", db_username);
    // println!("DB password: {}", db_password);
    // println!("DB host: {}", db_host);
    // println!("DB name: {}", db_name);
    // println!("DB port: {}", db_port);

    // let url = "mysql://app:secret@localhost:3306/mydb";
    // let url = format!("mysql://{db_username}:${db_password}@{db_host}:{db_port}/{db_name}");
    let url = format!("mysql://{db_username}:${db_password}@{db_host}:{db_port}/{db_name}");
    let pool = Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;

    let version: String = conn.query_first("SELECT VERSION()")?.unwrap();
    println!("{version}"); // e.g. 11.8.3-MariaDB
    Ok(())
}