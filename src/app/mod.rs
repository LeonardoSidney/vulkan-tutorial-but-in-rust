mod graphics;
use graphics::{Graphics, GraphicsType};

pub struct App {
    graphic_api: Graphics,
}

impl App {
    pub fn new() -> Self {
        let width: usize = 800;
        let height: usize = 600;
        let api_type: GraphicsType = App::select_graphics_api(GraphicsType::Vulkan);
        let graphic_api: Graphics = Graphics::new(width, height, api_type);

        Self { graphic_api }
    }

    pub fn execute(&self) {
        self.graphic_api.init_window();
        self.graphic_api.init_api();
        self._main_loop();
        self.graphic_api.cleanup();
    }

    fn select_graphics_api(selected_api: GraphicsType) -> GraphicsType {
        let mut api: GraphicsType = GraphicsType::Vulkan;
        if selected_api == GraphicsType::OpenGL {
            api = GraphicsType::OpenGL;
        }

        return api;
    }

    fn _main_loop(&self) {
        
        loop {
            if self.graphic_api.should_close() {
                break;
            }
            self.graphic_api.pool_events();
        }
    }
}
