use macroquad::prelude::*; // Handles window display

use rusqlite::{ // Handles SQLite database
	Connection,
	params};

fn conf() -> Conf {
	Conf {
		window_title: "Flashcard Application".to_owned(),
		fullscreen:false,
		..Default::default()
	}
}

async fn loading_screen(fullscreen: bool) {
	request_new_screen_size(984.0 , 668.0); // Ensures that the window is the correct size from start.

	if fullscreen == true {
		set_fullscreen(true);
	}

	debug!("Loading...");
	clear_background(Color::from_rgba(0, 0, 0, 1));
	draw_text("Loading...", screen_width() / 2.0 - 40.0, screen_height() / 2.0, 50.0, WHITE);
	next_frame().await;
}

fn get_centre(font: Font, font_size: u16, text: &str) -> Vec2 {
	let centre = get_text_center(
		text,
		Some(&font),
		font_size,
		1.0,
		0.0);

	return centre;
}

#[macroquad::main(conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Retrieve settings from db
	let fullscreen: bool = false;

	// Display loading screen
	loading_screen(fullscreen).await;

	// Font
	let open_sans_reg: Font = load_ttf_font("./src/assets/OpenSans-Regular.ttf").await.unwrap();

	// ## SQLite database ##
	let conn: Connection = Connection::open("flashcards.db")?; // Creates/opens database

	// Storage of tables
	let _ = conn.execute(
		"CREATE TABLE IF NOT EXISTS subjects (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL,
			date_weak_revised INTEGER NOT NULL,
			date_learning_revised INTEGER NOT NULL,
			date_strong_revised INTEGER NOT NULL,
		);", // Stores dates as seconds since epoch
		params![],
	);

	// ## Window settings ##
	// Subject settings
	let mut num_of_subjects: u16 = 0; // Update to be based on db ("If anyone needs more than 65,535 subjects, they have a problem" - Copilot)
	let mut creating_subject: bool = false;

	/* Stage settings
	0 = Subject selection/Settings, 1 = Changing settings,
	2 = Revision, 3 = Results, 4 = Add/Remove flashcards,
	5 = Edit flashcards */
	let mut stage: u8 = 0;
	
	// General colours
	let background_colour: Color = Color::from_rgba(0, 0, 0, 255);
	let text_colour: Color = Color::from_rgba(222, 222, 222, 255);

	// Card colours
	////let weak_colour = todo!();
	////let learning_colour = todo!();
	////let strong_colour = todo!();

	// ## Main loop ##
	debug!("Main loop reached...");
	loop {
		clear_background(background_colour);

		if stage == 0 {
			// # Select subjects #
			
			// # Settings button #

			// # Subject selection box #
			let centre: Vec2 = get_centre(
				open_sans_reg.clone(),
				40,
				"Select a subject from the list or create a new one!",
			);

			// # Subject selection instructions #
			draw_text_ex(
				"Select a subject from the list or create a new one!",
				screen_width()/2.0 - centre.x,
				100.0,
				TextParams {
					font: Some(&open_sans_reg),
					font_size: 40,
					color: text_colour,
					..Default::default()
				},
			);
			
			// # Subject list box #
			draw_rectangle(0.0, 100.0, 120.0, 60.0, GREEN);

			// # Subjects #

			// # Forward/Back buttons #
			if num_of_subjects > 10 {
				// Placeholder values
				todo!();
			} // Otherwise don't display

			// # Create new subject button #
			// Code for creating new subject
			
			// Handle edge case
			if creating_subject == true {
				if num_of_subjects - 65535 == 0 {
					error!("Cannot create subject: Maximum number (65,535) of subjects reached.");
				}
			}

			// # Handle clicks to move stages #

		} else if stage == 1 {
			// Change settings
		} else if stage == 2 {
			// Revision
		} else if stage == 3 {
			// Results
		} else if stage == 4 {
			// Add/Remove flashcards
		} else if stage == 5 {
			// Edit flashcards
		} else {
			panic!("ERROR 1: Invalid stage number");
		}

		////draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

		////draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

		// Debug window statements
		////info!("Screen width: {}", screen_width());
		////info!("Screen height: {}", screen_height());

		// End section (Nothing past this point please)
		next_frame().await;
	}
	
	////Ok(())
}