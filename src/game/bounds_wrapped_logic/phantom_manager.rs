use bevy_light_2d::light::PointLight2d;

use crate::prelude::*;

pub struct PhantomManagerPlugin;

impl Plugin for PhantomManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_phantom_objects_screens,
                update_monster_phantom_children,
                update_bomb_phantom_children,
                //TODO
                //update bomb_text_and_light_for_phantom_bombs
            ),
        );
    }
}

fn update_phantom_objects_screens(
    mut phantom_transforms: Query<
        (&mut Transform, &Parent, &BoundsWrappedPhantom),
        Without<WorldBoundsWrapped>,
    >,
    changed_bounds_wrapped_transforms: Query<
        (&Transform, Entity),
        (With<WorldBoundsWrapped>, Changed<Transform>),
    >,
) {
    for (parent_transform, parent_entity) in &changed_bounds_wrapped_transforms {
        for (mut phantom_transform, parent, bound_wrapped_phantom_component) in
            &mut phantom_transforms
        {
            if parent.get() == parent_entity {
                let locations_relative_to_parent =
                    BoundsWrappedPhantom::relative_location_to_parent(
                        parent_transform.translation.truncate(),
                    );
                if let Some(new_transform) =
                    locations_relative_to_parent.get(bound_wrapped_phantom_component.index)
                {
                    phantom_transform.translation =
                        new_transform.extend(phantom_transform.translation.z);
                }
            }
        }
    }
}

fn update_monster_phantom_children(
    changed_monsters: Query<
        (&Monster, &TextureAtlas, &Sprite, &Children),
        (
            Or<(Changed<Monster>, Changed<TextureAtlas>, Changed<Sprite>)>,
            Without<BoundsWrappedPhantom>,
        ),
    >,
    mut phantom_monsters: Query<
        (&mut Monster, &mut TextureAtlas, &mut Sprite),
        With<BoundsWrappedPhantom>,
    >,
) {
    for (changed_monster, changed_atlas, changed_sprite, changed_monster_children) in
        &changed_monsters
    {
        for changed_monster_child in changed_monster_children {
            if let Ok((mut monster, mut phantom_child_atlas, mut phantom_child_sprite)) =
                phantom_monsters.get_mut(*changed_monster_child)
            {
                *monster = changed_monster.clone();
                *phantom_child_atlas = changed_atlas.clone();
                *phantom_child_sprite = changed_sprite.clone();
            }
        }
    }
}

fn update_bomb_phantom_children(
    changed_bombs: Query<
        (&Bomb, &Sprite, &Children),
        (
            Or<(Changed<Bomb>, Changed<Sprite>)>,
            Without<BoundsWrappedPhantom>,
        ),
    >,
    mut phantom_bombs: Query<(&mut Bomb, &mut Sprite), With<BoundsWrappedPhantom>>,
) {
    for (changed_bomb, changed_sprite, changed_bomb_children) in &changed_bombs {
        for bomb_child in changed_bomb_children {
            if let Ok((mut bomb, mut phantom_child_sprite)) = phantom_bombs.get_mut(*bomb_child) {
                *bomb = changed_bomb.clone();
                *phantom_child_sprite = changed_sprite.clone();
            }
        }
    }
}
