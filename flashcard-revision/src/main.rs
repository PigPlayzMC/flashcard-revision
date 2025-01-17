use std::fs; // Handles reading and writing files

use macroquad::prelude::*; // Handles window display

use miniquad::window::dpi_scale;
use rusqlite::{ // Handles SQLite database
	params,
	Connection,
};

use toml::Table; // Handles TOML files for configuration and preferences

struct States {
	up: bool,
	add: bool,
	down: bool,
	settings: bool,
}

fn conf() -> Conf {
	// Load the icon at runtime
    let icon_small = Image::from_file_with_format(
		include_bytes!("./assets/images/stage_elements/icon_small.png"),
		Some(ImageFormat::Png)
	).unwrap();

	let icon_medium = Image::from_file_with_format(
		include_bytes!("./assets/images/stage_elements/icon_medium.png"),
		Some(ImageFormat::Png)
	).unwrap();

	let icon_big = Image::from_file_with_format(
		include_bytes!("./assets/images/stage_elements/icon_big.png"),
		Some(ImageFormat::Png)
	).unwrap();

	Conf {
		window_title: "Flashcard Application".to_owned(),
		fullscreen:false,
		high_dpi:true, // May cause issues in the future but yolo
		window_width:984,
		window_height:668,
		window_resizable:true,
		icon: Some(miniquad::conf::Icon {
			small: icon_small
				.bytes
				.clone()
				.try_into()
				.expect("Image size mismatch"),
			medium: icon_medium
				.bytes
				.clone()
				.try_into()
				.expect("Image size mismatch"),
			big: icon_big
				.bytes
				.clone()
				.try_into()
				.expect("Image size mismatch"),
		}),
		..Default::default()
	}
}

async fn loading_screen() {
	request_new_screen_size(984.0 , 668.0); // Ensures that the window is the correct size from start.

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

fn get_length(text: &str, font_size: u16, font: &Font) -> TextDimensions {
	let dimensions: TextDimensions = measure_text(
		text,
		Some(font),
		font_size,
		1.0);

	return dimensions;
}

async fn load_stage_element(file_name: &str) -> Texture2D {
	let path: String = format!("./src/assets/images/stage_elements/{}", file_name.to_string().trim());
	info!("Loading {0} from path: {1}", file_name, path.as_str());
	let result_ok: Texture2D;

	let mut result: Result<Texture2D, String> = match load_texture(&path).await {
		Ok(texture) => Ok(texture),
		Err(e) => {
			error!("Failed to load texture from path: {}. Error: {:?}", path, e);
			Err(format!("Load fallback texture")) // No semicolon *important*
		}
	};

	if let Err(e) = &result {
		info!("{}", e);
	}

	if result == Err("Load fallback texture".to_owned()) {
		info!("Attempting to load fallback texture");
		let recovery_path: String = format!("./src/assets/images/stage_elements/failed_to_load.png");
		// Hours spent trying to work out why path wasn't working without .png: 3
		info!("CRASH PREVENTION: Loading {0} from path: {1}", file_name, recovery_path.as_str());
		result = match load_texture(&recovery_path).await {
			Ok(texture) => Ok(texture),
			Err(_e) => {
				error!("Irrecoverable!!!");
				Err(format!("Don't delete textures."))
			}
		};
	};

	result_ok = result.unwrap();
	result_ok.set_filter(FilterMode::Linear);
	return result_ok
}

fn save_settings(settings: Table) {
	// Write settings to file
	fs::write("./src/settings.toml",
	toml::to_string(&settings)
	.expect("Cannot convert settings to string")
	.as_bytes())
	.expect("Cannot write settings to settings.toml");
}

fn get_subject_names(conn: Connection) -> Vec<String> {
	let mut stmt: rusqlite::Statement<'_> = conn.prepare("SELECT name FROM subjects;").unwrap();
	return stmt.query_map(params![], |row: &rusqlite::Row<'_>| {
		Ok(row.get::<_, String>(0)?)
	}).unwrap().map(|subject| subject.unwrap()).collect();
}

