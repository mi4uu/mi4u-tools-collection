use anyhow::{anyhow,Result};
use better_default::Default;
use duct::cmd;
use mtc_config::{Configuration, Deserialize, Serialize};
use mtc_toolbelt::get_git_default_branch_name;
use tracing::info;

use crate::configs::bookconfig::BookConfig;

// fn map_env_to_vec(v:)

#[derive(Debug,Default,Configuration,Serialize,Deserialize)]
#[config(name="mkbook", format="toml")]
pub struct Config{
    #[default(String::from("book"))]
    pub dir:String,
    #[default(std::option_env!("CARGO_PKG_AUTHORS").map(|s| vec![s.to_string()]))]
    pub authors:Option<Vec<String>>,
    #[default(env!("CARGO_PKG_NAME").into())]
    pub title:String,
    #[default(option_env!("CARGO_PKG_HOMEPAGE").map(|s|s.to_string()))]
    pub homepage:Option<String>,
    #[default(option_env!("CARGO_PKG_REPOSITORY").map(|s|format!("{}/tree/{}",s,get_git_default_branch_name!())))]
    pub repository_url:Option<String>,
    #[default(option_env!("CARGO_PKG_REPOSITORY").map(|s|format!("{}/edit/{}/{{path}}",s.to_string(),get_git_default_branch_name!() ) ))]
    pub repository_edit_url:Option<String>,
    #[default(None)]
    pub rust_edition:Option<String>,
    #[default(vec![])]
    pub copy_extra_dirs:Vec<String>,
    #[default(vec![])]
    pub disable_packages:Vec<String>,
    #[default(vec![])]
    pub enable_packages:Vec<String>,

}