mod env;

use std::cell::RefCell;
use ic_cdk::{query,update,init};
use serde::Deserialize;
use candid::CandidType;
use crate::env::{CanisterEnv, EmptyEnv, Environment};



thread_local! {
    // If RuntimeState doesn't implement Default you can wrap it in an Option instead
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

#[derive(CandidType, Deserialize)]
enum Direction {
    N, NE, E, SE, S, SW, W, NW, NNW,
}
#[derive(CandidType, Deserialize)]
enum Errors {
    Invalid,
    InvalidDirection
}


struct RuntimeState {
    pub data: Data,
    pub env: Box<dyn Environment>,
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnv {}),
            data: Data::default(),
        }
    }
}

#[init]
fn init() {
    let env = Box::new(CanisterEnv::new());
    let data = Data::default();
    let runtime_state = RuntimeState { env, data };

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}


#[derive(CandidType, Deserialize, Clone)]
struct Target {
    x : i32,
    y : i32,
}
#[derive(CandidType, Deserialize, Clone,Default)]
struct Data {
    player: Vec<Player>,
    coordinate: Vec<Coordinate>,
    target : Vec<Target>
}

#[derive(CandidType, Deserialize, Copy, Clone, Default)]
struct Coordinate{
    pub x:  i32,
    pub y : i32,
}
#[derive(CandidType, Deserialize, Clone)]
struct Player {
    name: String,
    coordinate: Coordinate,
    energy : i32,
    coin : u32,
    target : Target
}





#[update]
fn create_player() -> Result<(), Errors> {
    RUNTIME_STATE.with(|state| crate_player_imp(&mut state.borrow_mut()))
}

fn crate_player_imp(runtime_state: &mut RuntimeState) -> Result<(), Errors> {
    if !runtime_state.data.player.is_empty() {
        // If there's already a player created, return an error
        return Err(Errors::Invalid);
    }
    let id = runtime_state.env.random_u32();
    let id2 = runtime_state.env.random_u32();

    let target_for_player = Target{x : id as i32, y : id2 as i32 };
    let point = Coordinate { x: 0, y: 0 };
    runtime_state.data.player.push(Player {
        name: "unnamed".to_string(),
        coordinate: point,
        coin: 0,
        energy: 200,
        target: target_for_player,
    });
    Ok(())
}



#[update]
fn player_move(direction: Direction) -> Result<(), Errors> {
    RUNTIME_STATE.with(|state| player_move_imp(direction, &mut state.borrow_mut()))
}

fn player_move_imp(direction: Direction, runtime_state: &mut RuntimeState) -> Result<(), Errors> {
    let (dx, dy) = match direction {
        Direction::N => (0, 1),
        Direction::NE => (1, 1),
        Direction::E => (1, 0),
        Direction::SE => (1, -1),
        Direction::S => (0, -1),
        Direction::SW => (-1, -1),
        Direction::W => (-1, 0),
        Direction::NW => (-1, 1),
        Direction::NNW => (-1, 2), // Example of custom direction

    };

    for player in &mut runtime_state.data.player {
        // Update player's coordinates based on the direction
        player.coordinate.x += dx;
        player.coordinate.y += dy;
    }

    Ok(())
}

#[query]
fn player_loc()->Vec<Coordinate>{
    RUNTIME_STATE.with(|state| get_playet_loc_imp(&mut state.borrow_mut()))
}
fn get_playet_loc_imp (runtime_state: &RuntimeState) -> Vec<Coordinate>{
    runtime_state
        .data
        .player
        .iter()
        .map(|player| player.coordinate ).clone()
        .collect()

}

#[query]
fn get_target() -> Vec<Target> {
    RUNTIME_STATE.with(|state| get_target_imp(&state.borrow()))
}

fn get_target_imp(runtime_state: &RuntimeState) -> Vec<Target> {
    runtime_state
        .data
        .player
        .iter()
        .map(|player| player.target.clone())
        .collect()
}










