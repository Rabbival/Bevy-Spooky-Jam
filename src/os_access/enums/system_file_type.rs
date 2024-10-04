use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, EnumIter)]
pub enum SystemFileType {
    TextFile,
}

impl SystemFileType {
    pub fn to_postfix(&self) -> String {
        match self {
            Self::TextFile => String::from(".txt"),
        }
    }
}
