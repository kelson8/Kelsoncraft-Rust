use std::error::Error;
use std::path::Path;
use clap::{Command, arg, Parser, ArgMatches};

// For colors and other stuff

use ansi_term::Colour;
// use git2::ErrorClass::Repository;
use kcnet_lib::{json_handler, log_handler::logger, json_handler::Vec2D, json_handler::Vec3D, os_util};

// Requires libgit2-dev and libgit2-1.7 on Debian.
// https://github.com/ropensci/git2r/issues/410
use git2::Repository;

use serde::Deserialize;

// https://docs.rs/clap/latest/clap/

// https://oneuptime.com/blog/post/2026-02-03-rust-clap-cli-applications/view
// https://dev.to/moseeh_52/getting-started-with-clap-a-beginners-guide-to-rust-cli-apps-1n3f


// For CLI arguments.

// TODO Look into this later
// https://crates.io/crates/clap-verbosity-flag

// Rust CLI guides
// https://rust-cli.github.io/book/tutorial/setup.html

const PROGRAM_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// ReVC Git variables

const REVC_GIT_URL: &'static str = "https://git.kelsoncraft.net/kelson8/reVC";

/// The repo git url for the ReVC project.
const REVC_GIT_REPO: &'static str = "https://git.kelsoncraft.net/kelson8/reVC.git";
// const REVC_CURRENT_VERSION: &'static str = "";
/// The latest SHA hash for the ReVC git commit, for now it's hardcoded until I fix this.
const REVC_LATEST_COMMIT_SHA1: &'static str = "eb8a2a67351f31c159e8cef83c7e76d1607329b7";
/// The build status of the ReVC project.
const REVC_BUILD_STATUS: &'static str = "";

// Not implemented yet.
// const REVC_RELEASE_BUILD: &'static str = "";

#[derive(Debug)]
/// The status of the project, if it has been built.
enum BuildStatus {
    NotBuilt = 0,
    Built = 1
}

/// The status of the SHA checksum, if it has been verififed against my latest build SHA256.
enum ReVcChecksumStatus {
    HashFail = 0,
    HashSuccess = 1
}

/// The ReVC repo struct for cloning and building.
struct ReVcRepo {
    /// The URL to this ReVC project.
    url: String,
    /// The repository details.
    repo: Repository,
    /// The version of this ReVC project.
    version: String,
    /// The latest Git commit for this repo.
    latest_commit_hash: String,
    /// The build status of the project workflow runs.
    build_status: BuildStatus,

    /// The commit message.
    commit_message: String,
    /// The hash for the current built project, I will need to set this for Windows and Linux
    current_build_hash: String,
    // The current release build, not setup yet.
    // release_build: String,
}

/// Simple program to test projects in this repo.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The user to get the home directory for.
    #[arg(short, long)]
    user_home: String,

    /// The repo to clone for ReVC
    #[arg(short, long)]
    revc_git_repo: String,

    // Number for the arguments.
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

/// For testing colors in the console
///
/// <https://docs.rs/ansi_term/latest/ansi_term/>
fn color_console_output() {
    let blue_string = Colour::Blue.paint("Test");

    println!("{}", blue_string);
}

/// Cli commands for this project.
///
/// This works! Although it defaults to the clone option instead of a help option for it.
/// I will need to fine tune and fix this later.
///
/// Credit to this GitHub page for this guide: <https://github.com/clap-rs/clap/discussions/5433>
fn cli() -> Command {
    Command::new("kcnet")
        .about("KCNet CLI tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)

        .subcommand(
            Command::new("revc-repo")
                .args_conflicts_with_subcommands(true)
                .flatten_help(true)
                // Clone and build the ReVC repo.
                .subcommand(Command::new("clone"))
                .subcommand(Command::new("build")),
        )

        .args([
            // arg!(-c --config <CONFIG> "Optionally set a config to use"),
            arg!(-d --debug "Turn on debugging information"),
            // arg!(-r --repo-path "Set the repository path to clone to"),
        ])
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-b --book <PLAYBOOK>)]
}

//-------------
// ReVC Repo Testing
//-------------

// Check if the ReVC Git repo exists.
// fn check_revc_repo_exists(repo_path: &str) -> bool {
//     Path::new(repo_path).exists()
// }

//

