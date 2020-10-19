use std::io::Read;
use std::process::{Command, Stdio};

use regex::Regex;

use crate::Result;

#[derive(Debug)]
pub struct IoregInfo {
    gpu_names: Vec<String>,
}

impl IoregInfo {
    pub fn gpu_names(&self) -> &Vec<String> {
        &self.gpu_names
    }
}

pub fn get_ioreg_info() -> Result<IoregInfo> {
    let mut sysctl_cmd = Command::new("ioreg")
        .arg("-d")
        .arg("12")
        .arg("-c")
        .arg("IOPCIDevice")
        .arg("-k")
        .arg("model")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut sysctl_cmd_stdout = sysctl_cmd.stdout.take().unwrap();

    let mut sysctl_output = String::new();

    sysctl_cmd_stdout.read_to_string(&mut sysctl_output)?;

    let gpu_name_re = Regex::new(r#"model" = <"([^"]+)"#)?;

    let gpu_names = gpu_name_re
        .captures_iter(&sysctl_output)
        .filter(|m| m.get(1).is_some())
        .map(|m| m.get(1).unwrap().as_str().to_owned())
        .collect::<Vec<String>>();

    Ok(IoregInfo { gpu_names })
}
