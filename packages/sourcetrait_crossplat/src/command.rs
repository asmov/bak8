use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdKind {
    Copy,
    Open,
    Editor,
    Which,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdCheck {
    Utf8,
}

pub enum CommandReturn {
    Output(process::Output),
    Child(process::Child),
}

impl CommandReturn {
    pub fn from(kind: CmdKind, mut command: Command) -> CrossResult<Self> {
        let output = command.output()
            .map_err(|source| CrossError::cmd_call(kind, source))?;
        
        match output.status.success() {
            true => Ok(Self::Output(output)),
            false => CrossError::err_cmd(kind, output),
        }
    }
    
    pub fn unchecked_from(kind: CmdKind, mut command: Command) -> CrossResult<Self> {
        command.output()
            .map(|output| Self::Output(output))
            .map_err(|source| CrossError::cmd_call(kind, source))
    }
    
    pub fn optional_utf8_from(kind: CmdKind, mut command: Command) -> CrossResult<Option<String>> {
        let mut output = command.output()
            .map_err(|source| CrossError::cmd_call(kind, source))?;
        
        if !output.status.success() {
            return CrossError::err_cmd::<Option<String>>(kind, output);
        }
        
        let stdout: Vec<_> = output.stdout.drain(..).collect();
        let s = String::from_utf8(stdout)
            .map(|s| s.trim().to_string())
            .map_err(|_| CrossError::cmd_checked(kind, CmdCheck::Utf8, output))?;
        
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
    
    pub fn spawn(kind: CmdKind, mut command: Command) -> CrossResult<Self> {
        command.spawn()
            .map(|child| Self::Child(child))
            .map_err(|source| CrossError::cmd_call(kind, source))
    }
    
    pub fn run(kind: CmdKind, command: Command, child_process: bool) -> CrossResult<Self> {
        if child_process {
            Self::spawn(kind, command)
        } else {
            Self::from(kind, command)
        }
    }
    
    pub fn run_unchecked(kind: CmdKind, command: Command, child_process: bool) -> CrossResult<Self> {
        if child_process {
            Self::spawn(kind, command)
        } else {
            Self::unchecked_from(kind, command)
        }
    }
    
    pub fn is_child_process(self) -> bool {
        match self {
            Self::Child(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_child(self) -> process::Child {
        match self {
            Self::Child(child) => child,
            _ => panic!("Not a child process"),
        }
    }

    pub fn unwrap_output(self) -> process::Output {
        match self {
            Self::Output(output) => output,
            _ => panic!("Not an output"),
        }
    }
}
