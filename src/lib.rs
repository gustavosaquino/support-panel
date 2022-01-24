extern crate cosmic_component_system as ccs;
extern crate gtk4 as gtk;

pub mod components;
pub mod gresource;
pub mod localize;
pub mod support_info;
pub mod system76;
pub mod vendor;
pub mod widgets;

use anyhow::Context;

pub fn generate_logs_subprocess() -> anyhow::Result<String> {
    let home_dir = dirs::home_dir().context("no home directory")?;

    std::process::Command::new("pkexec")
        .arg("pop-support")
        .arg("generate-logs")
        .arg(home_dir)
        .output()
        .context("failed to start command to generate logs")
        .and_then(|output| {
            let output = String::from_utf8(output.stdout)
                .context("output of command to generate logs is corrupted")?;

            let path = dbg!(&output)
                .strip_prefix("PATH ")
                .context("command that generated logs did not provide path to logs")?;

            Ok(path.trim().to_owned())
        })
}
