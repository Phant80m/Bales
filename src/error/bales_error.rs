use super::BalesError;
use crate::archive::Archive;
use std::fmt;
use strum::VariantNames;
impl fmt::Display for BalesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use owo_colors::OwoColorize;
        match self {
            Self::NoFileExtension((input, output, command)) => {
                let cmd = if command == "pkg" {
                    format!(
                        "{} bales {} {} -o {}",
                        "x".red().bold(),
                        command,
                        input,
                        output.underline().red()
                    )
                } else {
                    format!(
                        "{} bales {} {} -o {}",
                        "x".red().bold(),
                        command,
                        input.underline().red(),
                        output
                    )
                };
                let fixed_cmd = if command == "pkg" {
                    format!(
                        "{} bales {} {} -o {}",
                        "".green().bold(),
                        command,
                        input,
                        (output.to_owned() + ".tar.gz").underline().green()
                    )
                } else {
                    format!(
                        "{} bales {} {} -o {}",
                        "".green().bold(),
                        command,
                        (input.to_owned() + ".tar.gz").underline().green(),
                        output,
                    )
                };
                write!(
                    f,
                    "\n{}  {}\n   {}\n    {}\n   {}\n    {}",
                    "󱞪".bold(),
                    "No file extension specified!".red().bold(),
                    "Your command: ".bold(),
                    cmd,
                    "correct usage (example): ".bold(),
                    fixed_cmd,
                )
            }
            Self::FileExists(path, input) => {
                let format = format!(
                    "{} {} {}\n   {} {}",
                    format!("Path specified as {}:", input).bold().red(),
                    path.display().underline(),
                    "exists!".red().bold(),
                    "".green().bold(),
                    format!("Specify a {} path that does not exist!", input).bold(),
                );
                write!(f, "\n{}  {}", "󱞪".bold(), format)
            }
            Self::NoFileExists(path, input) => {
                let format = format!(
                    "{} {} {}\n   {} {}",
                    format!("Path specified as {}:", input).bold().red(),
                    path.display().underline(),
                    "does not exists!".red().bold(),
                    "".green().bold(),
                    format!("Specify a {} path that exists!", input).bold(),
                );
                write!(f, "\n{}  {}", "󱞪".bold(), format)
            }
            Self::UnknownArchiveType(extension) => {
                write!(
                    f,
                    "\n{}  {} .{}\n   {}\n{}",
                    "󱞪".bold(),
                    "Unknown compression format:".red().bold(),
                    extension.underline(),
                    "Available Formats".bold(),
                    Archive::VARIANTS
                        .to_vec()
                        .iter()
                        .map(|item| format!("  - {}", item))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            Self::RelativeUrlWithoutBase(url) => {
                write!(
                    f,
                    "\n{}  {}\n   {}\n    {}\n   {}\n    {}",
                    "󱞪".bold(),
                    "No url base specified!".red().bold(),
                    "url entered: ".bold(),
                    url.red().underline(),
                    "correct usage (example): ".bold(),
                    format!("{}{}", "https://".green().underline(), url),
                )
            }
            Self::InvalidChar(char) => {
                let format = format!(
                    "{} \n   {}",
                    "One or more invalid characters were present in: "
                        .bold()
                        .red(),
                    char.underline()
                );
                write!(f, "\n{}  {}", "󱞪".bold(), format)
            }
            Self::NoOutputSpecified(input) => {
                let command = format!("{} bales extract --url {}", "x".red().bold(), input,);
                let fixed = format!(
                    "{} bales extract --url {} -o {}",
                    "".green().bold(),
                    input,
                    "example.tar.gz".green().underline(),
                );
                write!(
                    f,
                    "\n{}  {}\n   {}\n    {}\n   {}\n    {}",
                    "󱞪".bold(),
                    "No output specified!".red().bold(),
                    "Your command: ".bold(),
                    command,
                    "correct usage (example): ".bold(),
                    fixed,
                )
            }
        }
    }
}
