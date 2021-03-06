# FBSim

Retro-looking game-like football simulation environment for trying out AI ideas, written in [Rust](https://www.rust-lang.org/) using [amethyst](https://amethyst.rs/).

I wrote all of this in my free time to learn a bit about Rust and game development. The code is very scarcely documented and there is no testing at all. Use at your own risk.

[Video of running game](https://www.youtube.com/watch?v=OGKFFvp1zzA)


![Screenshot of running game](screenshots/2020-11-20_17-09-47.png)

## Instructions

To load this install amethyst's dependencies and then do:

```
source env.sh # Needed because of a bug with rendy in vulkan.
cargo build
```

You may need to change amethyst's features in the `Cargo.toml` file if you're not on linux!

You can use `cargo run` to run it.

## Agents

Currently this only implements a `Basic` engine, where all players but the defender and the goalie run after the ball, and `BasicWingWait` which is (just a bit) slightly more complicated.

## Adding a new agent.

If you want to write your own engine, the way to do it is:

1. Create a structure implementing the trait `src/engines/engine.rs::Engine`. (Look at `src/engines/basic.rs` implementations as a guide)
2. Add it to the default engine registry in `src/resources/engine_registry.rs::EngineRegistry::default`.
3. Modify `assets/sprites/player.ron` so that `entities.data.extras.player.robot.logic_module` is `EngineRunner("<registered_name_for_your_engine>")`.
4. Run `cargo run` and see your agents play against the default `Basic` engine! (You can change `enemy.ron` to make them use a different engine as well).

## Roadmap

- Implement Reinforment Learning-based agents. (What I did this all for).
- Add support for implementations in a scripting language.
- Add more game-like features?

# Copyright

Copyright Ian Tayler <iangtayler@gmail.com> 2020. Licensed under the [MIT license](/LICENSE). Use and distribute without restriction, according to the license text.
