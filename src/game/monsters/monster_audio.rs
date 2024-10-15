use crate::prelude::*;

pub struct MonsterAudioPlugin;

impl Plugin for MonsterAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sound_monster_sounds);
    }
}

fn sound_monster_sounds(
    mut monster_state_set_reader: EventReader<MonsterStateChanged>,
    mut bomb_explosion_reader: EventReader<BombExploded>,
    mut sounds_event_writer: EventWriter<SoundEvent>,
) {
    for set_request in monster_state_set_reader.read() {
        if let MonsterState::Chasing(_) = set_request.next_state {
            sounds_event_writer.send(SoundEvent::MonsterBattleCry);
        }
        break;
    }
    for bomb_explosion in bomb_explosion_reader.read() {
        if bomb_explosion.hit_monster {
            sounds_event_writer.send(SoundEvent::MonsterDeathCry);
        }
        break;
    }
}
