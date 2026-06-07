use std::io;

#[derive(Debug, Clone, Copy)]
enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
}

enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Water,
    Ship(ShipType), // Ship type
    Hit,
    Miss,
}

struct ShipSpace {
    #[allow(dead_code)]//will be used in the future when we implement the logic to check if a ship is sunk or not. make the add function much cleaner.
    ship_type: ShipType,
    orientation: Orientation,
    start: (usize, usize),
}
struct Gamer {
    board: [[Cell; 8]; 8],
    num_of_ships_left: u8,
    ships: [ShipSpace; 3], //ship[0] is the carrier, ship[1] is the battleship, ship[2] is the destroyer
}

impl Gamer {
    //this creates a new gamer with an empty board and no ships
    fn new() -> Self {
        Self {
            board: [[Cell::Water; 8]; 8],
            num_of_ships_left: 3,
            ships: [
                ShipSpace {
                    ship_type: ShipType::Carrier,
                    orientation: Orientation::Horizontal,
                    start: (0, 0),
                },
                ShipSpace {
                    ship_type: ShipType::Battleship,
                    orientation: Orientation::Horizontal,
                    start: (0, 0),
                },
                ShipSpace {
                    ship_type: ShipType::Destroyer,
                    orientation: Orientation::Horizontal,
                    start: (0, 0),
                },
            ],
        }
    }

    //this tells us how many ships the gamer has left
    fn status(&self) {
        println!("Ships remaining: {}", self.num_of_ships_left);
    }

