use std::collections::HashMap;

use crate::prompt_modules::Directory;

pub trait PromptModule { // todo add touched_env so render() is only called when the output will be changed, however this will require persistant data
    fn render(&self) -> Option<(String, u8, u8)>;
    fn box_clone(&self) -> Box<dyn PromptModule>;
}

impl Clone for Box<dyn PromptModule> {
    fn clone(&self) -> Box<dyn PromptModule> {
        self.box_clone()
    }
}

pub fn modules_from_strings(strs: Vec<&str>) -> Vec<Box<dyn PromptModule>> { // this is kinda a terrible way to program this but I can't use hashmap literals without adding a dep so... REFACTOR
    let mut modules_lookup: HashMap<&str, Box<dyn PromptModule>> = HashMap::new(); // This should become its own struct eventually.. REFACTOR
    modules_lookup.insert("directory", Box::new(Directory));

    let mut modules: Vec<Box<dyn PromptModule>> = vec![];
    
    for str in strs {
        if modules_lookup.contains_key(str) {
            modules.push(modules_lookup.get_key_value(str).unwrap().1.clone());
        }
    }

    modules
}