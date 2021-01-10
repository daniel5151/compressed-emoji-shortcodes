use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process;

use serde::Deserialize;

const EMOJI_DB: &str = "https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json";

#[derive(Deserialize)]
struct Emoji {
    emoji: Option<String>,
    aliases: Vec<String>,
    category: String,
}

fn main() {
    let emoji_db_path = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("emoji.json");
    if !emoji_db_path.exists() {
        if !process::Command::new("curl")
            .arg("-O")
            .arg(EMOJI_DB)
            .spawn()
            .expect("could not spawn `curl`. Is it installed?")
            .wait()
            .unwrap()
            .success()
        {
            panic!("failed to `curl` emoji JSON db")
        }
    }

    let source = fs::read(emoji_db_path)
        .expect("Can't load emoji.json. Did you run `download_emoji_db.sh?`");
    let emoji: Vec<Emoji> = serde_json::from_slice(&source).unwrap();

    let codegen_path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let file = &mut BufWriter::new(File::create(&codegen_path).unwrap());

    writeln!(file, "pub static SHORTCODES: &[&'static str] = &[").unwrap();
    for e in &emoji {
        // if e.category == "Flags" {
        //     continue;
        // }

        if let Some(_unicode_emoji) = &e.emoji {
            for name in &e.aliases {
                writeln!(file, "    \"{}\",", name).unwrap();
            }
        }
    }
    writeln!(file, "];").unwrap();

    // generate the PHF
    let mut m = phf_codegen::Map::new();
    for e in &emoji {
        // if e.category == "Flags" {
        //     continue;
        // }

        if let Some(unicode_emoji) = &e.emoji {
            let code = format!("\"{}\"", unicode_emoji);
            for name in &e.aliases {
                m.entry(name.as_str(), &code);
            }
        }
    }
    let m = m.build();

    writeln!(file, "/// Compile time generated lookup table for emoji.").unwrap();
    writeln!(file, "/// ").unwrap();
    writeln!(file, "/// Taken from https://github.com/github/gemoji").unwrap();
    write!(file, "pub static EMOJI: phf::Map<&'static str> = ").unwrap();
    writeln!(file, "{};", m).unwrap();
}
