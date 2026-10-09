use std::env;
// use std::time::Duration;
// use mysql::*;
// use mysql::prelude::*;

// Switching to sqlx for testing
// TODO Fix this
// https://dev.to/behainguyen/rust-mysql-connect-execute-sql-statements-and-stored-procs-using-crate-sqlx-3djk
// use sqlx::{Pool, MySql, Error, MySqlPool, query_as, query};
use sqlx::{Pool, MySql, Error, MySqlPool};
// use sqlx::mysql::MySqlPoolOptions;

// Another guide for sqlx
// https://sqlx.dev/

const PROGRAM_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Basic test for connecting to a Mysql/MariaDB server in rust.

use async_std::task;
// use sqlx::pool::PoolOptions;
use sqlx::types::chrono;

/// Database connection info
struct SqlConnection {
    /// Username for DB.
    db_username: String,
    /// Password for DB.
    db_password: String,
    /// Hostname for DB.
    db_host: String,
    /// Database name to use
    db_name: String,
    /// Database port to use, I will make this an i32 later once I can with the env.
    db_port: String
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct User {
    id: i32,
    email: String,
    username: String,
    hashed_password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    // created_at: chrono::NaiveDateTime,
}

/// Testing with Mysql/Mariadb
///
/// TODO Try to get this to work
///
/// <https://oneuptime.com/blog/post/2026-03-31-mysql-rust-sqlx-library/view>
/// <https://reintech.io/blog/sqlx-tutorial-type-safe-sql-rust-without-orm>
// async fn sql_test(pool: Pool<MySql>,
//                     email: &str, username: &str, hashed_password: &str) -> Result<User, sqlx::Error> {
//     // sqlx::query!("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, email, username. hashed_password)")
//     //     .execute(&pool)
//     //     .await;
//
//     let user = query_as!(
//         User,
//         r#"
//         INSERT INTO users (id, email, username, hashed_password, created_at)
//         VALUES (1, email, username, hashed_password, "")
// --         VALUES (1, "test@test.com", "test", "", chrono::Utc::now())
//         RETURNING id, email, username, created_at,
//         id,
//         email,
//         username,
//         hashed_password,
//         created_at
//         "#)
//         .fetch_one(&pool)
//         .await?;
//     //     email,
//     //     username
//
//     Ok(user)
//
//     // let users: Vec<User> = sqlx::query_as!(
//     // User,
//     // "SELECT email, username, hashed_password FROM users WHERE email = ?",
//     // chrono::Utc::now() - chrono::Duration::days(7))
//     //
//     //     ;
// }


/// Connect to a sql server with Mysql/Mariadb using sqlx
///
/// This seems to work now, this uses my custom SqlConnection struct which contains the database info.
///
async fn connect(connection: SqlConnection) -> Result<Pool<MySql>, Error> {
// async fn connect(connection: SqlConnection) -> Pool<MySql> {
    let db_url = format!("mysql://{}:{}@{}:{}/{}?prefer_socket=false",
                                                           connection.db_username,
                                                           connection.db_password,
                                                           connection.db_host,
                                                           connection.db_port,
                                                           connection.db_name);

    MySqlPool::connect(db_url.as_str()).await

    // https://oneuptime.com/blog/post/2026-03-31-mysql-rust-sqlx-library/view
    // MySqlPoolOptions::new()
    //     .max_connections(20)
    //     .min_connections(1)
    //     .acquire_timeout(Duration::from_secs(5))
    //     .idle_timeout(Duration::from_secs(600))
    //     .connect(db_url.as_str())
    //     .await.expect("Failed to connect to DB")
}

/// Test a connection to the Mysql/Mariadb database
///
/// Taken from this medium article
/// * <https://dev.to/behainguyen/rust-mysql-connect-execute-sql-statements-and-stored-procs-using-crate-sqlx-3djk>
///
async fn do_test_connection(connection: SqlConnection) {
    let result = task::block_on(connect(connection));

    match result {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }

        Ok(_) => {
            println!("Connected to database successfully.");
        }
    }
}

// async fn get_all_users()

// Changed to async
// https://sealos.io/docs/guides/databases/mysql/rust/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Display the program info message on startup.
    println!("--------------------");
    println!("Running {} v{}", PROGRAM_NAME, VERSION);
    println!("--------------------\n");


    // Load the .env file for later use
    // There is a .env.example file in here which can be renamed to .env and used.
    // https://env.dev/guides/rust-env-variables
    dotenvy::dotenv().ok();

    // Get the database values from the .env.
    // DO NOT hardcode these in here!
    let db_username = env::var("DB_USERNAME").unwrap_or_default();
    let db_password = env::var("DB_PASSWORD").unwrap_or_default();
    let db_host = env::var("DB_HOST").unwrap_or_default();
    let db_name = env::var("DB_NAME").unwrap_or_default();
    let db_port = env::var("DB_PORT").unwrap_or_default();

    // Setup the SqlConnection struct with the .env values.
    let sql_connection = SqlConnection {
        db_username,
        db_password,
        db_host,
        db_port,
        db_name
    };

    task::block_on(do_test_connection(sql_connection));

    // let url = "mysql://app:secret@localhost:3306/mydb";
    // let url = format!("mysql://{db_username}:${db_password}@{db_host}:{db_port}/{db_name}");
    // let url = format!("mysql://{}:{}@{}:{}/{}?prefer_socket=false", db_username, db_password, db_host, db_port, db_name);
    // let pool = Pool::new(url.as_str())?;
    // let mut conn = pool.get_conn()?;

    // let connection_opts = mysql::Opts::from_url("mysql://root:password@localhost:3307/mysql?prefer_socket=false")?;
    // let connection_opts = mysql::Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}?prefer_socket=false", db_username, db_password, db_host, db_port, db_name))?;
    // let pool = mysql::Pool::new(connection_opts).await?;

    // You can create new default builder
    // let mut builder = OptsBuilder::new();
    // builder = builder.ip_or_hostname(Some("foo"))
    //     .db_name(Some(db_name.as_str())
    //              .db_password(Some(db_password.as_str()))
        // );



    // let version: String = conn.query_first("SELECT VERSION()")?.unwrap();
    // println!("{version}"); // e.g. 11.8.3-MariaDB
    Ok(())
}