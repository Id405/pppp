use std::env::var;

use crate::{PromptModule, StyleConfig, prompt_module::modules_from_strings};

struct PromptConfig {
    seperator: String
}

impl PromptConfig {
    fn from_env() -> Self {
        PromptConfig {
            seperator: var("PPPP_PROMPT_SEPERATOR").unwrap_or("".to_string())
        }
    }
}

pub struct Prompt {
    modules: Vec<Box<dyn PromptModule>>
}

impl Prompt {
    pub fn new() -> Self {
        let modules_keys = var("PPPP_MODULES").ok().unwrap_or("directory".to_string()); // TODO add string literal support to PPP_MODULES
        let modules_keys = modules_keys.split(",").collect::<Vec<_>>(); //TODO smarter parsing here, respecting whitespace

        let modules = modules_from_strings(modules_keys);

        Self {
            modules
        }
    }

    pub fn render(&self) -> String {
        let prompt_config = PromptConfig::from_env();

        let mut output = "".to_string();

        for module in &self.modules {
            let (string, background_start, background_end) = module.render().unwrap_or(("".to_string(), 0, 0));

            let mut style_start = StyleConfig::default();

            output = output + " " + &string + " " + &prompt_config.seperator;
            output += &StyleConfig::default().to_string();
        }
        output.strip_prefix(" ").unwrap_or(&output).to_string() + " "
    }
}