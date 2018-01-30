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
pub struct CommandFailed {
    name: String,
    output: Output,
}

impl CommandFailed {
    pub fn new<S: Into<String>>(name: S, output: Output) -> CommandFailed {
        CommandFailed {
            name: name.into(),
            output: output,
        }
    }
}

impl Display for CommandFailed {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The call to \"{}\" failed", self.name)?;
        if let Some(code) = self.output.status.code() {
            write!(f, " (exit code: {})", code)?;
        }

        Ok(())
    }
}
