use crate::state::State;
use macroquad::prelude::*;

pub struct App {
    event_loop: EventLoop,
    state: State,
}

pub struct AppOptions {
    pub fullscreen: bool,
}

impl App {
    pub fn new(options: &AppOptions) -> Self {
        set_fullscreen(options.fullscreen);
        Self {
            state: State::new(),
            event_loop: EventLoop::new(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            if self.event_loop.should_exit {
                break;
            }
            self.state.update(&mut self.event_loop);
            self.state.render();
            self.state.ui();
            next_frame().await;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new(&AppOptions { fullscreen: true })
    }
}

pub struct EventLoop {
    should_exit: bool,
}
impl EventLoop {
    pub fn new() -> Self {
        Self { should_exit: false }
    }

    pub fn exit(&mut self) {
        self.should_exit = true
    }
}

pub trait StateHandler<T> {
    fn new() -> T;
    fn handle_key_press(&mut self, event_loop: &mut EventLoop);
    fn update(&mut self, event_loop: &mut EventLoop);
    fn render(&self);
    fn ui(&mut self);
}
