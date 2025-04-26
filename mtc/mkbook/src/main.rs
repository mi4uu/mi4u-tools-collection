//! This a dev cli, see `HELP` strings for details.
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use anyhow::{Context, Result as AnyResult};
use clap::{Arg, Command};
use duct::cmd;
use std::env;


pub fn main_with_args(args: &[String]) -> AnyResult<()> {
   

    let cli = Command::new("mkbook")
        .subcommand(
            Command::new("coverage").arg(
                Arg::new("dev")
                    .short('d')
                    .long("dev")
                    .help("generate an html report"),
            ),
        )
        .subcommand(Command::new("install").about("install tools needed").alias("i"))
        .subcommand(Command::new("vars"))
        .subcommand(Command::new("generate").alias("gen"))
        .subcommand(Command::new("ci"));

    let matches = cli.get_matches_from(args);
    println!("Received subcommand: {:?}", matches.subcommand());

    let res = match matches.subcommand() {
        Some(("vars", _)) => {
            let envs=mtc_make_a_book::tasks::ci::env();
            let root = mtc_toolbelt::env_utils::get_workspace_root();
            println!("workspace root: {root:?}");
            let root = mtc_toolbelt::get_package_root!();
            println!("package root: {root:?}");
           envs
        }
        Some(("install",_))=>{
            install()
        }
        Some(("generate",_))=>{
            mtc_make_a_book::tasks::book::make_book()
        }
        Some(("ci", _)) | None => mtc_make_a_book::tasks::ci::ci(),
        _ => {
            eprintln!("Error: Unrecognized subcommand");
            Err(anyhow::Error::msg("Unrecognized subcommand"))
        }
    };

    res
}
/// The main entry point of the application.
///
/// This function collects command-line arguments and passes them to `main_with_args` for
/// further processing and execution of the appropriate subcommands.
///
/// # Returns
///
/// * `AnyResult<()>`: An `Ok(())` variant if the application runs successfully,
///   or an `Err` variant encapsulating any error that occurs during execution.
///
/// # Errors
///
/// This function will propagate any errors returned by `main_with_args`.
pub fn main() -> AnyResult<()> {
    tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_default_env())
    .init();

    let args: Vec<String> = env::args().collect();
    main_with_args(&args)
}

pub fn binstall(name:&str) -> AnyResult<()> {

    cmd!("cargo", "binstall", "-y" , name).run()?;

    Ok(())
}
pub fn install() -> AnyResult<()> {
    cmd!("cargo", "install", "cargo-binstall").run()?;

    binstall("cargo-watch")?;
    binstall("cargo-hack")?;
    binstall("mdbook")?;
    binstall("mdbook-callouts")?;
    binstall("mdbook-kroki-preprocessor")?;
    binstall("mdbook-protobuf")?;
    binstall("rustdoc-md")?;
    binstall("cargo-modules")?;

    // cmd!("cargo", "install", "cargo-bloat").run()?;
    // cmd!("rustup", "component", "add", "llvm-tools-preview").run()?;
    // cmd!("cargo", "install", "grcov").run()?;
    Ok(())
}