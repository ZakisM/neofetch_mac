use colored::Colorize;

use crate::ioreg::IoregInfo;
use crate::systemctl::SystemctlInfo;
use crate::whoami::WhoAmI;

pub fn format_system_data(
    whoami: &WhoAmI,
    systemctl_info: &SystemctlInfo,
    ioreg_info: &IoregInfo,
) -> Vec<String> {
    let mut result = String::new();

    result.push_str(&format!(
        "{}@{}\n",
        whoami.0.bright_green().bold(),
        systemctl_info.hostname().bright_green().bold()
    ));

    result.push_str(&"-".repeat(
        whoami.0.chars().count() + systemctl_info.hostname().chars().count() + "@".chars().count(),
    ));

    result.push_str(&format!(
        "\n{}: macOS {} {} {}",
        "OS".bright_yellow().bold(),
        systemctl_info.macos_version(),
        systemctl_info.macos_subversion(),
        systemctl_info.arch_type()
    ));

    result.push_str(&format!(
        "\n{}: {}",
        "Host".bright_yellow().bold(),
        systemctl_info.hardware_model()
    ));

    result.push_str(&format!(
        "\n{}: {}",
        "Kernel".bright_yellow().bold(),
        systemctl_info.kernel_version()
    ));

    result.push_str(&format!(
        "\n{}: {}",
        "Uptime".bright_yellow().bold(),
        systemctl_info.uptime()
    ));

    result.push_str(&format!(
        "\n{}: {}",
        "CPU".bright_yellow().bold(),
        systemctl_info.cpu_name()
    ));

    result.push_str(&format!(
        "\n{:>6} physical, {} logical cores",
        systemctl_info.physical_cores(),
        systemctl_info.logical_cores()
    ));

    result.push_str(&format!(
        "\n{}: {}",
        "GPU".bright_yellow().bold(),
        ioreg_info.gpu_names().join(", ")
    ));

    result.push_str(&format!(
        "\n{}: {}GB",
        "Memory".bright_yellow().bold(),
        systemctl_info.memory_amount()
    ));

    result.lines().map(|l| l.to_owned()).collect()
}
