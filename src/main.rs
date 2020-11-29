mod lib;

pub struct Game {
    count: i32,
    players: i32
}

impl lib::actions::SimpleAction for Game {
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