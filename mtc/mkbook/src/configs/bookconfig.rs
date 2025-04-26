use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::mkbook::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookConfig {
    pub book: BookSection,
    pub rust: RustSection,
    pub output: OutputSection,
    pub build: BuildSection,
    pub preprocessor: PreprocessorSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookSection {
    pub authors: Option<Vec<String>>,
    pub language: String,
    pub src: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RustSection {
    pub edition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputSection {
    pub html: HtmlSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlSection {
    pub git_repository_url: Option<String>,
    pub edit_url_template: Option<String>,
    pub additional_js: Vec<String>,
    pub additional_css: Vec<String>,
    pub search: HtmlSearchSection,
    pub playground: HtmlPlaygroundSection,
    pub fold: HtmlFoldSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlSearchSection {
    pub limit_results: u32,
    pub use_boolean_and: bool,
    pub boost_title: u32,
    pub boost_hierarchy: u32,
    pub boost_paragraph: u32,
    pub expand: bool,
    pub heading_split_level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlPlaygroundSection {
    pub editable: bool,
    pub copyable: bool,
    pub copy_js: bool,
    pub line_numbers: bool,
    pub runnable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlFoldSection {
    pub enable: bool,
    pub level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSection {
    pub create_missing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreprocessorSection {
    pub callouts: PreprocessorCallouts,
    pub autosummary: Option<()>,
    pub protobuf: PreprocessorProtobuf,
    pub kroki_preprocessor: PreprocessorKroki,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreprocessorCallouts {
    pub after: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreprocessorProtobuf {
    pub after: Vec<String>,
    pub command: String,
    pub proto_descriptor: String,
    pub nest_under: String,
    pub proto_url_root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreprocessorKroki {
    pub after: Vec<String>,
}

impl From<Config> for BookConfig {
    fn from(config: Config) -> Self {
        BookConfig {
            book: BookSection {
                authors: config.authors,
                language: "en".to_string(),
                src: "src".to_string(),
                title: config.title,
            },
            rust: RustSection {
                edition: config.rust_edition,
            },
            output: OutputSection {
                html: HtmlSection {
                    git_repository_url: config.repository_url,
                    edit_url_template: config.repository_edit_url,
                    additional_js: vec![],
                    additional_css: vec!["mdbook-protobuf.css".to_string()],
                    search: HtmlSearchSection {
                        limit_results: 20,
                        use_boolean_and: true,
                        boost_title: 2,
                        boost_hierarchy: 2,
                        boost_paragraph: 1,
                        expand: true,
                        heading_split_level: 2,
                    },
                    playground: HtmlPlaygroundSection {
                        editable: true,
                        copyable: true,
                        copy_js: true,
                        line_numbers: true,
                        runnable: true,
                    },
                    fold: HtmlFoldSection {
                        enable: true,
                        level: 0,
                    },
                },
            },
            build: BuildSection {
                create_missing: false,
            },
            preprocessor: PreprocessorSection {
                callouts: PreprocessorCallouts {
                    after: vec!["autosummary".to_string()],
                },
                autosummary: None,
                protobuf: PreprocessorProtobuf {
                    after: vec!["autosummary".to_string()],
                    command: "mdbook-protobuf".to_string(),
                    proto_descriptor: "./build/proto_file_descriptor_set.pb".to_string(),
                    nest_under: "Protocol".to_string(),
                    proto_url_root: "https://github.com/zakhenry/mdbook-protobuf/tree/master/demo/proto/".to_string(),
                },
                kroki_preprocessor: PreprocessorKroki {
                    after: vec!["autosummary".to_string()],
                },
            },
        }
    }
}
impl BookConfig{


pub fn to_toml(&self) -> String {
    toml::to_string(self).expect("Failed to serialize configuration")
}
}