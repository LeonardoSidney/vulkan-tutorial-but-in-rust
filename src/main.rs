mod app;
mod gateways;
mod controllers;
mod infra;

use app::App;

fn main() {
    let app: App = crate::App::new();
    app.execute();
}
