use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OsAccessError<'a> {
    CouldntFindFile(SystemFileName),
    BadFolderPath(FolderToAccess),
    CouldntParseFile(SystemFileName),
    MismatchingPostfix(MismatchError<'a>),
}

impl<'a> Display for OsAccessError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OsAccessError::CouldntFindFile(system_file_name) => {
                write!(
                    f,
                    "couldn't find file {}",
                    system_file_name.name_with_postfix
                )
            }
            OsAccessError::BadFolderPath(folder_to_access) => {
                write!(f, "bad folder file for {}", folder_to_access)
            }
            OsAccessError::CouldntParseFile(system_file_name) => {
                write!(f, "couldn't parse {}", system_file_name.name_with_postfix)
            }
            OsAccessError::MismatchingPostfix(mismatch_error) => mismatch_error.fmt(f),
        }
    }
}

impl<'a> From<MismatchError<'a>> for OsAccessError<'a> {
    fn from(mismatch_error: MismatchError<'a>) -> Self {
        OsAccessError::MismatchingPostfix(mismatch_error)
    }
}
