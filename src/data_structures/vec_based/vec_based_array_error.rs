use std::fmt::{Debug, Display};

use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VecBasedArrayError<
    T: Debug + Copy + PartialEq,
    S: Debug + Copy + PartialEq,
    const N: usize,
> {
    FoundNoItemToMatchWith(S, VecBasedArray<T, N>),
    IndexOutOfRange(usize, VecBasedArray<T, N>),
    ItemWithAffectedEntityNotFound(Entity),
}

impl<T: Debug + Copy + PartialEq, S: Debug + Copy + PartialEq, const N: usize> Display
    for VecBasedArrayError<T, S, N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FoundNoItemToMatchWith(item, vec_based_array) => {
                write!(
                    f,
                    "Couldn't find item to match with: {:?} in vec-based array: {:?} ",
                    item, vec_based_array
                )
            }
            Self::IndexOutOfRange(index, vec_based_array) => {
                write!(
                    f,
                    "Index: {:?} out of range for vec-based array: {:?} ",
                    index, vec_based_array
                )
            }
            Self::ItemWithAffectedEntityNotFound(entity) => {
                write!(f, "Couldn't find item with affected entity: {:?}", entity,)
            }
        }
    }
}
