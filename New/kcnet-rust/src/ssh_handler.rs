// https://www.slingacademy.com/article/handling-platform-specific-code-with-cfg-attributes-in-rust-modules/

use std::error::Error;
use openssh::{KnownHosts, Session};

/// List of commands to use for the SSH test.
/// For now this just has the ls command for testing.
pub enum LinuxCommand {
    // List files
    LS = 0,

}

#[cfg(target_os = "windows")]
pub async fn ssh_test(_username: &str, _host_name: &str) -> Result<(), Box<dyn Error>> {
    println!("ERROR: This only works on Linux based systems!");

    Ok(())
}

#[cfg(target_os = "linux")]
/// SSH testing, this works for connecting to a remote SSH host with SSH keys.
pub async fn ssh_test(username: &str, host_name: &str, command: LinuxCommand) -> Result<(), Box<dyn Error>> {
    let session = Session::connect_mux(format!("{}@{}", username, host_name), KnownHosts::Strict).await?;

    match command {
        LinuxCommand::LS => {
            println!("-----------------");
            println!("Listing files on server {}\n", host_name);
            let ls = session.command("ls").output().await?;
            eprintln!("{}", String::from_utf8(ls.stdout).expect("server output was not valid UTF-8"));
            println!("-----------------");
        }
    }

    // let whoami = session.command("whoami").output().await?;
    // assert_eq!(whoami.stdout, b"\n");
    session.close().await?;

    Ok(())
}
