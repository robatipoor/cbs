#[derive(Debug)]
pub enum Message {
    Get,
    Set(String, Option<usize>),
    Data(String),
    Ok,
}

impl Message {}
