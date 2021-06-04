#![forbid(unsafe_code)]
#![deny(warnings)]
use clap::{AppSettings, Clap};
use comrak::ComrakOptions;
use http::Uri;
use std::path::PathBuf;
use async_fs::File;
use futures_lite::io::AsyncWriteExt;

pub mod licenses;

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Ferdinand Linnenberg <ferdinand@linnenberg.dev>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    path: PathBuf,
    outpath: PathBuf,
}

fn license(
    name: &str,
    version: &str,
    license_text: &str,
    authors: Option<String>,
    repository: Option<String>,
) -> Vec<String> {
    let mut h1 = match repository {
        None => {
            format!("# {} ({})", name, version)
        }
        Some(repository) => {
            format!("# [{}]({}) ({})", name, repository, version)
        }
    };
    match authors {
        None => {}
        Some(author) => h1 = format!("{} by {}", h1, author),
    }
    vec![h1, "\n\n".to_string(),license_text.to_string()]
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();

    let metadata_command = cargo_metadata::MetadataCommand::new()
        .manifest_path(opts.path)
        .clone();
    let deps = match cargo_license::get_dependencies_from_cargo_lock(metadata_command, false, false)
    {
        Ok(v) => v,
        Err(err) => {
            log::error!("{:?}", err);
            return;
        }
    };

    let mut out_markdown = vec![];
    for dependency in deps {
        match dependency.license {
            None => match dependency.license_file {
                None => {
                    log::warn!("{} has no license !", dependency.name);
                }
                Some(lfile) => match &dependency.repository {
                    None => {
                        log::warn!("{} has license file, but no repository!", dependency.name);
                    }
                    Some(repo) => match repo.parse::<Uri>() {
                        Ok(v) => {
                            let uri_path = v.path();
                            let repo_splits = uri_path.split('/').collect::<Vec<&str>>();
                            let repo_author = repo_splits[0];
                            let repo_name = repo_splits[0];

                            let license_url = format!(
                                "https://raw.githubusercontent.com/{}/{}/main/{}",
                                repo_author, repo_name, lfile
                            );

                            match reqwest::get(license_url).await {
                                Ok(response) => match response.text().await {
                                    Ok(rtext) => {
                                        out_markdown.append(&mut license(
                                            &dependency.name,
                                            &dependency.version.to_string(),
                                            &rtext,
                                            dependency.authors,
                                            dependency.repository,
                                        ));
                                    }
                                    Err(e) => {
                                        log::warn!(
                                            "Failed to read license for {}. Error: {:?}",
                                            dependency.name,
                                            e
                                        )
                                    }
                                },
                                Err(e) => {
                                    log::warn!(
                                        "Failed to retrieve license file for {}. Error: {:?}",
                                        dependency.name,
                                        e
                                    )
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("{} has an invalid URI: {:?}", dependency.name, e);
                        }
                    },
                },
            },
            Some(license_name) => {
                out_markdown.append(&mut license(
                    &dependency.name,
                    &dependency.version.to_string(),
                    &licenses::lookup_md(&license_name, &dependency.authors).join("\n\n"),
                    dependency.authors,
                    dependency.repository,
                ));
            }
        }
    }

    let outhtml = comrak::markdown_to_html(&out_markdown.join(""), &ComrakOptions::default());
    match File::create(&opts.outpath).await {
        Ok(mut f) => {
            match f.write_all(outhtml.as_bytes()).await {
                Ok(_) => {
                    log::info!("Wrote output to {}", &opts.outpath.display());
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        },
        Err(e) => {
            log::error!("{:?}", e);
        }
    };
}
