# DoomGame
This is a personal project, where I try to build a Doom like game using Rust programming language.

1. main.rs calls app::run()
2. app.rs creates the Bevy app:
   - Adds plugins (rendering, physics)
   - Adds startup systems (spawn world, camera, physics setup)
   - Adds update systems (player movement, mouse, exit)
3. The systems run every frame:
   - Keyboard moves player velocity
   - Mouse rotates camera
   - Escape exits the app
