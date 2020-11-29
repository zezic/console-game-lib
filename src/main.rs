mod lib;
use lib::actions::SimpleAction;

pub struct Game {
    count: i32,
    players: i32
}

impl SimpleAction for Game {
    fn action(&self) {
        println!("this is an action!");
        println!("count: {}, players: {}", self.count, self.players);
    }
}

fn main() {
    let game = Game {
        count: 8,
        players: 2,
    };

    game.action()
}   
