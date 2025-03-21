mod app;
mod utils;
mod glfw;
mod vulkan;

use app::App;

fn main() {
    let app: App = crate::App::new();
    app.execute();
}
