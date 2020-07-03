use std::{
    error::Error,
    fmt::{self, Display},
    io,
    string::FromUtf8Error,
};

use codemap::{Span, SpanLoc};

pub type SassResult<T> = Result<T, Box<SassError>>;

#[derive(Debug)]
pub struct SassError {
    kind: SassErrorKind,
}

// todo: we should split the unclonable errors (io, potentially others) into
// a separate enum to allow these methods to be infallible
#[allow(clippy::unimplemented)]
impl Clone for SassError {
    #[inline]
    fn clone(&self) -> Self {
        match &self.kind {
            SassErrorKind::Raw(a, b) => SassError {
                kind: SassErrorKind::Raw(a.clone(), *b),
            },
            SassErrorKind::ParseError { message, loc } => SassError {
                kind: SassErrorKind::ParseError {
                    message: message.clone(),
                    loc: loc.clone(),
                },
            },
            _ => unimplemented!(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl PartialEq for SassError {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match &self.kind {
            SassErrorKind::Raw(a, b) => match &other.kind {
                SassErrorKind::Raw(c, d) => a == c && b == d,
                _ => false,
            },
            SassErrorKind::ParseError {
                message: message1,
                loc: loc1,
            } => match &other.kind {
                SassErrorKind::ParseError {
                    message: message2,
                    loc: loc2,
                } => message1 == message2 && loc1 == loc2,
                _ => false,
            },
            _ => unimplemented!(),
        }
    }
}

impl Eq for SassError {}

impl SassError {
    pub(crate) fn raw(self) -> (String, Span) {
        match self.kind {
            SassErrorKind::Raw(string, span) => (string, span),
            e => todo!("unable to get raw of {:?}", e),
        }
    }

    pub(crate) const fn from_loc(message: String, loc: SpanLoc) -> Self {
        SassError {
            kind: SassErrorKind::ParseError { message, loc },
        }
    }
}

#[derive(Debug)]
enum SassErrorKind {
    /// A raw error with no additional metadata
    /// It contains only a `String` message and
    /// a span
    Raw(String, Span),
    ParseError {
        message: String,
        loc: SpanLoc,
    },
    IoError(io::Error),
    FromUtf8Error(String),
}

impl Display for SassError {
    // TODO: trim whitespace from start of line shown in error
    // TODO: color errors
    // TODO: integrate with codemap-diagnostics
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (message, loc) = match &self.kind {
            SassErrorKind::ParseError { message, loc } => (message, loc),
            SassErrorKind::FromUtf8Error(s) => return writeln!(f, "Error: {}", s),
            SassErrorKind::IoError(s) => return writeln!(f, "Error: {}", s),
            SassErrorKind::Raw(..) => todo!(),
        };
        let line = loc.begin.line + 1;
        let col = loc.begin.column + 1;
        writeln!(f, "Error: {}", message)?;
        let padding = vec![' '; format!("{}", line).len() + 1]
            .iter()
            .collect::<String>();
        writeln!(f, "{}|", padding)?;
        writeln!(f, "{} | {}", line, loc.file.source_line(loc.begin.line))?;
        writeln!(
            f,
            "{}| {}{}",
            padding,
            vec![' '; loc.begin.column].iter().collect::<String>(),
            vec!['^'; loc.end.column.max(loc.begin.column) - loc.begin.column.min(loc.end.column)]
                .iter()
                .collect::<String>()
        )?;
        writeln!(f, "{}|", padding)?;
        writeln!(f, "./{}:{}:{}", loc.file.name(), line, col)?;
        Ok(())
    }
}

impl From<io::Error> for Box<SassError> {
    #[inline]
    fn from(error: io::Error) -> Box<SassError> {
        Box::new(SassError {
            kind: SassErrorKind::IoError(error),
        })
    }
}

impl From<FromUtf8Error> for Box<SassError> {
    #[inline]
    fn from(error: FromUtf8Error) -> Box<SassError> {
        Box::new(SassError {
            kind: SassErrorKind::FromUtf8Error(format!(
                "Invalid UTF-8 character \"\\x{:X?}\"",
                error.as_bytes()[0]
            )),
        })
    }
}

impl From<(&str, Span)> for Box<SassError> {
    #[inline]
    fn from(error: (&str, Span)) -> Box<SassError> {
        Box::new(SassError {
            kind: SassErrorKind::Raw(error.0.to_owned(), error.1),
        })
    }
}

impl From<(String, Span)> for Box<SassError> {
    #[inline]
    fn from(error: (String, Span)) -> Box<SassError> {
        Box::new(SassError {
            kind: SassErrorKind::Raw(error.0, error.1),
        })
    }
}

impl Error for SassError {
    #[inline]
    fn description(&self) -> &'static str {
        "Sass parsing error"
    }
}
