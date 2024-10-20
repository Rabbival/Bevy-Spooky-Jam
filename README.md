Soul Bomb Monster Hunt v0.2.0
=============================

Soul Bomb Monster Hunt Bevy Spooky Jam entry.

### Installation requirements

* Git ≥ v2.0
* Rust ≥ v1.8
* Cargo ≥ v1.80

### Installation instructions

```bash
$ git clone git@github.com:Rabbival/Bevy-Spooky-Jam.git soul-bomb-monster-hunt
$ cd soul-bomb-monster-hunt
$ cargo run --release
```

#### WASM notes

```bash
$ cargo run --target wasm32-unknown-unknown -- DISABLE_OUTPUT_LOG_FILE
```

### Game instructions

#### Goal

When near a bomb, press left mouse button to pick it up. When held, aim the bomb, then release it.

Monsters can't see you, but they can hear you, so don't get too close. Luckily for you, picking up a soul bomb slows your perception of time, so you'll have an easier time aiming them!

Try to kill as many monsters as possible. Good luck!

#### Controls

* press <kbd>A</kbd> or arrow <kbd>LEFT</kbd> to move player left
* press <kbd>D</kbd> or arrow <kbd>RIGHT</kbd> to move player right
* press <kbd>W</kbd> or arrow <kbd>UP</kbd> to move player up
* press <kbd>S</kbd> or arrow <kbd>DOWN</kbd> to move player down
* press <kbd>LEFT MOUSE BUTTON</kbd> to pickup, hold, and throw a pumpkin bomb aiming to cursor pointer
