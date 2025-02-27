use game_engine::{game_loop, input::Input, Game};
use winit::keyboard::KeyCode;

struct Test { id: i32}

impl Test{
    fn new() -> Self {
        Test { id: 0 }
    }
}

impl Game for Test
{
    fn update(&mut self, input: &Input) 
    { 
        if input.is_key_pressed(KeyCode::Space)
        {
            self.id += 1;
            println!("Id: {}", self.id);
        }
    }

    fn render(&self) {}
}

fn main() 
{
    // pollster::block_on(run());
    pollster::block_on(game_loop(Box::new(Test::new())));
}