/// Build the ReVC project.
///
/// I got this to clone the repo.
/// I just have to figure something out for this before it's ready to be used.
///
/// This will display and use values from my ReVC project with the ReVcRepo struct.
///
/// This will run Git clone for the ReVC repo, download premake,
///  generate the premake build files, attempt to build the project and a bit more.
///
///
fn build_revc(repo_path: &str) -> Result<(), git2::Error> {

    let client = reqwest::blocking::Client::new();
    // https://docs.rs/git2/latest/git2/



    // 1. Check if the repo path exists, if not clone it.
    let repo = if Path::new(repo_path).exists() {
        println!("ReVC repo exists, attempting to build it now.");
        Repository::open(repo_path)?

    } else {
        // println!("Cloning ReVC...");
        println!("ReVC repo doesn't exist, cloning it for you now.");
        Repository::clone(REVC_GIT_REPO, repo_path)?
    };

    // View the details of the Git repo.
    let latest_commit_hash = repo
        .head()?
        .target()
        .ok_or_else(|| git2::Error::from_str("HEAD does not point to a commit"))?
        .to_string();

    let sha = repo.head()?.peel_to_commit()?.id().to_string();

    let commit_message = repo
        .head()?
        .peel_to_commit()?
        .message()
        .unwrap_or("")
        .to_owned();

    let revc_repo = ReVcRepo {
        url: REVC_GIT_REPO.to_owned(),
        // version: repo
        version: "".to_string(),
        latest_commit_hash,
        build_status: BuildStatus::NotBuilt,
        commit_message,
        // TODO Make this be obtained from the executable once built.
        // Also, verify it against a Windows or Linux SHA256 for the executable depending on the platform.
        current_build_hash: "".to_string(),
        repo,
    };

    let repo_status = format!(r"ReVC Repo status:
            URL: {}
            Build status: {:?}
            Commit message: {}
            Commit Hash: {}",
                              revc_repo.url,
                              revc_repo.build_status,
                              revc_repo.commit_message,
                              revc_repo.latest_commit_hash);

    // TODO Try to figure this out for running commands, so I can build ReVC with Rust.
    // let mut command = std::process::Command::new("ls")
    // std::process::Command::new("ls")
    //     .args(["ls", " -lah"])
    //     .output()
    //     .expect("failed to execute process");
    // println!("{}", command.);

    println!("{}", repo_status);



    //

    Ok(())
    // }


    // 2. Make sure premake is installed somewhere, if not require it to be downloaded
    // https://premake.github.io/download/

    // 3. Generate the makefile if on Linux, and build the project with make.

    // 4. Optionally, verify the SHA256 Checksum against my latest build of the commit SHA that I use for the builds.

    // 5. Copy the executable, Lua scripts and extra game files to Vice City game folder.
}

fn main() {
    // Display the program info message on startup.
    println!("--------------------");
    println!("Running {} v{}", PROGRAM_NAME, VERSION);
    println!("--------------------\n");

    // New
    // https://github.com/clap-rs/clap/discussions/5433
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("revc-repo", sub_matches)) => {
            let revc_command = sub_matches.subcommand().unwrap_or(("clone", sub_matches));
            match revc_command {
                ("clone", sub_matches) => {
                    println!("Cloning ReVC Git repo...");
                }
                ("build", sub_matches) => {
                    println!("Building ReVC Git repo...");

                    if let Err(e) = build_revc("./reVC-test") {
                        eprintln!("ReVC build failed: {}", e);
                        std::process::exit(1);
                    }

                    // build_revc("./reVC-test").expect("Error: ReVC could not be built.");
                }
                // _ => {
                //     println!("Invalid subcommand");
                // }

                (&_, _) => unreachable!(),
            }

        }
        _ => unreachable!(),
    }
    //


    // let args = Args::parse();
    // println!("Hello {}!", args.user_home);

    //--------
    // Check the users home directory
    // https://idiomatic-rust-snippets.org/essentials/std-lib/some.html

    // This works here!
    // Now to figure out how to do this without an argument, make this return the user home directly.

    // This might be useful for something
    // https://docs.rs/clap/latest/clap/struct.ArgMatches.html

    // let user_home_dir = os_util::get_home_dir(&args.user_home);
    // println!("Home user: {}", user_home_dir.unwrap().display());
    // println!("Home user: {:#?}", user_home_dir);


    // Output with colors
    // color_console_output();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.user_home);

    // }
}