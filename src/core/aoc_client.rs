use std::{fmt, fs, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use reqwest::Method;
use crate::core::input::Input;

pub struct AocClient {
    client: reqwest::Client,
}

pub enum AocClientError<I: Input> {
    Request(reqwest::Error),
    Io(io::Error),
    InputError(Box<I::ParseError>),
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

    pub async fn get_challenge<I: Input>(&self, year: u32, day: u32) -> Result<I, AocClientError<I>> {
        let text = match self.get_challenge_from_fs(year, day)? {
            Some(text) => text,
            None => self.get_challenge_from_server(year, day).await?
        };

        match I::from_input(text).await {
            Ok(input) => Ok(input),
            Err(err) => Err(AocClientError::from_input_error(err)),
        }
    }

    fn cache_path(year: u32, day: u32) -> PathBuf {
        PathBuf::from(format!(".cache/{}/day{:0>2}-input.txt", year, day))
    }

    // Get the cached input from the cache file
    fn get_challenge_from_fs(&self, year: u32, day: u32) -> Result<Option<String>, io::Error> {
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

    fn try_save_challenge_to_fs(&self, year: u32, day: u32, text: &str) -> Result<(), io::Error> {
        let path = Self::cache_path(year, day);
        fs::create_dir_all(path.parent().expect("Path should have parent"))?;
        fs::write(&path, text)
    }

    async fn get_challenge_from_server(&self, year: u32, day: u32) -> Result<String, reqwest::Error> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let res = self.client.request(Method::GET, &url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        self.try_save_challenge_to_fs(year, day, &res)
            .unwrap_or_else(|err| {
                crate::core::io::print_debug(format_args!("Failed to cache input: {}", err));
            });

        Ok(res)
    }
}

impl<I: Input> AocClientError<I> {
    fn from_input_error(err: I::ParseError) -> Self {
        AocClientError::InputError(Box::new(err))
    }
}

impl<I: Input> From<reqwest::Error> for AocClientError<I> {
    fn from(err: reqwest::Error) -> Self {
        AocClientError::Request(err)
    }
}

impl<I: Input> From<io::Error> for AocClientError<I> {
    fn from(err: io::Error) -> Self {
        AocClientError::Io(err)
    }
}

impl<I: Input> fmt::Debug for AocClientError<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AocClientError::Request(err) => {
                write!(f, "Request error: ")?;
                fmt::Debug::fmt(err, f)
            }
            AocClientError::Io(err) => {
                write!(f, "IO error: ")?;
                fmt::Debug::fmt(err, f)
            }
            AocClientError::InputError(err) => {
                write!(f, "Input error: ")?;
                fmt::Debug::fmt(err, f)
            }
        }
    }
}

impl<I: Input> fmt::Display for AocClientError<I> {
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
            AocClientError::InputError(err) => {
                write!(f, "Input error: ")?;
                fmt::Display::fmt(err, f)
            }
        }
    }
}

impl<I: Input> std::error::Error for AocClientError<I> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AocClientError::Request(err) => Some(err),
            AocClientError::Io(err) => Some(err),
            AocClientError::InputError(err) => Some(err.as_ref()),
        }
    }
}
