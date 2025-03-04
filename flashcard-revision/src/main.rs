use std::{fs, time::SystemTime}; // Handles reading and writing files

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

async fn load_icon_element(file_name: &str) -> Texture2D {
	let path: String = format!("./src/assets/images/icons/{}", file_name.to_string().trim());
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

fn subject_exists(subject_number: u16, page: i32, subjects_per_page: i32, subjects: &Vec<String>) -> bool {
	if subject_number + (page as u16 * subjects_per_page as u16) <= subjects.len() as u16 {
		// Subject exists as it is less than or equal to the length of the subject list
		true
	} else {
		// Subject does not exist :(
		false
	}
}

#[macroquad::main(conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
	let mut texture_chosen: &Texture2D;
	let mut width: f32;
	let mut height: f32;
	let mut loading: bool = true;

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
	let time_started = SystemTime::now();// Not a quick program
	loading_screen().await;

	// Load textures
	println!();
	info!("Loading textures...");

	// Main elements
	// Stages
	let stage0_no_blank: Texture2D = load_stage_element("stage0_no_blank.png").await;
	let stage0_arrows_blank: Texture2D = load_stage_element("stage0_arrows_blank.png").await;

	// Icons
	let settings_notification: Texture2D = load_icon_element("settings_notification.png").await;

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
	let subjects_per_page: i32 = 6;
	let mut creating_subject: bool = false;

	/* Stage settings
	0 = Subject selection/Settings, 1 = Changing settings,
	2 = Revision, 3 = Results, 4 = Add/Remove flashcards,
	5 = Edit flashcards */
	let mut stage: u8 = 0;
	
	// General colours
	let background_colour: Color = Color::from_rgba(0, 0, 0, 255); //rgb(0, 0, 0)
	let text_colour: Color = Color::from_rgba(0, 0, 0, 255); //rgb(222, 222, 222)
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
	// Info environment statements
	////info!("Screen width: {}", screen_width()); // On my machine: 984 by 668
	////info!("Screen height: {}", screen_height());
	println!();

	// ## Main loop ##
	loading = false;
	let time_loaded = SystemTime::now()
		.duration_since(time_started)
		.expect("Time went backwards");
	info!("Program loaded in {:?} seconds", time_loaded);
	debug!("Main loop reached...");
	debug!("");
	debug!("");
	loop {
		// Info environment statements
		////info!("Screen width: {}", screen_width()); // On my machine: 1440 by 900
		////info!("Screen height: {}", screen_height());

		clear_background(background_colour);

		if stage == 0 {
			// # Forward/Back buttons #
			if num_of_subjects > 6 { // 6 subjects is maximum for display
				// Display buttons in purple
				texture_chosen = &stage0_no_blank;
			} else { // Otherwise don't display
				// Display buttons in gray
				texture_chosen = &stage0_arrows_blank;
			}
			// # Draw stage #
			width = (3840.0/1920.0*screen_width())/2.0;
			// Original width to height ratio = 3840:2160 = 16:9
			// So height must equal width/16*9
			// Could also consider doing this based on height as this seems to be the limiting
			// factor on my display
			height = width/16.0*9.0;
			draw_texture_ex(
				&texture_chosen,
				screen_width()/2.0 - width/2.0,
				0.0,
				WHITE,
				DrawTextureParams {
					source: Some(Rect::new(0.0, 0.0, 3840.0, 2160.0)), // Use the full size of the texture
					dest_size: Some(Vec2::new(width, height)),
					..Default::default()
				},
			);// End of subject display loop

			// Debug variable displays
			draw_text(&get_fps().to_string(), 20.0, 20.0, 20.0, WHITE);
			draw_text(&mouse_position().0.to_string(), 20.0, 100.0, 20.0, WHITE);
			draw_text(&mouse_position().1.to_string(), 20.0, 150.0, 20.0, WHITE);
			
			// # Display subjects #
			let mut sub_number: usize = (0 + (page) * subjects_per_page) as usize;
			// Check that all 6 subjects can be drawn

			for _ in (0+page*subjects_per_page)..(page*subjects_per_page+subjects_per_page) {
				if sub_number >= subjects.len() {
					////info!("Not displaying subject with number {} due to lack of existence...", sub_number);
					break;
				} else {
					////info!("Drawing subject with number {}", sub_number);
					draw_text_ex(
						subjects[sub_number].as_str(),
						405., // 405 for all items on my machine
						180., // 180 for first item on my machine
						TextParams {
							font: Some(&open_sans_reg),
							font_size: (40),
							////font_scale: (),
							////font_scale_aspect: (),
							color: (text_colour),
							..Default::default()
						},
					);
					sub_number += 1;
				}
			}

			// # Check mouse collisions #
			if is_mouse_button_pressed(MouseButton::Left) {
				info!("[E] Mouse click registered at {:?}", mouse_position());
				// Check if mouse if on the subject box
				if mouse_position().0 >= (378.) { // Subject box inner bounds
					if mouse_position().1 >= 128. {
						// On my machine
						// May have to stop nesting these statements
						if mouse_position().0 <= (1068.) { // Subject box outer bounds
							if mouse_position().1 <= (670.) {
								// ^^ Y coord doesn't seem to be used correctly ^^
								// Maybe 4 if statements would fix this issue?
								// Also on my machine
								info!("[H] Mouse click indentified as within subject box");
								// Identify which subject was clicked
								// 670/6*1+128 is the wrong number. Around 239.66666666666667
								// Actual desired number is around 202.5 for the first number
								// Actual desired number is around 282.5 for the second number
								// This jump is 80
								// The actual jump in the program is 111.6666666666667
								// The program jump is off by 31.6666666666667
								// This can easily be rectified by switching to 80 (On my machine)
								if mouse_position().1 <= (80. * 1. + 128.) {
									// Subject one
									info!("[H] Mouse click identified as subject one");
									if subject_exists(1, page, subjects_per_page, &subjects) {
										info!("[H] Subject click handled as subject exists");
										// Subject clicked and must now be handled
									} else {
										info!("[H] Subject click not handled as subject exists");
									}

								} else if mouse_position().1 <= (80. * 2. + 128.) {
									// Subject two
									info!("[H] Mouse click identified as subject two");
									if subject_exists(1, page, subjects_per_page, &subjects) {
										info!("[H] Subject click handled as subject exists");
										// Subject clicked and must now be handled
									} else {
										info!("[H] Subject click not handled as subject exists");
									}

								} else if mouse_position().1 <= (80. * 3. + 128.) {
									// Subject three
									info!("[H] Mouse click identified as subject three");
									if subject_exists(1, page, subjects_per_page, &subjects) {
										info!("[H] Subject click handled as subject exists");
										// Subject clicked and must now be handled
									} else {
										info!("[H] Subject click not handled as subject exists");
									}
									
								} else if mouse_position().1 <= (80. * 4. + 128.) {
									// Subject four
									info!("[H] Mouse click identified as subject four");
									if subject_exists(1, page, subjects_per_page, &subjects) {
										info!("[H] Subject click handled as subject exists");
										// Subject clicked and must now be handled
									} else {
										info!("[H] Subject click not handled as subject exists");
									}
									
								} else if mouse_position().1 <= (80. * 5. + 128.) {
									// Subject five
									info!("[H] Mouse click identified as subject five");
									if subject_exists(1, page, subjects_per_page, &subjects) {
										info!("[H] Subject click handled as subject exists");
										// Subject clicked and must now be handled
									} else {
										info!("[H] Subject click not handled as subject exists");
									}
									
								} else if mouse_position().1 <= (80. * 6. + 128.) { // This bound is too large?
									// Subject six
									info!("[H] Mouse click identified as subject six");
									if subject_exists(1, page, subjects_per_page, &subjects) {
										info!("[H] Subject click handled as subject exists");
										// Subject clicked and must now be handled
									} else {
										info!("[H] Subject click not handled as subject exists");
									}
									
								} else {
									error!("[E] Mouse click not identified as any subject despite being within subject box");
								}
							}
						}
					}
				}
			}

			// Handle edge case
			// Not needed in this stage but will be needed in future stages
			if creating_subject == true {
				if num_of_subjects - 65535 == 0 {
					error!("Cannot create subject: Maximum number (65,535) of subjects reached.");
				} else {
					// Create a subject
				}
			}

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

		// Debug window statements
		////info!("Screen width: {}", screen_width());
		////info!("Screen height: {}", screen_height());
		////info!("Mouse position: {:?}", mouse_position());

		// End section (Nothing past this point please)
		next_frame().await;
	}
	
	////Ok(())
}