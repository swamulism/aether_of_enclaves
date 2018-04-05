use piston_window::{ButtonState, Key};

pub trait Moveable {
    fn handle_input(&mut self, state: ButtonState, key: Option<Key>);
    // fn collision(&mut self, game: &Game) -> bool;
    fn update_position(&mut self);
    fn update_self_velocity(&mut self);
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/*
player inventory 
array size 3 or vec size 3?
queue:
pop_front
push_back

stack:
pop_back
push_back
*/

/*
game state can be implemented in 2 ways
stack
or 
Deterministic finite automaton / Finite State Machine

{
    private enum States { state_0, state_1, state_2 };
    private States state_now;

    void Start () {
        state_now = States.state_0;
    }

    void Update () {
        switch (state_now) {
            case States.state_0 : state_0(); break;
            case States.state_1 : state_1(); break;
            case States.state_2 : state_2(); break;
            default : break;
        }

    void state_0() {

        // Business logic
        state_now = States.state_1;
    }

    // Others function, one for each state
}

*/

/*

maybe have states in creature for what is currently happening

Enum:
Still
Moving
Jumping
Attacking
Throwing
Interacting

*/