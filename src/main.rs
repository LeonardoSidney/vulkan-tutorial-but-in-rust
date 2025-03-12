mod app;
mod utils;
mod glfw;

use app::App;

fn main() {
    let app: App = crate::App::new();
    app.execute();
}
