mod lib;
use lib::actions::Action;

pub struct Game {
    count: i32,
    players: i32
}

impl Action for Game {
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

    let object1 = lib::objects::create("test".to_string());
    lib::objects::remove(&object1);
    println!("{}", object1.exists);

    game.action()
}   
