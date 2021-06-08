mod prompt_module;
mod prompt;
mod common_config;
mod prompt_modules;

pub use prompt_module::PromptModule;
pub use prompt::Prompt;
pub use common_config::StyleConfig;

fn main() {
    let prompt = Prompt::new();
    println!("{}", prompt.render());
}