#[macroquad::main(conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// ## Icon ##
	let path: &str = "icon.icns";
	load_image(path);

	// ## User settings ##
	// Settings variables
	let settings: Table;
	let mut fullscreen: bool;
	let mut num_of_subjects: u16 = 0; // "If anyone needs more than 65,535 subjects, they... have a problem" - Copilot
	// ^^ Needs a default value to prevent uninitialized variable error ^^
	
	info!("Miniquad DPI: {}", dpi_scale());
	info!("Macroquad DPI: {}", screen_dpi_scale());

	// Struct to control whether buttons need to be gray or purple
	let states = States {
		up: false,
		down: false,
		add: false,
		settings: false,
	};

	// Application variables
	let mut text: &str;
	let mut texture_chosen: &Texture2D ;

	// Create or read settings file
	if !fs::exists("./src/settings.toml").expect("Cannot verify existence of settings.toml") {
		// Settings file does not exist :(
		info!("Creating settings file...");
		// Create settings file
		settings = toml::toml! {
			fullscreen = false
			number_of_subjects = 0
		};

		// Set settings
		fullscreen = settings.get("fullscreen").expect("Cannot retrieve fullscreen setting from settings.toml")
		.as_bool()
		.expect("Fullscreen setting is not a boolean");

		// Save settings to file
		save_settings(settings);
	} else {
		// Settings file exists :)
		info!("Loading settings.toml...");
		// Load settings file
		settings = toml::from_str(fs::read_to_string("./src/settings.toml").expect("Cannot read settings.toml").as_str()).expect("Cannot parse settings.toml");
		
		// Retrieve settings from db
		fullscreen = settings.get("fullscreen").expect("Cannot retrieve fullscreen setting from settings.toml")
		.as_bool()
		.expect("Fullscreen setting is not a boolean");

		num_of_subjects = settings.get("number_of_subjects").expect("Cannot retrieve number_of_subjects setting from settings.toml")
		.as_integer()
		.expect("Subject number setting is not an integer")
		as u16; //* Remember to change if number of subjects needs updating */
	}

	if fullscreen == true {
		set_fullscreen(true);
	}

	// Display loading screen
	loading_screen().await;

	// Load textures
	println!();
	info!("Loading textures...");

	// Main elements
	let header: Texture2D = load_stage_element("header.png").await;

	let flashcard_box: Texture2D = load_stage_element("flashcard_box.png").await;

	// Buttons
	let up_button: Texture2D = load_stage_element("up_button.png").await;
	let up_button_pressed: Texture2D = load_stage_element("up_button_pressed.png").await;

	let down_button: Texture2D = load_stage_element("down_button.png").await;
	let down_button_pressed: Texture2D = load_stage_element("down_button_pressed.png").await;

	let add_button: Texture2D = load_stage_element("add_button.png").await;
	let add_button_pressed: Texture2D = load_stage_element("add_button_pressed.png").await;

	let settings: Texture2D = load_stage_element("settings_button.png").await;
	let settings_pressed: Texture2D = load_stage_element("settings_button_pressed.png").await;

	info!("Texture load complete!");
	println!();

	// Font
	let open_sans_reg: Font = load_ttf_font("./src/assets/fonts/OpenSans-Regular.ttf").await.unwrap();

	// ## SQLite database ##
	let conn: Connection = Connection::open("flashcards.db")?; // Creates/opens database

	// Storage of tables
	let _ = conn.execute(
		"CREATE TABLE IF NOT EXISTS subjects (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL,
			date_weak_revised INTEGER NOT NULL,
			date_learning_revised INTEGER NOT NULL,
			date_strong_revised INTEGER NOT NULL
		);", // Stores dates as seconds since epoch
		params![],
	);

	// ## Window settings ##
	// Subject settings
	let subjects: Vec<String> = get_subject_names(conn);
	// ^^ This will need updating when the database is updated later in the program ^^
	let page: i32 = 0; // This allows for one page per subject so should not be too small
	let subjects_per_page: i32 = 6; // Must update to be based on screen size

	let mut creating_subject: bool = false;

	/* Stage settings
	0 = Subject selection/Settings, 1 = Changing settings,
	2 = Revision, 3 = Results, 4 = Add/Remove flashcards,
	5 = Edit flashcards */
	let mut stage: u8 = 0;
	
	// General colours
	let background_colour: Color = Color::from_rgba(0, 0, 0, 255); //rgb(0, 0, 0)
	let text_colour: Color = Color::from_rgba(222, 222, 222, 255); //rgb(222, 222, 222)
	let bounding_box: Color = Color::from_rgba(0, 80, 27, 255);    //rgb(0, 80, 27)
	// ^^ Alpha must be set to 0 in production ^^

	// Card colours
	////let weak_colour = todo!();
	////let learning_colour = todo!();
	////let strong_colour = todo!();

	// ## Debug variable displays ##
	println!();
	info!("Settings:");
	info!("Fullscreen: {}", fullscreen);
	info!("Number of subjects: {}", num_of_subjects);
	println!();

	// ## Main loop ##
	debug!("Main loop reached...");
	loop {
		clear_background(background_colour);

		// Debug fps statement
		draw_text(&get_fps().to_string(), 20.0, 20.0, 20.0, text_colour);

		if stage == 0 {
			// # Select subjects #
			
			// # Settings button #

			// # Draw header #
			draw_texture_ex(
				&header,
				screen_width()/2.0 - 460.0,
				0.0,
				WHITE,
				DrawTextureParams {
					source: Some(Rect::new(0.0, 0.0, 1840.0, 160.0)), // Use the full size of the texture
					dest_size: Some(Vec2::new(920.0, 80.0)), // Resize to fit 920 by 728
					..Default::default()
				},
			);
			
			// # Subject list box #
			draw_texture_ex(
				&flashcard_box,
				screen_width()/2.0-460.0,
				screen_height()/2.0-364.0,
				WHITE,
				DrawTextureParams {
					source: Some(Rect::new(0.0, 0.0, 1840.0, 1456.0)), // Use the full size of the texture
					dest_size: Some(Vec2::new(920.0, 728.0)), // Resize to fit 920 by 728
					..Default::default()
				},
			);

			// # Up button #
			if states.up == true {
				texture_chosen = &up_button_pressed;
			} else {
				texture_chosen = &up_button
			}
			draw_texture_ex(
				texture_chosen,
				600.-180.,
				screen_height()-screen_height()*17./108., 
				WHITE,
				DrawTextureParams {
					source: Some(Rect::new(0.0,0.0,120.0,120.0)),
					..Default::default()
				});

			let mut index: i32 = 0; // People and SQLite3 start counting from 1 but for formatting 0 is required
			for subject in &subjects {
				if index >= page * subjects_per_page {
					if index < page * subjects_per_page + subjects_per_page {
						let subject_text: String = (index + 1).to_string() + ". " + subject;
						text = &subject_text;

						let centre: Vec2 = get_centre(
							open_sans_reg.clone(),
							40,
							text,
						);

						////info!("{}", centre.y);
						let offset: f32 = index as f32 * 62.5 + 200.0;
						// ^^ centre.y is a negative value ^^

						// Display each subject's name
						draw_text_ex(
							&text,
							screen_width() / 2.0 - centre.x,
							offset,
							TextParams {
								font: Some(&open_sans_reg),
								font_size: 40,
								color: text_colour,
								..Default::default()},
						);
						
						index += 1;
					}
				}
			} // End of subject display loop

			// # Forward/Back buttons #
			if num_of_subjects > 6 { // 6 subjects is maximum for display
				// Placeholder values
				todo!();
			} // Otherwise don't display

			// # Create new subject button #
			

			// Code for creating new subject
			
			// Handle edge case
			if creating_subject == true {
				if num_of_subjects - 65535 == 0 {
					error!("Cannot create subject: Maximum number (65,535) of subjects reached.");
				} else {
					// Create a subject
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