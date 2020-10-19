use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::SystemTime;

use chrono::NaiveTime;
use regex::Regex;

use crate::error::CustomError;
use crate::Result;

#[derive(Debug)]
pub struct SystemctlInfo {
    hostname: String,
    hardware_model: String,
    cpu_name: String,
    physical_cores: u8,
    logical_cores: u8,
    memory_amount: u8,
    arch_type: String,
    kernel_version: String,
    uptime: String,
    macos_version: String,
    macos_subversion: String,
}

impl SystemctlInfo {
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    pub fn hardware_model(&self) -> &str {
        &self.hardware_model
    }
    pub fn cpu_name(&self) -> &str {
        &self.cpu_name
    }
    pub fn physical_cores(&self) -> u8 {
        self.physical_cores
    }
    pub fn logical_cores(&self) -> u8 {
        self.logical_cores
    }
    pub fn memory_amount(&self) -> u8 {
        self.memory_amount
    }
    pub fn arch_type(&self) -> &str {
        &self.arch_type
    }
    pub fn kernel_version(&self) -> &str {
        &self.kernel_version
    }
    pub fn uptime(&self) -> &str {
        &self.uptime
    }
    pub fn macos_version(&self) -> &str {
        &self.macos_version
    }
    pub fn macos_subversion(&self) -> &str {
        &self.macos_subversion
    }
}

pub fn get_systemctl_info() -> Result<SystemctlInfo> {
    let mut sysctl_cmd = Command::new("sysctl")
        .arg("-n")
        .args(&[
            "kern.hostname",
            "hw.model",
            "machdep.cpu.brand_string",
            "hw.physicalcpu",
            "hw.logicalcpu",
            "hw.memsize",
            "kern.version",
            "kern.osrelease",
            "kern.boottime",
            "kern.osproductversion",
            "kern.osversion",
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let sysctl_cmd_stdout = sysctl_cmd.stdout.take().unwrap();

    let mut sysctl_cmd_stdout = BufReader::new(sysctl_cmd_stdout).lines();

    let hostname = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read hostname"))??;

    let hardware_model = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read hardware model"))??;

    let cpu_name = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read CPU name"))??
        .replace("(TM)", "")
        .replace("(R)", "")
        .replace("CPU ", "");

    let physical_cores = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read CPU physical core count"))??
        .parse()
        .map_err(|_| CustomError::new("Failed to convert CPU physical core count to u8"))?;

    let logical_cores = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read CPU logical core count"))??
        .parse()
        .map_err(|_| CustomError::new("Failed to convert CPU logical core count to u8"))?;

    let memory_amount = (sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read memory amount"))??
        .parse::<u64>()
        .map_err(|_| CustomError::new("Failed to convert memory amount to u64"))?
        / (1024 * 1024 * 1024)) as u8;

    let arch_type_re = Regex::new(r#"RELEASE_(\w+)"#)?;

    let arch_type_raw = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read raw arch type"))??;

    let arch_type = arch_type_re
        .captures(&arch_type_raw)
        .and_then(|c| c.get(1))
        .ok_or_else(|| CustomError::new("Failed to read arch type"))
        .map(|c| c.as_str().to_owned())?;

    let kernel_version = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read kernel version"))??;

    let uptime_re = Regex::new(r#"sec = (\d+),"#)?;

    let uptime_raw = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read raw uptime"))??;

    let uptime_secs: u64 = uptime_re
        .captures(&uptime_raw)
        .and_then(|c| c.get(1))
        .ok_or_else(|| CustomError::new("Failed to read uptime"))
        .map(|c| c.as_str().to_owned().parse())??;

    let curr_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let uptime = NaiveTime::from_num_seconds_from_midnight((curr_time - uptime_secs) as u32, 0)
        .format("%H:%M:%S")
        .to_string();

    let macos_version = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read macos version"))??;

    let macos_subversion = sysctl_cmd_stdout
        .next()
        .ok_or_else(|| CustomError::new("Failed to read macos version"))??;

    Ok(SystemctlInfo {
        hostname,
        hardware_model,
        cpu_name,
        physical_cores,
        logical_cores,
        memory_amount,
        arch_type,
        kernel_version,
        uptime,
        macos_version,
        macos_subversion,
    })
}
