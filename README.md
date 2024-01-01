# farmstead.
Farmstead is a game about farming, developed for farmers, by farmers. It is built using the Rust programming language and the Bevy game engine.

## Game Elements

### Entities
The game includes various entities such as the player and clickable objects. The player entity has components like [Player](file:///Users/willmcglaughlin/Documents/Games/farmstead/src/entities/player.rs#4%2C12-4%2C12), `Highlight[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)AnimationIndices`, and `AnimationTimer`. The `Highlight` component is used to highlight the player when selected.

### User Interface
The user interface includes a 2D camera that can be moved around using the `W`, `A`, `S[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)D` keys. The camera's zoom level can be adjusted using the [Z](file:///Users/willmcglaughlin/Documents/Games/farmstead/src/ui/camera.rs#42%2C44-42%2C44) and [X](file:///Users/willmcglaughlin/Documents/Games/farmstead/src/ui/camera.rs#46%2C44-46%2C44) keys.

### Map
The game map is a tilemap generated using Perlin noise. It includes different types of tiles such as [Field](file:///Users/willmcglaughlin/Documents/Games/farmstead/src/map/tilemap.rs#29%2C24-29%2C24), `Grass[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)Farmland[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)Dirt`, `Stone`, and `Rock`.

### Player Interaction
The player can be moved around the map by clicking on a location. The player's sprite animation changes based on the movement.

## Development
The game is organized into different modules such as `entities[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)ui`, and [map](file:///Users/willmcglaughlin/Documents/Games/farmstead/src/main.rs#4%2C5-4%2C5). The `main.rs` file is the entry point of the game where all the systems and plugins are added to the Bevy app.

## Dependencies
The game uses several dependencies including [bevy](file:///Users/willmcglaughlin/Documents/Games/farmstead/Cargo.toml#9%2C1-9%2C1), `bevy_ecs_tilemap[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)noise[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)num-derive[, ](file:///Users/willmcglaughlin/Documents/Games/farmstead/README.md#2%2C21-2%2C21)num-traits`, and `rand`.

## Building the Game
To build the game, you need to have Rust installed on your machine. You can then use the command `cargo build` to compile the game.

## Running the Game
You can run the game using the command `cargo run`.