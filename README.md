# battleShipGE
# Battleship Game in Rust

A terminal-based, turn-based Battleship game implemented in Rust.

## Features
- Two-player local multiplayer.
- Ships placement (Carrier, Battleship, Destroyer).
- Boundary validation and hit tracking.
- Turn-based game loop.

## How to Run
1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Open your terminal in the project folder.
4. Run: `cargo run`

## How to Play
- Follow the on-screen prompts to place your ships.
- Enter X and Y coordinates to fire shots.
- The game will keep track of hits, misses, and remaining ships.1

## Gameplay
`Add ships for both the users`
Gamer 1, place your ships.
first place the carrier
`choose coordinates and the orientation for the ships`
enter the x value of the shot
1
enter the y value of the shot
1
enter the orientation of the ship. 1 for horizontal, 2 for vertical
1
first place the battleship
enter the x value of the shot
0
enter the y value of the shot
0
enter the orientation of the ship. 1 for horizontal, 2 for vertical
2
first place the destroyer
enter the x value of the shot
3
enter the y value of the shot
3
enter the orientation of the ship. 1 for horizontal, 2 for vertical
1
Gamer 2, place your ships.
first place the carrier
enter the x value of the shot
0
enter the y value of the shot
0
enter the orientation of the ship. 1 for horizontal, 2 for vertical
1
first place the battleship
enter the x value of the shot
0
enter the y value of the shot
2
enter the orientation of the ship. 1 for horizontal, 2 for vertical
2
first place the destroyer
enter the x value of the shot
2
enter the y value of the shot
2
enter the orientation of the ship. 1 for horizontal, 2 for vertical
1
game start!
`3 shows the number of ships enemy has left`
please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.
3
Ships remaining: 3
`4 shows your own map C-carrier, B-battleship, D-destroyer, W-water, X-hit and M-miss`
please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.
4
Gamer's board status:
B W W W W W W W 
B C C C C C W W 
B W W W W W W W 
B W W D D D W W 
W W W W W W W W 
W W W W W W W W 
W W W W W W W W 
W W W W W W W W 
`2 to shoot, you choose the coordinate and game tells you if you hit, miss hit or are rehitting`
please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.
2
enter the x value of the shot
0
enter the y value of the shot
0
Hit!
`also tells how many ships are left`
Ships remaining: 2
please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.
3
Ships remaining: 3
please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.
4
`see the map to see the hit ship`
Gamer's board status:
X X X X X W W W 
W W W W W W W W 
B W D D D W W W 
B W W W W W W W 
B W W W W W W W 
B W W W W W W W 
W W W W W W W W 
W W W W W W W W 
`1 for simple exit`
please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.
1
