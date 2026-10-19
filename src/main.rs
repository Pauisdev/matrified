use matrified::app::App;

#[macroquad::main("Matrified")]
async fn main() {
    let mut app = App::default();
    app.run().await;
}
