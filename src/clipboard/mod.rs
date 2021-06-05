use std::sync::{Arc, Mutex};

use clipboard::{self, ClipboardContext, ClipboardProvider};
use tokio::task::spawn_blocking;

pub struct Clipboard {
    ctx: Arc<Mutex<ClipboardContext>>,
}

impl Clipboard {
    pub fn new() -> crate::Result<Self> {
        let ctx = Arc::new(Mutex::new(
            ClipboardProvider::new().map_err(|e| e.to_string())?,
        ));
        Ok(Self { ctx })
    }

    pub async fn get(&self) -> crate::Result<String> {
        let ctx = self.ctx.clone();
        spawn_blocking(move || match ctx.lock() {
            Ok(mut guard) => match guard.get_contents() {
                Ok(content) => content,
                Err(e) => {
                    panic!("get content error message : {}", e.to_string());
                }
            },
            Err(e) => {
                panic!("get lock error message : {}", e.to_string());
            }
        })
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn set(&self, content: String) -> crate::Result {
        let ctx = self.ctx.clone();
        spawn_blocking(move || match ctx.lock() {
            Ok(mut guard) => {
                if let Err(e) = guard.set_contents(content) {
                    panic!("set content error message : {}", e.to_string());
                }
            }
            Err(e) => {
                panic!("get lock error message : {}", e.to_string());
            }
        })
        .await
        .map_err(|e| e.to_string())
    }
}
