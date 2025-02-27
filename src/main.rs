use game_engine::{game_loop, input::Input, EngineEvent};
use winit::{event::MouseButton, keyboard::KeyCode};

struct Test { id: i32}

impl Test{
    fn new() -> Self {
        Test { id: 0 }
    }
}

impl EngineEvent for Test
{
    fn update(&mut self, input: &Input) 
    { 
        if input.is_key_hold(KeyCode::Space)
        {
            self.id += 1;
            println!("Id: {}", self.id);
        }
        if input.is_mouse_pressed(MouseButton::Left)
        {
            println!("Position: {:?}", input.mouse_position());
        }

        if input.is_key_pressed(KeyCode::Tab)
        {
            println!("Tab pressed");
        }
    }

    fn render(&self) {}
}

fn main()   
{
    // pollster::block_on(run());
    pollster::block_on(game_loop(Box::new(Test::new())));
}