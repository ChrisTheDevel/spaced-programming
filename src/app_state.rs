use crate::screens::Screen;

#[derive(Clone)]
pub struct AppState {
    pub screen_state: Screen,
    pub should_render: bool,
}

impl AppState {
    pub fn set_should_render(mut self, should_render: bool) -> Self {
        self.should_render = should_render;
        self
    }

    pub fn goto_screen(mut self, screen: Screen) -> Self {
        self.screen_state = screen;
        self.set_should_render(true)
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            screen_state: Screen::WelcomeScreen,
            should_render: false,
        }
    }
}
