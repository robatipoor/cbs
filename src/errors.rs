use failure::Fail;
use std::result::Result as StdResult;

pub type Result<T = ()> = StdResult<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "")]
    ParseError,
    #[fail(display = "")]
    DaemonError,
    #[fail(display = "")]
    ClipboardProviderError,
    #[fail(display = "")]
    PollCodecError,
    #[fail(display = "")]
    PollFlushError,
    #[fail(display = "")]
    SetContentsError,
    #[fail(display = "")]
    GetContentsError,
    #[fail(display = "")]
    FileNotExistError,
    #[fail(display = "")]
    RemoveFileError,
    #[fail(display = "")]
    OpenFileError,
    #[fail(display = "")]
    ReadToStringError,
}
