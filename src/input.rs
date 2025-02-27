use std::collections::HashSet;

use winit::{event::{ElementState, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

pub struct Input
{
    keys_pressed: HashSet<KeyCode>,
}

impl Input
{
    pub fn new() -> Self
    {
        Self
        {
            keys_pressed: HashSet::new(),
        }
    }

    pub fn update_keys(&mut self, event: &WindowEvent)
    {
        if let WindowEvent::KeyboardInput 
            { 
                event: KeyEvent
                {
                    state,
                    physical_key: PhysicalKey::Code(key),
                    ..
                },
                ..
            } = event
        {
            match state
            {
                ElementState::Pressed => { self.keys_pressed.insert(*key); }
                ElementState::Released => { self.keys_pressed.remove(key); }
            }
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool
    {
        self.keys_pressed.contains(&key)
    }
}