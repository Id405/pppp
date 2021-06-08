pub struct LiteralModule {
    string: String,
}

impl PromptModule for LiteralModule {
    fn render(&self) -> Option<(String, u8, u8)> {
        let style = StyleConfig::default();
        Some((string, style.background_color, style.background_color))
    }

    fn box_clone(&self) -> Box<dyn PromptModule> {
        Box::new(self.clone()) // Turn this into a derive.. REFACTOR
    }
}