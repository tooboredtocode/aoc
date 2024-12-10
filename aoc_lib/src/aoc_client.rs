use std::{fmt, fs, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use reqwest::Method;

pub struct AocClient {
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum AocClientError {
    Request(reqwest::Error),
    Io(io::Error),
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

    pub async fn get_challenge(&self, year: u16, day: u8) -> Result<String, AocClientError> {
        match self.get_challenge_from_fs(year, day)? {
            Some(text) => Ok(text),
            None => self.get_challenge_from_server(year, day).await
                .map_err(Into::into)
        }
    }

    fn cache_path(year: u16, day: u8) -> PathBuf {
        PathBuf::from(format!(".cache/{}/day{:0>2}-input.txt", year, day))
    }

    // Get the cached input from the cache file
    fn get_challenge_from_fs(&self, year: u16, day: u8) -> Result<Option<String>, io::Error> {
        let path = Self::cache_path(year, day);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                return if err.kind() == io::ErrorKind::NotFound {
                    Ok(None) // File not found
                } else {
                    Err(err)
                }
            }
        };

        let size = file.metadata().map(|m| m.len() as usize).ok();
        let mut res = String::new();
        res.try_reserve_exact(size.unwrap_or(0))?;
        file.read_to_string(&mut res)?;

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
                crate::io::print_debug(format_args!("Failed to cache input: {}", err));
            });

        Ok(res)
    }
}

impl From<reqwest::Error> for AocClientError {
    fn from(err: reqwest::Error) -> Self {
        AocClientError::Request(err)
    }
}

impl From<io::Error> for AocClientError {
    fn from(err: io::Error) -> Self {
        AocClientError::Io(err)
    }
}

impl fmt::Display for AocClientError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AocClientError::Request(err) => {
                write!(f, "Request error: ")?;
                fmt::Display::fmt(err, f)
            }
            AocClientError::Io(err) => {
                write!(f, "IO error: ")?;
                fmt::Display::fmt(err, f)
            }
        }
    }
}

impl std::error::Error for AocClientError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AocClientError::Request(err) => Some(err),
            AocClientError::Io(err) => Some(err),
        }
    }
}
