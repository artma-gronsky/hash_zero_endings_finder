use std::io::stdout;

use crossterm::{
    execute,
    style::{self, Color, Print, PrintStyledContent, Stylize},
};

pub fn output_number_with_hash(number: &usize, digest: &str) {
    let number = style::style(format!("{}", number)).with(Color::DarkYellow);
    let digest = style::style(digest).with(Color::Green);

    let _ = execute!(
        stdout(),
        PrintStyledContent(number),
        Print(", ".to_string()),
        PrintStyledContent(digest),
        Print("\n".to_string())
    );
}
