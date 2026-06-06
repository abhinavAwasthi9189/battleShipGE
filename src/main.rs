#[derive(Debug, Clone, Copy)]
enum ShipType {
    Carrier { orientation: char, length: u8 },
    Battleship { orientation: char, length: u8 },
    Destroyer { orientation: char, length: u8 },
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Water,
    Ship(ShipType),
    Hit,
    Miss,
}

struct Gamer {
    board: [[Cell; 8]; 8],
    ships: u8,
}

impl Gamer {
    //this creates a new gamer with an empty board and no ships
    fn new() -> Self {
        Self {
            board: [[Cell::Water; 8]; 8],
            ships: 0,
        }
    }

    //this tells us how many ships the gamer has left
    fn status(&self) {
        println!("Ships remaining: {}", self.ships);
    }

    //this shows the current state of the gamer's board, with W for water, C for carrier, B for battleship, D for destroyer, X for hit, and O for miss
    fn map(&self) {
        println!("Gamer's board status:");
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Water => print!("W "),
                    Cell::Ship(ShipType::Carrier { .. }) => print!("C "),
                    Cell::Ship(ShipType::Battleship { .. }) => print!("B "),
                    Cell::Ship(ShipType::Destroyer { .. }) => print!("D "),
                    Cell::Hit => print!("X "),
                    Cell::Miss => print!("O "),
                }
            }
            println!();
        }
    }
}

fn main() {
    //starting the game between two gamers
    StartGame();
}

fn StartGame() {
    //test steps for now, we will just create two gamers and print their boards. In the future, we will add more functionality such as placing ships, taking turns, etc.
    let mut gamer1 = Gamer::new();
    let mut gamer2 = Gamer::new();
    // Initialize the game board, place ships, etc.
    gamer1.board[0][0] = Cell::Ship(ShipType::Carrier { orientation: 'H', length: 5 });
    gamer2.board[0][0] = Cell::Ship(ShipType::Destroyer { orientation: 'H', length: 3 });
    gamer1.map();
    gamer2.map();
}
