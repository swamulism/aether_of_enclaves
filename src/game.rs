// use user_interface::{Interface};
use piston_window::*;
use player::{Player};
use input_handler::{InputHandler};

pub enum GameState { Title, Main, InMenu }
use self::GameState::*;

// Global Game State variable.
// pub static mut GAME_STATE: GameState = Title;
// pub static mut PLAYER: Player = Player { x: 0.0, y: 0.0 };


/**
	Implementation of the Game object.

	@field u_i The user interface.
*/
pub struct Game {
	window: PistonWindow,
	input_hnd: InputHandler,
	player: Player,
	game_state: GameState,
}

impl Game {
	// Constructor of the Game.
	pub fn new() -> Self {
		Game {
			window: WindowSettings::new("AOE", (400, 400))
				.exit_on_esc(true)
		        .build()
		        .unwrap(),
		    input_hnd: InputHandler::new(),
			player: Player::new(),
			game_state: GameState::Title
		}
	}

	// The game loop.
	pub fn run(&mut self) {
		// while self.ui.update() {}

		while let Some(e) = self.window.next() {
	         match e {
	            Event::Input(Input::Button(args)) => {
	                // if args.button == Button::Keyboard(Key::W) { player.move_player(Direction::Up); }
	                // if args.button == Button::Keyboard(Key::S) { player.move_player(Direction::Down); }
	                // if args.button == Button::Keyboard(Key::A) { player.move_player(Direction::Left); }
	                // if args.button == Button::Keyboard(Key::D) { player.move_player(Direction::Right); }
					self.input_hnd.handle_input(args.button, &mut self.player, &mut self.game_state);
	            }

				Event::Loop(Loop::Update(args)) => {
	                //Update Events
	            }

	            Event::Loop(Loop::Render(args)) => {
	                // window.draw_2d(&e, |context, graphics| {
	                //     clear([0.0, 0.0, 0.0, 1.0], graphics);
	                //     let red = [1.0, 0.0, 0.0, 1.0];
	                //     let square = [player.x_pos, player.y_pos, 100.0, 100.0];
	                //     rectangle(red, square, context.transform, graphics);
	                // });

					self.window.draw_2d(&e, |context, graphics| {
				        clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
						match self.game_state {
							Main => {
								// Draw Player.
								let red = [1.0, 0.0, 0.0, 1.0];
			       				let player_image = [self.player.x, self.player.y, 15.0, 15.0];
			       				rectangle(red, player_image, context.transform, graphics);
							},
							Title => {

							},
							_ => {}
						}
			   	    });
	            }

	            _ => {}
	        }
	    }
	}

	// pub fn update(&mut self) -> bool {
	// 	while let Some(e) = self.window.next() {
	//         if let Some(button) = e.press_args(){
	//             self.input_hnd.handle_input(button);
	//
	//             // Need to manually implement exit on escape.
	//             if button == Button::Keyboard(Key::Escape) {
	// 	        	return false;
	// 	        }
	//         }
	//         self.display(e);
	//         break;
	//     }
	//     true
	// }

	// pub fn display(&mut self, e: Event) {
	// 	self.window.draw_2d(&e, |context, graphics| {
	//         clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
	// 		match game_state {
	// 			Main => {
	// 				// Draw Player.
	// 				let red = [1.0, 0.0, 0.0, 1.0];
    //    				let player_image = [PLAYER.x, PLAYER.y, 15.0, 15.0];
    //    				rectangle(red, player_image, context.transform, graphics);
	// 			},
	// 			Title => {
	//
	// 			},
	// 			_ => {}
	// 		}
   	//     });
	// }
}
