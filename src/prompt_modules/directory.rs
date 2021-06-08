use std::env::{current_dir, var};

use crate::{PromptModule, StyleConfig};

#[derive(Clone)]
pub struct Directory;

pub trait PrettyPathDisplay {
    fn pretty_display(&self) -> String;
}

impl PrettyPathDisplay for std::path::PathBuf {
    fn pretty_display(&self) -> String {
        let mut string = self.display().to_string();
        let home_dir_string = var("HOME").unwrap_or("".to_string());

        if string.starts_with(&home_dir_string) {
            string = "~".to_string() + &string.strip_prefix(&home_dir_string).unwrap().to_string();
        }

        let split = string.split("/").collect::<Vec<_>>();

        let seperator = var("PPPP_DIRECTORY_SEPERATOR").unwrap_or("  ".to_string());
        let seperator = seperator.as_str();

        let mut rendered_string = "".to_string();

        for part in split {
            if part != ""{
                rendered_string = rendered_string + part + seperator;
            }
        }

        let rendered_string = rendered_string.strip_prefix(" ").unwrap_or(&rendered_string).to_string();
        let rendered_string = rendered_string.strip_suffix(&seperator).unwrap_or(&rendered_string).to_string();

        rendered_string
    }
}

impl PromptModule for Directory {
    fn render(&self) -> Option<(String, u8, u8)> {
        let mut default_style = StyleConfig::default(); // TODO builder pattern?????? mayhaps??
        default_style.background_color = 8;
        default_style.light = true;
        let style = StyleConfig::from_env("PPPP_DIRECTORY", default_style);

        current_dir().ok().map(|x| {
            (
                style.to_string() + &x.pretty_display(),
                style.background_color,
                style.background_color,
            )
        })
    }
    fn box_clone(&self) -> Box<dyn PromptModule> {
        Box::new(self.clone()) // Turn this into a derive.. REFACTOR
    }
}
