use macroquad::prelude::*;

async fn loading_screen() {
	clear_background(Color::from_rgba(0, 0, 0, 1));
	draw_text("Loading...", screen_width() / 2.0 - 40.0, screen_height() / 2.0, 50.0, WHITE);
	next_frame().await;
}

#[macroquad::main("Flashcard Revision")]
async fn main() {
	// Display loading screen
	loading_screen().await;

	// ##  ##
	// General colours
	let background_colour: Color = Color::from_rgba(88, 138, 85, 1);
	////let text_colour = todo!();

	// Card colours
	////let weak_colour = todo!();
	////let learning_colour = todo!();
	////let strong_colour = todo!();

    loop {
        clear_background(background_colour);

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        ////draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);


		// End section (Nothing past this point please)
        next_frame().await;
    }
}