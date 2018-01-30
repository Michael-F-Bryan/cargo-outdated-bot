use std::process::Output;
use std::fmt::{self, Display, Formatter};
use std::io;

/// Unable to invoke a command.
#[derive(Debug, Fail)]
#[fail(display = "Unable to invoke \"{}\"", name)]
pub struct FailedInvocation {
    name: String,
    #[cause]
    inner: io::Error,
}

impl FailedInvocation {
    pub fn new<S: Into<String>>(name: S, inner: io::Error) -> FailedInvocation {
        FailedInvocation {
            name: name.into(),
            inner: inner,
        }
    }
}

#[derive(Debug, Fail)]
pub struct FailedCommand {
    name: String,
    output: Output,
}

impl Display for FailedCommand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The call to \"{}\" failed", self.name)?;
        if let Some(code) = self.output.status.code() {
            write!(f, " (exit code: {})", code)?;
        }

        Ok(())
    }
}
