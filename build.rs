use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process;

use serde::Deserialize;

const EMOJI_DB: &str = "https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json";

#[derive(Deserialize)]
struct Emoji {
    emoji: String,
    aliases: Vec<String>,
    // category: String,
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
    let mut emoji: Vec<Emoji> = serde_json::from_slice(&source).unwrap();

    // insert easter egg
    emoji.push(Emoji {
        emoji: "💯👌😩👌🔥".into(),
        aliases: vec!["prilik".into()],
    });
    let emoji = emoji;

    let codegen_path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let file = &mut BufWriter::new(File::create(&codegen_path).unwrap());

    let mut emoji_map = BTreeMap::new(); // used later to check for collisions

    writeln!(
        file,
        "pub static RAW_PAIRS: &[(&'static str, &'static str)] = &["
    )
    .unwrap();
    for e in &emoji {
        for name in &e.aliases {
            emoji_map.insert(name.clone(), e.emoji.clone());
            writeln!(file, "    (\"{}\", \"{}\"),", name, e.emoji).unwrap();
        }
    }
    writeln!(file, "];").unwrap();

    let emoji_map = emoji_map;

    let mut emoji_by_len: BTreeMap<usize, Vec<_>> = BTreeMap::new();
    for e in &emoji {
        for name in &e.aliases {
            emoji_by_len
                .entry(e.emoji.len())
                .or_default()
                .push((name.as_str(), e.emoji.clone()))
        }
    }

    writeln!(file, "pub static RAW_EMOJI_BY_LEN: &[(usize, usize)] = &[").unwrap();
    for (k, v) in emoji_by_len.iter() {
        writeln!(file, "    ({}, {}),", k, v.len()).unwrap();
    }
    writeln!(file, "];").unwrap();

    // consolidate small tables into larger tables

    let mut trimmed_tables = 0;
    let mut total_padding = 0;

    for _ in 0..emoji_by_len.len() {
        let min_table_len = *emoji_by_len.iter().min_by_key(|(_, v)| v.len()).unwrap().0;
        let entries1 = emoji_by_len.remove(&min_table_len).unwrap();

        let next_min_table_len = *emoji_by_len.iter().min_by_key(|(_, v)| v.len()).unwrap().0;
        let entries2 = emoji_by_len.remove(&next_min_table_len).unwrap();

        let (padding, insert, from, mut into) = if min_table_len < next_min_table_len {
            (
                next_min_table_len - min_table_len,
                next_min_table_len,
                entries1,
                entries2,
            )
        } else {
            (
                min_table_len - next_min_table_len,
                min_table_len,
                entries2,
                entries1,
            )
        };

        // stop consolidating if doing so would introduce more padding than the space
        // savings of trimming tables
        //
        // while this approach isn't guaranteed to find to maximum space savings (as it
        // is possible that the maximum `space_saved - padding` value occurs earlier on
        // through iteration), it does trim the maximum number of tables losslessly.
        //
        // less tables = less collisions = smaller fixup table, so I think this is fine.
        const HASHMAP_BINARY_OVERHEAD: usize = 64 + 4; // roughly size_of(phf::Map) + size_of(&phf::Map)
        if total_padding + (from.len() * padding) > HASHMAP_BINARY_OVERHEAD * (trimmed_tables + 1) {
            // bail, it's not worth it
            emoji_by_len.insert(into.iter().next().unwrap().1.len(), into);
            emoji_by_len.insert(from.iter().next().unwrap().1.len(), from);
            break;
        }

        trimmed_tables += 1;

        let padding_s = "\0".repeat(padding);

        into.extend({
            from.into_iter().map(|(k, mut v)| {
                (k, {
                    v.push_str(&padding_s);
                    total_padding += padding;
                    v
                })
            })
        });

        emoji_by_len.insert(insert, into);
    }

    writeln!(
        file,
        "pub static TRIMMED_TABLES: usize = {};",
        trimmed_tables
    )
    .unwrap();
    writeln!(file, "pub static TOTAL_PADDING: usize = {};", total_padding).unwrap();

    let mut maps = Vec::new();
    for (len, keys) in &emoji_by_len {
        // generate the PHFs
        let mut m = phf_codegen::Map::new(*len);
        for (name, emoji) in keys {
            m.entry(name, emoji);
        }
        maps.push(m);
    }

    // more iters = more random seeds = longer to compile = better fixup table
    // there are some considerable diminishing returns though, so don't go too crazy
    let mut iters = 40;
    let mut collisions = None;
    let mut built_maps = None;
    loop {
        let try_built_maps = maps.iter().map(|m| m.build(iters)).collect::<Vec<_>>();

        let mut try_collisions = Vec::new();
        for (word, true_emoji) in emoji_map.iter() {
            let mut emoji = None;
            for m in try_built_maps.iter() {
                if let Some(e) = m.get(word) {
                    emoji = Some(e.to_owned());
                    break;
                }
            }

            let emoji = emoji.unwrap();
            if &emoji != true_emoji {
                try_collisions.push((word, emoji, true_emoji.clone()))
            }
        }

        fn fixup_table_len(v: &Vec<(&String, String, String)>) -> usize {
            v.len() * core::mem::size_of::<*const u8>() * 2
                + v.iter().map(|(w, _, e)| w.len() + e.len()).sum::<usize>()
        }

        if fixup_table_len(&try_collisions)
            < collisions
                .as_ref()
                .map(|v| fixup_table_len(v))
                .unwrap_or(!0)
        {
            collisions = Some(try_collisions);
            built_maps = Some(try_built_maps);
        }

        iters -= 1;
        if iters == 0 {
            break;
        }
    }
    let (collisions, built_maps) = (collisions.unwrap(), built_maps.unwrap());

    writeln!(
        file,
        "pub static FIXUP_TABLE: [(&'static str, &'static str); {}] = [",
        collisions.len()
    )
    .unwrap();
    for (word, _emoji, true_emoji) in collisions.iter() {
        // eprintln!("{} | {} vs. {}", word, emoji, true_emoji);
        writeln!(file, r#"("{}", "{}"),"#, word, true_emoji).unwrap();
    }
    writeln!(file, "];").unwrap();

    // if !collisions.is_empty() {
    //     panic!("found collisions!");
    // }

    for m in built_maps {
        write!(file, "pub static EMOJI_{}: phf::Map = ", m.datum_len).unwrap();
        writeln!(file, "{};", m).unwrap();
    }

    writeln!(file, "pub static EMOJI_MAPS: &[&'static phf::Map] = &[").unwrap();
    for (len, _) in emoji_by_len {
        writeln!(file, "    &EMOJI_{},", len).unwrap();
    }
    writeln!(file, "];").unwrap();
}
