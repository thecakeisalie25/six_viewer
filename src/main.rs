//! A very small app to help me grade a certain homework.

use nu_ansi_term;
use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> anyhow::Result<()> {
    for line in BufReader::new(File::open("./output.txt")?).lines() {
        for c in line?.chars() {
            match c.to_ascii_uppercase() {
                'B' => print!("{}", nu_ansi_term::Color::Yellow.paint("B")),
                'W' => print!("{}", nu_ansi_term::Color::LightYellow.paint("W")),
                'T' => print!("{}", nu_ansi_term::Color::Red.paint("T")),
                '_' => print!("{}", nu_ansi_term::Color::White.paint("_")),
                other => print!("{}", nu_ansi_term::Color::default().paint::<String, _>(other.into())),
            }
            print!(" ")
        }
        println!()
    }
    return anyhow::Ok(());
}
