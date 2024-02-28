use std::collections::HashMap;

use egui::{Key, Modifiers};

use crate::actions::Action;
use crate::error::MdResult;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct HotKey {
    key: Key,
    modifiers: Modifiers,
}

pub(crate) struct KeyManager<'a> {
    hotkeys: HashMap<HotKey, Action<'a>>,
}

impl<'a> Default for KeyManager<'a> {
    fn default() -> Self {
        Self {
            hotkeys: HashMap::new(),
        }
    }
}

impl<'a> KeyManager<'a> {
    pub fn handle(&self, ctx: &egui::Context) {
        todo!("Handle hotkeys")
    }

    pub fn register_hotkey(&mut self, key: Key, modifiers: Modifiers, action: Action<'a>) {
        let hotkey = HotKey { key, modifiers };
        self.hotkeys.insert(hotkey, action);
    }

    pub fn handle_hotkey(&self, hotkey: HotKey) -> MdResult<()> {
        if let Some(action) = self.hotkeys.get(&hotkey) {
            todo!()
        } else {
            Ok(())
        }
    }

    pub fn from_config(config: HashMap<String, String>) -> MdResult<Self> {
        // config map will contain (key combination -> action name), the keys will be separated by '+'
        let mut hotkey_manager = KeyManager::default();

        // for (key_combination, action_name) in config {
        //     let key_combination: Vec<&str> = key_combination.split('+').collect();
        //     let key = key_combination.last().unwrap().parse::<Key>()?;
        //     let modifiers = key_combination[..key_combination.len()-1].iter().map(|&modifier| modifier.parse::<Modifiers>()).collect::<MdResult<Vec<Modifiers>>>()?;

        //     let action = Action::from_name(action_name.as_str())?;

        //     hotkey_manager.register_hotkey(key, Modifiers::default(), action);
        // }

        Ok(hotkey_manager)
    }
}
