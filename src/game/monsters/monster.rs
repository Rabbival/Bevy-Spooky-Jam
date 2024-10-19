use crate::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct Monster {
    pub hearing_ring_distance: f32,
    pub state: MonsterState,
    pub main_path: VecBasedArray<Vec3, MONSTER_MAX_PATH_VERTICES>,
    pub path_timer_sequence: Option<Entity>,
    pub animation_timer_sequence: Option<Entity>,
}

impl Monster {
    pub fn heading_direction_by_index(&self, index: usize) -> BasicDirection {
        let path_length = self.main_path.len();
        if let Some(vertice) = self.main_path.array[index] {
            if let Some(next_vertice) = self.main_path.array[(index + 1) % path_length] {
                let difference = (next_vertice - vertice).truncate();
                return BasicDirection::closest(difference);
            }
        }
        BasicDirection::Up //default
    }
}
