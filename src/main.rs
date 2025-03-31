extern crate piston_window;
extern crate find_folder;

use piston_window::*;

// width of the window where the game is played
const WIDTH: f64 = 800.0;
// height of the window where the game is played
const HEIGHT: f64 = 400.0;
// height of the player and CPU paddles
const PADDLE_HEIGHT: f64 = 80.0;
// width of the player and CPU paddles
const PADDLE_WIDTH: f64 = 10.0;
// height and width of the ball
const BALL_SIZE: f64 = 10.0;
// the constant modifier for how fast the paddles are allowed to move across the screen
// should remain under 1 to allow for there to always be a possibility to miss the ball
const PADDLE_SPEED: f64 = 0.9;
// constant modifier for how fast the ball can move across the screen
// should remain above PADDLE_SPEED to allow the possibility of misses from just being too slow
const BALL_SPEED: f64 = 1.0;

// structure for the game statistics
struct Game {
    player_y: f64, // the y position of the player paddle
    ai_y: f64, // y position of the CPU paddle
    ball_x: f64, // x position of the ball
    ball_y: f64, // y position of the ball
    ball_dx: f64, // change in x position of the ball
    ball_dy: f64, // change in y position of the ball
}

impl Game {
    fn new() -> Self {
        Game {
            // sets player paddle to be in the middle of the window's height
            player_y: (HEIGHT - PADDLE_HEIGHT) / 2.0,
            // sets CPU paddle to be in the middle of the window's height
            ai_y: (HEIGHT - PADDLE_HEIGHT) / 2.0,
            // set the ball to be in the middle of the window's width
            ball_x: WIDTH / 2.0,
            // set the ball to be in the middle of the window's height
            ball_y: HEIGHT / 2.0,
            // set the ball speed for the x direction
            ball_dx: BALL_SPEED,
            // set the ball speed for the y direction
            ball_dy: BALL_SPEED,
        }
    }

    // function to update the positions of the various assets within the window
    fn update(&mut self) {
        // update ball position
        self.ball_x += self.ball_dx; // make it go side to side
        self.ball_y += self.ball_dy; // make it go up and down

        // Bounce ball off top and bottom walls
        if self.ball_y <= 0.0 || self.ball_y >= HEIGHT - BALL_SIZE {
            self.ball_dy = -self.ball_dy; // send it in the opposite direction
        }

        // bounce ball off paddles
        // checks if the ball and the paddle are both in the same location in the window
        // first if statement checks for the player's paddle
        // the else if checks for the CPU paddle
        if self.ball_x <= PADDLE_WIDTH
            && self.ball_y >= self.player_y
            && self.ball_y <= self.player_y + PADDLE_HEIGHT
        {
            self.ball_dx = -self.ball_dx; // send it in the opposite direction
        } else if self.ball_x >= WIDTH - PADDLE_WIDTH - BALL_SIZE
            && self.ball_y >= self.ai_y
            && self.ball_y <= self.ai_y + PADDLE_HEIGHT
        {
            self.ball_dx = -self.ball_dx; // send it in the opposite direction
        }

        // reset ball if it goes out of bounds
        if self.ball_x < 0.0 || self.ball_x > WIDTH {
            self.ball_x = WIDTH / 2.0; // reset x position to default
            self.ball_y = HEIGHT / 2.0; // reset y position to default
        }

        self.update_ai(); // call the update_ai(&mut self) method, as shown below
    }

    // function to update the CPU paddle position during the game
    fn update_ai(&mut self) {
        let ai_speed = PADDLE_SPEED; // set the speed

        // move the CPU paddle along with the ball
        if self.ai_y + PADDLE_HEIGHT / 2.0 < self.ball_y {
            self.ai_y += ai_speed;
        } else if self.ai_y + PADDLE_HEIGHT / 2.0 > self.ball_y {
            self.ai_y -= ai_speed;
        }

        // prevent CPU paddle from going out of bounds
        if self.ai_y < 0.0 {
            self.ai_y = 0.0;
        }
        if self.ai_y > HEIGHT - PADDLE_HEIGHT {
            self.ai_y = HEIGHT - PADDLE_HEIGHT;
        }
    }

    // creating the game space and all the in game assets (paddles, ball)
    // takes 2 arguments other than self: Context and g
    // Context allows for the translation of the rectangles and the ball.
    // necessary for context.transform
    // g is of the type G2d, which is a 2D graphics engine object. It is what actually allows
    // us to draw the elements we want on the screen
    fn render(&self, context: Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g); // black background

        // player paddle asset
        rectangle(
            [1.0, 1.0, 1.0, 1.0], // sets it to white
            [0.0, self.player_y, PADDLE_WIDTH, PADDLE_HEIGHT], // define size and position
            context.transform, // move it as necessary
            g, // draw it on the window
        );

        // CPU paddle asset
        rectangle(
            [1.0, 1.0, 1.0, 1.0], // sets it to white
            [WIDTH - PADDLE_WIDTH, self.ai_y, PADDLE_WIDTH, PADDLE_HEIGHT], // size, position
            context.transform, // move as necessary
            g, // draw it on the window
        );

        // ball asset
        ellipse(
            [1.0, 1.0, 1.0, 1.0], // sets it to white
            [self.ball_x, self.ball_y, BALL_SIZE, BALL_SIZE], // size and position
            context.transform, // move as necessary
            g, // draw it on the window
        );
    }
}

// main function for running the program
fn main() {
    // create the window for the game
    let mut window: PistonWindow = WindowSettings::new("Pong Game", [WIDTH, HEIGHT])
        .exit_on_esc(true) // allows you to hit 'ESC' to exit the window
        .build()
        .unwrap(); // build window and handle errors

    // initialize the game state with default positions:
        // paddles at half the height of the window
        // ball at center of the window
    let mut game = Game::new();

    // variables to track the status of buttons pressed.
    // this allows for smooth movement of the paddles throughout the game.
    let mut player_up_pressed = false;
    let mut player_down_pressed = false;

    // main game loop that runs until window is closed
    while let Some(event) = window.next() {
        // event handler for a key being pressed down
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => player_up_pressed = true, // if up arrow is pressed, set to true
                Key::Down => player_down_pressed = true, // if down arrow is pressed, set true
                _ => {} // this tells it to ignore other key presses
            }
        }

        // event handler for when a key is released (essential for continuous movement
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Up => player_up_pressed = false, // if up arrow is released, set false
                Key::Down => player_down_pressed = false, // if down arrow is released, set false
                _ => {} // ignore other keys
            }
        }

        // smooth movement when keys are held down
        if player_up_pressed {
            game.player_y -= PADDLE_SPEED; // move paddle up at constant rate
        }
        if player_down_pressed {
            game.player_y += PADDLE_SPEED; // move paddle down at constant rate
        }

        // prevent player paddle from going out of bounds
        // reminder that (0,0) is the very top left of the window
        if game.player_y < 0.0 {
            game.player_y = 0.0; // keep paddle at top edge of the screen
        }
        if game.player_y > HEIGHT - PADDLE_HEIGHT {
            game.player_y = HEIGHT - PADDLE_HEIGHT; // keep paddle at the bottom of the screen
        }

        // update game logic (ball movement, paddle movement, collisions, etc.)
        // reminder that the update() method contains the call to update_ai(),
        // so there is no need to call it here. Although you could do that if you wanted, it just
        // is easier to understand that this function will update everything at once
        game.update();

        // render the game elements in the window
        window.draw_2d(&event, |context, g, _| {
            game.render(context, g); // render the game based on new movements that are made
        });
    }
}