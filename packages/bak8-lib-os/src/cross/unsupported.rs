use std::path::Path;
use crate::{CrossPlatform, CommandReturn};

pub(super) struct UnsupportedCrossPlatform;
impl CrossPlatform for UnsupportedCrossPlatform {
    fn in_terminal(&self) -> bool {
        unsupported!("Operating system is not supported")
    }

    fn run_best_editor(&self, file: &Path, child_process: bool) -> anyhow::Result<CommandReturn> {
        unsupported!("Operating system is not supported")
    }
}
