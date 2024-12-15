use std::fmt;

/// A one-off error that can be used to store a single error message.
pub struct OneOff<M> {
    msg: M,
}

impl<M> OneOff<M> {
    /// Create a new one-off error with the given message.
    pub fn new(msg: M) -> Self {
        Self { msg }
    }
}

impl<M> fmt::Debug for OneOff<M>
where
    M: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl<M> fmt::Display for OneOff<M>
where
    M: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl<M> std::error::Error for OneOff<M>
where
    M: fmt::Display + fmt::Debug,
{}
