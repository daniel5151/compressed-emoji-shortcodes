use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use keygraph_rs::KeySearch;
use rand::seq::IteratorRandom;

use maximally_compressed_gh_shortcodes::{lookup, SHORTCODES};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(unused_variables)]
    let kbd = keygraph_rs::generate_qwerty_us();

    let mut rng = rand::thread_rng();

    let shortcode_set = SHORTCODES.iter().copied().collect::<HashSet<_>>();

    let mut checks = 0;

    let words = BufReader::new(std::fs::File::open(
        "/home/danielprilik/Downloads/missp.dat.txt",
    )?);

    let mut correct_emoji = "";
    let mut correct_word = String::new();

    let mut skip = false;
    for ln in words.lines() {
        let ln = ln?;
        let word = if let Some(word) = ln.strip_prefix("$") {
            skip = false;
            if !shortcode_set.contains(word) {
                skip = true;
                continue;
            }
            correct_word = word.to_string();
            correct_emoji = lookup(word).unwrap();
            continue;
        } else {
            if skip {
                continue;
            }
            &ln
        };

        checks += 1;
        if lookup(word) == Some(correct_emoji) {
            eprintln!(
                "collision! {} | {} vs. {}",
                correct_emoji, correct_word, word
            );
        }
    }

    // also check for fat finger errors

    for word in SHORTCODES {
        correct_word = word.to_string();
        correct_emoji = lookup(word).unwrap();

        let mut word = word.as_bytes().to_vec();
        let mut orig;
        for i in 0..word.len() {
            orig = word[i];

            // check fat-fingers

            // for key in kbd.neighbors(kbd.find_key(word[i] as char).unwrap()).map(|k|
            // k.value) {
            for key in ' '..='~' {
                if key as u8 == orig {
                    continue;
                }

                word[i] = key as u8;
                let bad_word = String::from_utf8(word.clone()).unwrap();

                checks += 1;
                if lookup(&bad_word) == Some(correct_emoji) {
                    eprintln!(
                        "collision! {} | {} vs. {}",
                        correct_emoji, correct_word, bad_word
                    );
                }
            }

            // check accidental insertions
            for key in ' '..='~' {
                let mut bad_word = word.clone();
                bad_word.insert(i, key as u8);

                let bad_word = String::from_utf8(bad_word).unwrap();

                checks += 1;
                if lookup(&bad_word) == Some(correct_emoji) {
                    eprintln!(
                        "collision! {} | {} vs. {}",
                        correct_emoji, correct_word, bad_word
                    );
                }
            }

            word[i] = orig;
        }
    }

    // multiple random mispellings?
    for word in SHORTCODES {
        correct_word = word.to_string();
        correct_emoji = lookup(word).unwrap();

        let mut word = word.as_bytes().to_vec();

        let pick_two = (0..word.len()).choose_multiple(&mut rng, 2);
        let mut pick_two = pick_two.iter();

        let first = pick_two.next().cloned().unwrap();

        let orig = word[first];
        for key in ' '..='~' {
            if key as u8 == orig {
                continue;
            }
            word[first] = key as u8;

            let second = match pick_two.clone().next() {
                Some(v) => *v,
                None => continue,
            };

            let orig = word[second];
            for key in ' '..='~' {
                if key as u8 == orig {
                    continue;
                }
                word[second] = key as u8;

                let bad_word = String::from_utf8(word.clone()).unwrap();

                checks += 1;
                if lookup(&bad_word) == Some(correct_emoji) {
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

    Ok(())
}