    //this shows the current state of the gamer's board, with W for water, C for carrier, B for battleship, D for destroyer, X for hit, and O for miss
    fn map(&self) {
        println!("Gamer's board status:");
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Water => print!("W "),
                    Cell::Ship(ShipType::Carrier) => print!("C "),
                    Cell::Ship(ShipType::Battleship) => print!("B "),
                    Cell::Ship(ShipType::Destroyer) => print!("D "),
                    Cell::Hit => print!("X "),
                    Cell::Miss => print!("O "),
                }
            }
            println!();
        }
    }

    //this checks if the shot taken by the opponent is a hit or a miss, and updates the board accordingly. It also checks if the cell has already been targeted before.
    fn shot_taken(&mut self, x: usize, y: usize) {
        match self.board[y][x] {
            Cell::Water => {
                self.board[y][x] = Cell::Miss;
                println!("Miss!");
            }
            Cell::Ship(ship) => {
                self.ship_hit(ship);
                self.num_of_ships_left -= 1;
                println!("Hit!");
                // Here you would also want to check if the ship is sunk and update the Ships count accordingly.
            }
            Cell::Hit | Cell::Miss => {
                println!("Already targeted this cell.");
            }
        }
    }

    //this function is called when a ship is hit, and it updates the board to show the hit.
    fn ship_hit(&mut self, ship: ShipType) {
        // Here you would want to implement logic to check if the ship is sunk and update the Ships count accordingly.
        let size: u8;
        let (x, y): (usize, usize);
        match ship {
            ShipType::Carrier => {
                // Check if the carrier is sunk and update Ships count
                size = 5;
                (x, y) = self.ships[0].start;
            }
            ShipType::Battleship => {
                // Check if the battleship is sunk and update Ships count
                size = 4;
                (x, y) = self.ships[1].start;
            }
            ShipType::Destroyer => {
                // Check if the destroyer is sunk and update Ships count
                size = 3;
                (x, y) = self.ships[2].start;
            }
        }
        match self.ships[(5 - size) as usize].orientation {
            Orientation::Horizontal => {
                // Check vertically if the ship is sunk
                for i in 0..size {
                    self.board[y][x + i as usize] = Cell::Hit;
                }
            }
            Orientation::Vertical => {
                // Check horizontally if the ship is sunk
                for i in 0..size {
                    self.board[y + i as usize][x] = Cell::Hit;
                }
            }
        }
    }

    fn add(&mut self) {
        loop {
            println!("first place the carrier");
            println!("enter the x value of the shot");
            let x: usize = intput() as usize;
            println!("enter the y value of the shot");
            let y: usize = intput() as usize;
            println!("enter the orientation of the ship. 1 for horizontal, 2 for vertical");
            let orientation: usize = intput() as usize;
            if orientation == 1 {
                if x + 4 > 7 {
                    println!(
                        "invalid orientation, the ship will go out of bounds. please try again."
                    );
                    continue;
                }
                self.ships[0] = ShipSpace {
                    ship_type: ShipType::Carrier,
                    start: (x, y),
                    orientation: Orientation::Horizontal,
                };
                for i in 0..5 {
                    self.board[y][x + i as usize] = Cell::Ship(ShipType::Carrier);
                }
            } else if orientation == 2 {
                if y + 4 > 7 {
                    println!(
                        "invalid orientation, the ship will go out of bounds. please try again."
                    );
                    continue;
                }
                self.ships[0] = ShipSpace {
                    ship_type: ShipType::Carrier,
                    start: (x, y),
                    orientation: Orientation::Vertical,
                };
                for i in 0..5 {
                    self.board[y + i as usize][x] = Cell::Ship(ShipType::Carrier);
                }
            }
            break;
        }
        loop {
            println!("first place the battleship");
            println!("enter the x value of the shot");
            let x: usize = intput() as usize;
            println!("enter the y value of the shot");
            let y: usize = intput() as usize;
            println!("enter the orientation of the ship. 1 for horizontal, 2 for vertical");
            let orientation: usize = intput() as usize;
            if orientation == 1 {
                if x + 3 > 7 {
                    println!(
                        "invalid orientation, the ship will go out of bounds. please try again."
                    );
                    continue;
                }
                self.ships[1] = ShipSpace {
                    ship_type: ShipType::Battleship,
                    start: (x, y),
                    orientation: Orientation::Horizontal,
                };
                for i in 0..4 {
                    self.board[y][x + i as usize] = Cell::Ship(ShipType::Battleship);
                }
            } else if orientation == 2 {
                if y + 3 > 7 {
                    println!(
                        "invalid orientation, the ship will go out of bounds. please try again."
                    );
                    continue;
                }

                self.ships[1] = ShipSpace {
                    ship_type: ShipType::Battleship,
                    start: (x, y),
                    orientation: Orientation::Vertical,
                };
                for i in 0..4 {
                    self.board[y + i as usize][x] = Cell::Ship(ShipType::Battleship);
                }
            }
            break;
        }
        loop {
            println!("first place the destroyer");
            println!("enter the x value of the shot");
            let x: usize = intput() as usize;
            println!("enter the y value of the shot");
            let y: usize = intput() as usize;
            println!("enter the orientation of the ship. 1 for horizontal, 2 for vertical");
            let orientation: usize = intput() as usize;
            if orientation == 1 {
                if x + 2 > 7 {
                    println!(
                        "invalid orientation, the ship will go out of bounds. please try again."
                    );
                    continue;
                }
                self.ships[2] = ShipSpace {
                    ship_type: ShipType::Destroyer,
                    start: (x, y),
                    orientation: Orientation::Horizontal,
                };
                for i in 0..3 {
                    self.board[y][x + i as usize] = Cell::Ship(ShipType::Destroyer);
                }
            } else if orientation == 2 {
                if y + 2 > 7 {
                    println!(
                        "invalid orientation, the ship will go out of bounds. please try again."
                    );
                    continue;
                }
                self.ships[2] = ShipSpace {
                    ship_type: ShipType::Destroyer,
                    start: (x, y),
                    orientation: Orientation::Vertical,
                };
                for i in 0..3 {
                    self.board[y + i as usize][x] = Cell::Ship(ShipType::Destroyer);
                }
            }
            break;
        }
    }
}

fn main() {
    //starting the game between two gamers
    start_game();
}

fn start_game() {
    //test steps for now, we will just create two gamers and print their boards.
    let mut gamer1 = Gamer::new();
    let mut gamer2 = Gamer::new();
    // Initialize the game board, place ships, etc.
    //get gamers maps setuped
    println!("Gamer 1, place your ships.");
    gamer1.add();
    println!("Gamer 2, place your ships.");
    gamer2.add();
    println!("game start!");
    loop {
        //player chooses what to do.
        println!("please tell what to do. 1.exit 2. take a shot. 3. check status. 4. show map.");
        let input1: u8 = intput();
        if input1 == 1 {
            break;
        } else if input1 == 2 {
            //take a shot
            println!("enter the x value of the shot");
            let x: usize = intput() as usize;
            println!("enter the y value of the shot");
            let y: usize = intput() as usize;
            gamer2.shot_taken(x, y);
            gamer2.status();
            //this makes the game turn based. The first gamer will be the one who starts, and then they will switch after each turn.
            std::mem::swap(&mut gamer1, &mut gamer2);
        } else if input1 == 3 {
            //check status
            gamer2.status();
        } else if input1 == 4 {
            //show map
            gamer1.map();
        }
    }
}

fn intput() -> u8 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u8 = input.trim().parse().expect("Please type a number!");
    input
}
