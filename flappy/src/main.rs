use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State { mode: GameMode::Menu }
    }

    fn play (&mut self, _ctx: &mut BTerm){
        // TODO Fill in this stuf later
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }


    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome To Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
    
        
        if let Some(key) = ctx.key {
            // inside of a match loop you use commas 
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm){
        // functions use semicolons
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
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
