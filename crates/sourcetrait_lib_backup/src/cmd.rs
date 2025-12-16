pub mod rsync;
pub mod xz;
pub mod ssh_run;

#[derive(Debug, Copy, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum Cmd {
    rsync,
    xz,
    ssh,
    scp,
}