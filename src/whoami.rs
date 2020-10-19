use std::io::Read;
use std::process::{Command, Stdio};

use crate::Result;

#[derive(Debug)]
pub struct WhoAmI(pub String);

pub fn get_who_am_i() -> Result<WhoAmI> {
    let mut whoami_cmd = Command::new("whoami").stdout(Stdio::piped()).spawn()?;

    let mut whoami_cmd_stdout = whoami_cmd.stdout.take().unwrap();

    let mut whoami_output = String::new();

    whoami_cmd_stdout.read_to_string(&mut whoami_output)?;

    let whoami_output = whoami_output.trim_end_matches('\n').to_owned();

    Ok(WhoAmI(whoami_output))
}
