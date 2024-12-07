use std::error::Error;
use std::fmt;

pub struct StringError {
    string: String,
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl StringError {
    pub fn new(string: impl Into<String>) -> Self {
        Self { string: string.into(), cause: None }
    }

    pub fn with_cause<S, E>(string: S, cause: E) -> Self
    where
        S: Into<String>,
        E: Error + Send + Sync + 'static,
    {
        Self { string: string.into(), cause: Some(Box::new(cause)) }
    }
}

impl fmt::Debug for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "{}(", self.string)?;
            fmt::Debug::fmt(cause, f)?;
            write!(f, ")")
        } else {
            write!(f, "{}", self.string)
        }
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "{}, caused by", self.string)?;
            fmt::Display::fmt(cause, f)
        } else {
            write!(f, "{}", self.string)
        }
    }
}

impl Error for StringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}
