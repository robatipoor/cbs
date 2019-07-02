/// Represents variant types of content action
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Clear(Selection),
    Get(Selection),
    Set { content: String, select: Selection },
}

impl Action {
    pub fn get() -> Self {
        Action::Get(Selection::default())
    }
    pub fn set<S: Into<String>>(content: S) -> Self {
        Action::Set {
            content: content.into(),
            select: Selection::default(),
        }
    }
    pub fn clear() -> Self {
        Action::Clear(Selection::default())
    }
    pub fn select(self, selection: Selection) -> Self {
        use Action::*;
        match self {
            Clear(_) => Clear(selection),
            Get(_) => Get(selection),
            Set { content, .. } => Set {
                content: content,
                select: selection,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Selection {
    Primary,
    Secondary,
    Clipboard,
}

impl Default for Selection {
    fn default() -> Self {
        Selection::Clipboard
    }
}
