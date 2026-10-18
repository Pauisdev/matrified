mod app;
mod function;
mod math;
mod state;
mod ui;
use crate::app::{App, AppOptions};
mod scene;

#[macroquad::main("Matrified")]
async fn main() {
    let mut app = App::new(&AppOptions { fullscreen: true });
    app.run().await;
}
