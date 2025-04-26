//! This a dev cli, see `HELP` strings for details.

use anyhow::Result;
use duct::cmd;
use std::env;
pub fn ci()->Result<()>{
    println!("ci");
    Ok(())
}
pub fn env()->Result<()>{
    let r=cmd!("env").stdout_capture().run()?;
    tracing::info!("env:{:#?}",&r);
    let s = match str::from_utf8(&r.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    for l in s.to_string().split("\n").into_iter(){

        eprintln!("env:{}",l);
    }
    Ok(())
}