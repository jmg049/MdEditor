use std::path::Path;

use crate::{
    error::{MdError, MdResult},
    MdEdit,
};

pub enum Action<'a> {
    Save(fn(Box<dyn AsRef<Path>>, &MdEdit) -> MdResult<()>),
    New(fn(Option<Box<dyn AsRef<Path>>>, &'a mut MdEdit) -> MdResult<()>),
    Load(fn(Box<dyn AsRef<Path>>, &'a mut MdEdit) -> MdResult<()>),
    Export(fn(Option<Box<dyn AsRef<Path>>>, &'a mut MdEdit) -> MdResult<()>),
}

impl<'a> Action<'a> {
    pub(crate) fn from_name<S: AsRef<str>>(name: S) -> MdResult<Self> {
        let name = name.as_ref();
        match name {
            "save" => Ok(Action::Save(save)),
            "New" => Ok(Action::New(new)),
            "Load" => Ok(Action::Load(load)),
            "Export" => Ok(Action::Export(export)),
            _ => Err(MdError::InvalidAction("Invalid action name".into())),
        }
    }
}

pub(crate) fn save(path: Box<dyn AsRef<Path>>, md_edit: &MdEdit) -> MdResult<()> {
    let current_content = &md_edit.current_content;
    std::fs::write(path.as_ref(), current_content)?;
    Ok(())
}

pub(crate) fn new(path: Option<Box<dyn AsRef<Path>>>, md_edit: &mut MdEdit) -> MdResult<()> {
    todo!("New action");
}

pub(crate) fn load(path: Box<dyn AsRef<Path>>, md_edit: &mut MdEdit) -> MdResult<()> {
    todo!("Load action");
}

pub(crate) fn export(path: Option<Box<dyn AsRef<Path>>>, md_edit: &mut MdEdit) -> MdResult<()> {
    todo!("Export action");
}
