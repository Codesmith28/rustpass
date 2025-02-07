pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
