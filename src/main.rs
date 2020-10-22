use std::error::Error;

use colored::{Color, Colorize};

use crate::formatter::format_system_data;
use crate::ioreg::get_ioreg_info;
use crate::ipconfig::get_ipconfig;
use crate::systemctl::get_systemctl_info;
use crate::whoami::get_who_am_i;

mod error;
mod formatter;
mod ioreg;
mod ipconfig;
mod systemctl;
mod whoami;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let whoami = get_who_am_i()?;
    let ipconfig = get_ipconfig()?;
    let systemctl_info = get_systemctl_info()?;
    let ioreg_info = get_ioreg_info()?;

    let mut formatted_data = format_system_data(&whoami, &ipconfig, &systemctl_info, &ioreg_info);

    let ascii_art = format!(
        "{}{}{}{}{}",
        r#"                    'c.
                 ,xNMM.
               .OMMMMo
               OMMM0,
     .;loddo:' loolloddol;.
   cKMMMMMMMMMMNWMMMMMMMMMM0:  "#,
        r#"
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.
 XMMMMMMMMMMMMMMMMMMMMMMMX."#,
        r#"
;MMMMMMMMMMMMMMMMMMMMMMMM:
:MMMMMMMMMMMMMMMMMMMMMMMM:
.MMMMMMMMMMMMMMMMMMMMMMMMX.
 kMMMMMMMMMMMMMMMMMMMMMMMMWd."#,
        r#"
 .XMMMMMMMMMMMMMMMMMMMMMMMMMMk
  .XMMMMMMMMMMMMMMMMMMMMMMMMK."#,
        r#"
    kMMMMMMMMMMMMMMMMMMMMMMd
     ;KMMMMMMMWXXWMMMMMMMk.
       .cooc,.    .,coo:."#,
    );

    let ascii_art = ascii_art.lines().collect::<Vec<&str>>();

    (0..ascii_art.len() - formatted_data.len()).for_each(|_| {
        formatted_data.push("".to_string());
    });

    let rows = ascii_art.iter().zip(formatted_data.iter());

    println!();

    for (line_no, (ascii_line, data_row)) in rows.enumerate() {
        println!(
            "{:<033}{}",
            ascii_line.color(get_ascii_art_color(line_no)).bold(),
            data_row
        );
    }

    println!();
    println!();

    Ok(())
}

fn get_ascii_art_color(line_no: usize) -> Color {
    match line_no {
        0..=5 => Color::BrightGreen,
        6..=7 => Color::BrightYellow,
        8..=11 => Color::BrightRed,
        12..=13 => Color::BrightMagenta,
        14..=16 => Color::BrightBlue,
        _ => unreachable!(),
    }
}
