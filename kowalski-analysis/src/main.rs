use std::collections::{BTreeMap, BTreeSet};

use rand::seq::IteratorRandom;

use maximally_compressed_emoji_shortcodes::{
    lookup, EMOJI_MAPS, FIXUP_TABLE, RAW_EMOJI_BY_LEN, RAW_PAIRS, TOTAL_PADDING, TRIMMED_TABLES,
};

fn main() {
    eprintln!(
        "average shortcode length: {}",
        RAW_PAIRS.iter().map(|(s, _)| s.len() as f32).sum::<f32>() / RAW_PAIRS.len() as f32
    );
    eprintln!(
        "average emoji length: {}",
        RAW_PAIRS.iter().map(|(_, e)| e.len() as f32).sum::<f32>() / RAW_PAIRS.len() as f32
    );
    eprintln!();
    eprintln!("total (shortcode, emoji) pairs: {}", RAW_PAIRS.len());
    eprintln!(
        "raw emoji char data: {}",
        RAW_PAIRS.iter().map(|(_, e)| e.len()).sum::<usize>()
    );
    eprintln!();
    eprintln!(
        "raw shortcode char data: {}",
        RAW_PAIRS.iter().map(|(s, _)| s.len()).sum::<usize>()
    );
    eprintln!(
        "total checksums bytes: {}",
        EMOJI_MAPS.iter().map(|m| m.keys.len()).sum::<usize>()
    );
    eprintln!(
        "total displacement-table data (in bytes): {}",
        EMOJI_MAPS.iter().map(|m| m.disps.len() * 8).sum::<usize>()
    );
    eprintln!();
    let emoji_map_overhead = core::mem::size_of_val(&*EMOJI_MAPS[0]) + 4;
    let trimmed_tables_total = emoji_map_overhead * TRIMMED_TABLES;
    eprintln!(
        "trimmed tables: {} (roughly -{} * n = {})",
        TRIMMED_TABLES, emoji_map_overhead, trimmed_tables_total,
    );
    eprintln!("total padding: {}", TOTAL_PADDING);
    eprintln!(
        "   net: {}",
        trimmed_tables_total as isize - TOTAL_PADDING as isize
    );
    eprintln!();
    eprintln!(
        "fixup table overhead: {}",
        core::mem::size_of_val(&FIXUP_TABLE)
            + FIXUP_TABLE
                .iter()
                .map(|(s, e)| s.len() + e.len())
                .sum::<usize>()
    );
    for (s, v) in &FIXUP_TABLE {
        eprintln!("  {} | {}", s, v);
    }

    eprintln!("raw emoji-by-len groupings (pre-trim)");
    for (s, v) in RAW_EMOJI_BY_LEN {
        eprintln!("{} | {}", s, v);
    }

    eprintln!("\n\n-------------------------------\n\n");

    let raw_map = RAW_PAIRS.iter().copied().collect::<BTreeMap<_, _>>();

    // let mut c = 0;
    // for (code, _) in RAW_PAIRS {
    //     c += match lookup(code) {
    //         Some(_) => code.len() + 5,
    //         None => continue,
    //     }
    // }
    // eprintln!("{:?}", c);
    // return;

    // let test = RAW_PAIRS
    //     .iter()
    //     .filter(|(_, e)| e.len() == 12)
    //     .collect::<Vec<_>>();
    // for (k, v) in test {
    //     eprintln!("{}: {}", v, k);
    // }

    // let test = RAW_PAIRS.iter().fold(
    //     std::collections::BTreeMap::<usize, Vec<_>>::new(),
    //     |mut a, (code, e)| {
    //         assert!(e.as_bytes().iter().all(|b| *b != 0));
    //         a.entry(e.len()).or_default().push((code, e));
    //         a
    //     },
    // );
    // for (k, v) in &test {
    //     eprintln!("Length {} - {}", k, v.len());
    // }

    // for (k, v) in test {
    //     if k == 4 || k == 8 {
    //         continue;
    //     }
    //     eprintln!("Length {}", k);
    //     for (code, e) in v {
    //         eprintln!("  {}: {}", e, code);
    //     }
    // }

    // return;

    #[allow(unused_variables)]
    let kbd = keygraph_rs::generate_qwerty_us();

    let mut rng = rand::thread_rng();

    let shortcode_set = RAW_PAIRS
        .iter()
        .map(|(a, _)| a)
        .copied()
        .collect::<BTreeSet<_>>();

    let mut checks = 0;

    // ensure that the there are no false negatives
    for word in shortcode_set.iter() {
        let true_val = raw_map.get(word).unwrap();
        let val = lookup(word);
        if val != Some(true_val) {
            eprintln!(
                "invalid map result! {} returned {}|{} instead of {}|{}",
                word,
                val.unwrap().len(),
                val.unwrap(),
                true_val.len(),
                true_val
            );
        }
    }

    eprintln!("\n\n-------------------------------\n\n");

    // check all words

    let dict = std::fs::read_to_string("/usr/share/dict/words").unwrap();
    for word in dict.split('\n') {
        let word = word.to_owned().to_ascii_lowercase();
        if word.chars().any(|c| !c.is_ascii_alphabetic()) {
            continue;
        }

        if raw_map.contains_key(word.as_str()) {
            continue;
        }

        let val = lookup(&word);
        if let Some(val) = val {
            eprintln!("found surprising mapping! {} returned {}", word, val);
        }
    }

    eprintln!("\n\n-------------------------------\n\n");

    // check fat-fingers

    let mut correct_emoji = String::new();
    let mut correct_word = String::new();

    for word in shortcode_set.iter() {
        correct_word = word.to_string();
        correct_emoji = match lookup(word) {
            Some(v) => v.into(),
            None => continue,
        };

        let mut word = word.as_bytes().to_vec();
        let mut orig;
        for i in 0..word.len() {
            orig = word[i];

            // for key in kbd.neighbors(kbd.find_key(word[i] as char).unwrap()).map(|k|
            // k.value) {
            for key in 'a'..='z' {
                if key as u8 == orig {
                    continue;
                }

                word[i] = key as u8;
                let bad_word = String::from_utf8(word.clone()).unwrap();

                checks += 1;
                if lookup(&bad_word) == Some(&correct_emoji) {
                    eprintln!(
                        "collision! {} | {} vs. {}",
                        correct_emoji, correct_word, bad_word
                    );
                }
            }

            // check accidental insertions
            for key in 'a'..='z' {
                let mut bad_word = word.clone();
                bad_word.insert(i, key as u8);

                let bad_word = String::from_utf8(bad_word).unwrap();

                checks += 1;
                if lookup(&bad_word) == Some(&correct_emoji) {
                    eprintln!(
                        "collision! {} | {} vs. {}",
                        correct_emoji, correct_word, bad_word
                    );
                }
            }

            word[i] = orig;
        }
    }

    eprintln!("\n\n-------------------------------\n\n");

    // multiple random mispellings?
    for word in shortcode_set.iter() {
        correct_word = word.to_string();
        correct_emoji = match lookup(word) {
            Some(v) => v.into(),
            None => continue,
        };

        let mut word = word.as_bytes().to_vec();

        let pick_two = (0..word.len()).choose_multiple(&mut rng, 2);
        let mut pick_two = pick_two.iter();

        let first = pick_two.next().cloned().unwrap();

        let orig = word[first];
        for key in 'a'..='z' {
            if key as u8 == orig {
                continue;
            }
            word[first] = key as u8;

            let second = match pick_two.clone().next() {
                Some(v) => *v,
                None => continue,
            };

            let orig = word[second];
            for key in 'a'..='z' {
                if key as u8 == orig {
                    continue;
                }
                word[second] = key as u8;

                let bad_word = String::from_utf8(word.clone()).unwrap();

                checks += 1;
                if lookup(&bad_word) == Some(&correct_emoji) {
                    eprintln!(
                        "collision! {} | {} vs. {}",
                        correct_emoji, correct_word, bad_word
                    );
                }
            }
            word[second] = orig;
        }

        word[first] = orig;
    }

    eprintln!("checked {} misspellings", checks);
}
