use bevy_light_2d::light::PointLight2d;
use rand::Rng;

use crate::prelude::*;

pub struct DynamicLightManagerPlugin;

impl Plugin for DynamicLightManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            candle_light_bomb_effect.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn candle_light_bomb_effect(mut bomb_point_light_query: Query<&mut PointLight2d>) {
    let mut rng = rand::thread_rng();
    for mut bomb in bomb_point_light_query.iter_mut() {
        bomb.intensity = rng.gen_range(0.0..3.0);
        bomb.falloff = rng.gen_range(0.0..1.0);
    }
}
