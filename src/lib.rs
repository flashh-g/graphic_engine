use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
pub struct Setup {}

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add<F>(&mut self, mut func: F) -> F
    where
        F: Any,
    {
        func
    }
}
pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
