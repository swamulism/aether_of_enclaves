/**
	Input Handler uses *inheritance to handle the sets of
	commands which might be inputted from the Game and user.

	Modeled with the Command programming design pattern.

	*Rust doesn't truly support inheritance & classes, so this
	has been manually recreated to the best of our ability.
*/

use piston_window::{Button,Key};
use game::{GameState, GAME_STATE, PLAYER};
use self::GameState::*;

#[derive(Debug)]

// Used for Move sub-struct.
pub enum Direction { Up, Down, Left, Right, Nil }


/** 
	Ancestor object for different types of input Commands. 
*/
trait Command {
	// Initializer for Command.
	fn new() -> Self;	

	// Execute actions based on type of Command.
	// @param Option<Key> And optional key value.
	fn execute(&mut self, Option<Key>);
}


/**
	Implementation of the OpenMenu Command.
*/
struct OpenMenu {}

impl OpenMenu { pub fn new() -> Self { OpenMenu {} } }

impl Command for OpenMenu {
	fn new() -> Self { OpenMenu {} }

	// Unused param _key.
	fn execute(&mut self, _key: Option<Key>) {
		unsafe {
			match GAME_STATE {
				Main => {
					println!("Menu opened.");
					GAME_STATE = InMenu;
				},
				InMenu => {
					println!("Menu closed.");
					GAME_STATE = Main;
				}
				_ => {}
			}
		}
	}
}


/**
	Implementation of the Action Command.
	Used when the player presses the action button (Return).
*/
struct Action {}

impl Action { pub fn new() -> Self { Action {} } }

impl Command for Action {
	fn new() -> Self { Action {} }

	// Unused param _key.
	fn execute(&mut self, _key: Option<Key>) {
		unsafe {
			match GAME_STATE {
				Title => {
					println!("Changing state to Main.");
					GAME_STATE = Main;
				},
				_ => {}
			}
		}
	}
}


/**
	Implementation of the Move Command.
	This will either move the player or move selections
	in a menu.

	@field dir The direction of the Command.
*/
struct Move {}

impl Move {
	pub fn new() -> Self { Move { } }
}

impl Command for Move {
	fn new() -> Self { Move { } }

	// @param key The input key.
	fn execute(&mut self, key: Option<Key>) {
		use self::Direction::*;
		let mut dir = Nil;
		match key {
			Some(Key::W) => dir = Up,
			Some(Key::A) => dir = Left,
			Some(Key::S) => dir = Down,
			Some(Key::D) => dir = Right,
			_ => {},
		}
		println!("Moving {:?}.", dir);
		unsafe { PLAYER.update_position(dir, 15.0); }
	}
}


/**
	Implementation of the Input Handler.
	Executes respective Commands given Button input.

	@field move_dir The Move Command handler.
	@field action The Action Command handler.
	@field open_menu The OpenMenu Command handler.
*/
pub struct InputHandler {
	move_dir: Move,
	action: Action,
	open_menu: OpenMenu,
}

impl InputHandler {
	// Constructor.
	pub fn new() -> Self { 
		InputHandler {
			move_dir: Move::new(),
			action: Action::new(),
			open_menu: OpenMenu::new(),
		} 
	}

	// @param button The input button arguments.
    pub fn handle_input(&mut self, button: Button) {
    	use self::Key::*;
    	match button {
    		Button::Keyboard(key) => {
    			match key {
    				Return => self.action.execute(None),
	    			Tab => self.open_menu.execute(None),
	    			W | A | S | D => self.move_dir.execute(Some(key)),
	    			_ => {}
    			}
	    	},
	    	_ => {},
    	}
    }
}