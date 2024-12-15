use anyhow::{Context, Result};
use reqwest::Method;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

pub struct AocClient {
    client: reqwest::Client,
}

const USER_AGENT: &str = concat!("aoc-client/", env!("CARGO_PKG_VERSION"));

impl AocClient {
    pub fn new(session: impl AsRef<str>) -> Self {
        fn inner(session: &str) -> AocClient {
            let client = reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .user_agent(USER_AGENT)
                .default_headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert(
                        reqwest::header::COOKIE,
                        reqwest::header::HeaderValue::from_str(&format!("session={}", session)).unwrap(),
                    );
                    headers
                })
                .build()
                .expect("Failed to build reqwest::Client");

            AocClient {
                client
            }
        }

        inner(session.as_ref())
    }

    pub async fn get_challenge(&self, year: u16, day: u8) -> Result<String> {
        match self.get_challenge_from_fs(year, day)
            .context("Failed to check the cache for the challenge")?
        {
            Some(text) => Ok(text),
            None => self.get_challenge_from_server(year, day).await
                .context("Failed to fetch challenge from server")
        }
    }

    fn cache_path(year: u16, day: u8) -> PathBuf {
        PathBuf::from(format!(".cache/{}/day{:0>2}-input.txt", year, day))
    }

    // Get the cached input from the cache file
    fn get_challenge_from_fs(&self, year: u16, day: u8) -> Result<Option<String>> {
        let path = Self::cache_path(year, day);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                return if err.kind() == io::ErrorKind::NotFound {
                    Ok(None) // File not found
                } else {
                    Err(anyhow::Error::from(err)
                        .context("Something went wrong while trying to open the cached input file"))
                }
            }
        };

        let size = file.metadata().map(|m| m.len() as usize).ok();
        let mut res = String::new();
        res.try_reserve_exact(size.unwrap_or(0))
            .context("Failed to read the cached input file")?;
        file.read_to_string(&mut res)
            .context("Failed to read the cached input file")?;

        Ok(Some(res))
    }

    fn try_save_challenge_to_fs(&self, year: u16, day: u8, text: &str) -> Result<(), io::Error> {
        let path = Self::cache_path(year, day);
        fs::create_dir_all(path.parent().expect("Path should have parent"))?;
        fs::write(&path, text)
    }

    async fn get_challenge_from_server(&self, year: u16, day: u8) -> Result<String, reqwest::Error> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let res = self.client.request(Method::GET, &url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        self.try_save_challenge_to_fs(year, day, &res)
            .unwrap_or_else(|err| {
                crate::io::print_debug(format_args!("{:?}",
                    anyhow::Error::new(err)
                        .context("Failed to save the challenge to the cache")
                ));
            });

        Ok(res)
    }
}
