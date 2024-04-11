use bracket_lib::{prelude::*, random};

enum GameMode {
    Menu,
    Playing,
    End
}

// Constants
// const's type must be decleared
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct State {
    player: Player, // adding an instance of the player to the game state
    frame_time: f32, // tracks the time accumulated between frames to control the game’s speed
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

// custom data type I have created
struct Player {
    x: i32, // 32 bit signed integer of the position of the character on the x axis
    y: i32, // 32 bit signed integer of the position of the character on the y axis
    velocity: f32 //  floating point integer
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

// fuctionality for the created custom data type of Obstacle
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rand = RandomNumberGenerator::new();
        Obstacle { 
            x, 
            gap_y: rand.range(10, 40), 
            size: i32::max(2, 20 - score) 
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32){
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // For loop for the drawing the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|'),
            );
        
        // Bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|')
                );
            }
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

// functionality for the created custom data type
impl Player {
    // constructor initializing a new Player instance to a specified started position
    fn new(x: i32, y:i32) -> Self {
        Player { 
            x: x, 
            y: y, 
            velocity: 0.0 
        }
    }

    // rendering the player character on screen 
    fn render(&mut self, ctx: &mut BTerm){
        // places a single character on screen
        ctx.set(
            0, // what position on the x axis to render your character
            self.y, // what position on the y axis to render your character
            YELLOW, 
            BLACK, 
            to_cp437('@') // character to be rendered
        );
    }

    fn gravity(&mut self){
        if self.velocity < 2.0{
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0
        }
    }

    fn flap(&mut self){
        self.velocity = -2.0;
    }

}

impl State {
    fn new() -> Self {
        State { 
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn play (&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "SPACE To Flap");
        // printing the score on screen using format macro so that it prints as a string
        ctx.print(0, 1, &format!("Score: {}", self.score));
        // rendering the obstacles on screen
        self.obstacle.render(ctx, self.player.x);
        // passing the obstacle generates a score
        if self.player.x > self.obstacle.x {
            self.score +=1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    // runs whenever the new game restarts - resetting the game state and indicating that the game is in progress
    fn restart(&mut self) {
        // brings the player back to the edge of the screen
        self.player = Player::new(5, 25);
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
        // the ticks
        self.frame_time = 0.0;
        // the game enters into the playing mode
        self.mode = GameMode::Playing;
    }


    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome To Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
    
        // reads the input from the user
        if let Some(key) = ctx.key {
            // inside of a match loop you use commas 
            match key {
                // reads the key input of P from the keyboard
                // if it is P then the game restarts
                VirtualKeyCode::P => self.restart(),
                // if Q the game quits
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm){
        // functions use semicolons
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points!", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        // registering keyboard input from the user
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

}

impl GameState for State{
    // ctx provides functions for interacting with the game display
    fn tick(&mut self, ctx: &mut BTerm){
        //ctx.cls(); // clears the window on screen, ensuring no residula data from the previous frames carries over
        //ctx.print(1, 1, "Hello Terminal"); // the 1,1 are screen space coordinates
        
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
            
        }
    
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()// here we request a screen that is 80x50
        .with_title("Flappy Dragon") // the screen which will be displayed will be titled Flappy Dragon
        .build()?; // 

    main_loop(context, State::new() )
}
