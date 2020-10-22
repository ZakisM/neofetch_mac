use std::io::Read;
use std::process::{Command, Stdio};

use crate::Result;

#[derive(Debug)]
pub struct IpConfig(pub String);

pub fn get_ipconfig() -> Result<IpConfig> {
    let mut ipconfig_cmd = Command::new("ipconfig")
        .arg("getifaddr")
        .arg("en0")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut ipconfig_cmd_stdout = ipconfig_cmd.stdout.take().unwrap();

    let mut ipconfig_output = String::new();

    ipconfig_cmd_stdout.read_to_string(&mut ipconfig_output)?;

    let ipconfig_output = ipconfig_output.trim_end_matches('\n').to_owned();

    Ok(IpConfig(ipconfig_output))
}
