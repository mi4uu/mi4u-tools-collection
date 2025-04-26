use std::fs::write;

use anyhow::{anyhow,Result};
use better_default::Default;
use duct::cmd;
use mtc_config::{Configuration, Deserialize, Serialize};
use mtc_toolbelt::{env_utils::get_workspace_root, get_git_default_branch_name};
use tracing::info;

use crate::configs::{bookconfig::BookConfig, mkbook::Config};

// fn map_env_to_vec(v:)

 fn get_config()->Config{


    Config::load_or_default()
}

pub fn make_book()->Result<()>{
    let config: Config=get_config();
    info!("config: {:#?}",config);
    let book_path=get_workspace_root().join(&config.dir);
    let book_config=BookConfig::from(config);
    // println!("{}",book_config.to_toml());

    std::fs::create_dir_all(&book_path)?;
    std::fs::create_dir_all(&book_path.join("src"))?;
    std::fs::write(&book_path.join("book.toml"), book_config.to_toml())?;
    std::fs::write(&book_path.join("src/SUMMARY.md"), "# SUMMARY\n\n- [readme](<README.md>)\n")?;
    std::fs::write(&book_path.join("src/README.md"), "# README\n\nhi\n")?;
    std::fs::write(&book_path.join(".gitignore"), "book/\n")?;



    Ok(())
}
