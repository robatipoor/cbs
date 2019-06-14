use failure::Fail;
use std::result::Result as StdResult;

pub type Result<T = ()> = StdResult<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "parse error")]
    ParseError,
    #[fail(display = "daemon error")]
    DaemonError,
    #[fail(display = "clipboard provider error")]
    ClipboardProviderError,
    #[fail(display = "poll codec error")]
    PollCodecError,
    #[fail(display = "poll flush error")]
    PollFlushError,
    #[fail(display = "set content error")]
    SetContentsError,
    #[fail(display = "set content error")]
    GetContentsError,
    #[fail(display = "file not exist error")]
    FileNotExistError,
    #[fail(display = "remove file error")]
    RemoveFileError,
    #[fail(display = "open file error")]
    OpenFileError,
    #[fail(display = "read to string error")]
    ReadToStringError,
}
