use std::path::Path;
use crate::CommandReturn;

pub(crate) fn is_gui() -> bool {
    todo!("implement macos")
}

pub(crate) fn run_best_editor(_file: &Path, _child_process: bool) -> anyhow::Result<CommandReturn> {
    todo!("implement macos")
}
