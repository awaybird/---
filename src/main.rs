use std::collections::hash_set::SymmetricDifference;

#[derive(Debug)]
enum Direction{
    E,
    S,
    W,
    N
}

enum PlayerAction{
    Wait,
    Attack(Direction),
    Move{
        direction:Direction,
        speed: u8
    }


}

fn main(){
    let sim_player_action = PlayerAction::Move { direction:Direction::N , speed: 8 };
    match sim_player_action{
        PlayerAction::Wait => println!("player want's have a rest."),
        PlayerAction::Attack(direction) => println!("player want's to attack dirction {:?}", direction),
        PlayerAction::Move { direction, speed } => 
        println!("player want's to move in direction {:?} for speed {}", direction, speed),
    }
}