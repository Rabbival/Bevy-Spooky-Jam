Soul Bomb Monster Hunt v0.3.1
=============================

Soul Bomb Monster Hunt Bevy Spooky Jam entry.
A link to the game's itch page: https://rabbival.itch.io/soul-bomb-monster-hunt-bevy-spooky-jam-entry

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


Try to kill as many monsters as possible, and survive as long as you can.

Monsters can't see you, but they can hear you, so don't get too close. Luckily for you, picking up a soul bomb slows your perception of time, so you'll have an easier time aiming them!

Good luck!

#### Controls

* Press <kbd>A</kbd> or arrow <kbd>LEFT</kbd> to move player left
* Press <kbd>D</kbd> or arrow <kbd>RIGHT</kbd> to move player right
* Press <kbd>W</kbd> or arrow <kbd>UP</kbd> to move player up
* Press <kbd>S</kbd> or arrow <kbd>DOWN</kbd> to move player down
* Press <kbd>LEFT MOUSE BUTTON</kbd> to pickup, hold, and throw a soul bomb aiming to cursor pointer
  * You may also press <kbd>Spacebar</kbd> to pickup, and hold a soul bomb aiming to cursor pointer
