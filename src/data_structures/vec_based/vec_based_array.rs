use crate::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecBasedArray<T: Debug + Copy + PartialEq, const N: usize> {
    pub array: [Option<T>; N],
    next_uninitialized_index: usize,
}

impl<T: Debug + Copy + PartialEq, const N: usize> VecBasedArray<T, N> {
    pub fn new(value: Vec<T>) -> Self {
        let mut array = [None; N];
        let shortened_vec = truncated_if_at_limit(value, N);
        let next_uninitialized_index = shortened_vec.len();
        for (i, elem) in shortened_vec.into_iter().enumerate() {
            array[i] = Some(elem);
        }
        Self {
            array,
            next_uninitialized_index,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.array.iter().flatten().copied()
    }

    pub fn len(&self) -> usize {
        self.next_uninitialized_index
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove_by_item(&mut self, item_to_remove: T) -> Result<T, VecBasedArrayError<T, T, N>> {
        self.remove_first_matching_item(item_to_remove, |a, b| a == b)
    }

    fn remove_first_matching_item<F, S>(
        &mut self,
        item_to_match: S,
        matcher: F,
    ) -> Result<T, VecBasedArrayError<T, S, N>>
    where
        S: Debug + Clone + Copy + PartialEq,
        F: Fn(S, T) -> bool,
    {
        match self.get_first_matching_item(item_to_match, matcher) {
            Some((index, _)) => self.remove_by_index(index),
            None => Err(VecBasedArrayError::FoundNoItemToMatchWith(
                item_to_match,
                *self,
            )),
        }
    }

    fn get_first_matching_item<F, S>(&self, item_to_match: S, matcher: F) -> Option<(usize, T)>
    where
        S: Debug + Clone + Copy + PartialEq,
        F: Fn(S, T) -> bool,
    {
        let mut maybe_matching_item = None;
        for (index, item) in self.iter().enumerate() {
            if matcher(item_to_match, item) {
                maybe_matching_item = Some((index, item));
                break;
            }
        }
        maybe_matching_item
    }

    pub fn remove_by_index<S: Debug + Copy + PartialEq>(
        &mut self,
        index: usize,
    ) -> Result<T, VecBasedArrayError<T, S, N>> {
        if index < self.next_uninitialized_index {
            let removed_item = self.array[index];
            self.array[index] = None;
            self.next_uninitialized_index -= 1;
            self.array.swap(index, self.next_uninitialized_index);
            Ok(removed_item.unwrap())
        } else {
            Err(VecBasedArrayError::IndexOutOfRange(index, *self))
        }
    }
}

impl<const N: usize> VecBasedArray<TimerAffectedEntity, N> {
    pub fn affected_entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.iter()
            .map(|affected_entity| affected_entity.affected_entity)
    }

    pub fn calculator_entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.iter()
            .filter_map(|affected_entity| affected_entity.value_calculator_entity)
    }

    pub fn remove_by_affected_entity(
        &mut self,
        entity_to_remove: Entity,
    ) -> Result<TimerAffectedEntity, VecBasedArrayError<TimerAffectedEntity, Entity, N>> {
        self.remove_first_matching_item(entity_to_remove, |entity_to_remove, affected_entity| {
            entity_to_remove == affected_entity.affected_entity
        })
    }

    pub fn remove_by_calculator_entity(
        &mut self,
        calculator_to_remove: Entity,
    ) -> Result<TimerAffectedEntity, VecBasedArrayError<TimerAffectedEntity, Entity, N>> {
        self.remove_first_matching_item(calculator_to_remove, Self::calculator_matcher)
    }

    pub fn get_by_calculator_entity(
        &self,
        calculator_to_remove: Entity,
    ) -> Option<TimerAffectedEntity> {
        self.get_first_matching_item(calculator_to_remove, Self::calculator_matcher)
            .map(|(_, item)| item)
    }

    fn calculator_matcher(
        calculator_to_remove: Entity,
        affected_entity: TimerAffectedEntity,
    ) -> bool {
        matches!(affected_entity.value_calculator_entity, Some(calculator_entity) if calculator_entity == calculator_to_remove)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_based_vec_creation() {
        let vec = vec![1, 2];

        let bigger_vec_based_array: VecBasedArray<usize, 3> = VecBasedArray::new(vec.clone());
        let smaller_vec_based_array: VecBasedArray<usize, 1> = VecBasedArray::new(vec.clone());

        assert_eq!(bigger_vec_based_array.array, [Some(1), Some(2), None]);
        assert_eq!(bigger_vec_based_array.next_uninitialized_index, 2);
        assert_eq!(smaller_vec_based_array.array, [Some(1)]);
        assert_eq!(smaller_vec_based_array.next_uninitialized_index, 1);
    }

    #[test]
    fn test_array_based_vec_iter() {
        let vec = vec![1, 2];

        let bigger_vec_based_array: VecBasedArray<usize, 3> = VecBasedArray::new(vec.clone());
        let smaller_vec_based_array: VecBasedArray<usize, 1> = VecBasedArray::new(vec.clone());

        assert_eq!(bigger_vec_based_array.iter().count(), 2);
        assert_eq!(smaller_vec_based_array.iter().count(), 1);
        assert_eq!(bigger_vec_based_array.iter().next(), Some(1));
        assert_eq!(smaller_vec_based_array.iter().next(), Some(1));
    }

    #[test]
    fn test_remove_by_item() {
        let vec = vec![1, 2];
        let mut vec_based_array: VecBasedArray<usize, 2> = VecBasedArray::new(vec.clone());
        let original_vec_based_array_copy = vec_based_array;

        let valid_removal_result = vec_based_array.remove_by_item(1);
        let item_not_found_removal_result = vec_based_array.remove_by_item(1);

        assert_eq!(2, original_vec_based_array_copy.next_uninitialized_index);
        assert_eq!([Some(2), None], vec_based_array.array);
        assert_eq!(1, vec_based_array.next_uninitialized_index);
        assert_eq!(Ok(1), valid_removal_result);
        assert_eq!(
            Err(VecBasedArrayError::FoundNoItemToMatchWith(
                1,
                vec_based_array
            )),
            item_not_found_removal_result
        );
    }
}
