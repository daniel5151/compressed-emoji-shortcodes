use std::collections::HashSet;

use rand::seq::IteratorRandom;

use maximally_compressed_gh_shortcodes::{lookup, RAW_PAIRS};

fn main() {
    let mut buf = [0; 8];

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
    // for (k, v) in test {
    //     if k < 7 {
    //         continue;
    //     }
    //     eprintln!("Count {}", k);
    //     for (code, e) in v {
    //         eprintln!("  {}: {}", e, code);
    //     }
    // }

    // return Ok(());

    #[allow(unused_variables)]
    let kbd = keygraph_rs::generate_qwerty_us();

    let mut rng = rand::thread_rng();

    let shortcode_set = RAW_PAIRS
        .iter()
        .map(|(a, _)| a)
        .copied()
        .collect::<HashSet<_>>();

    let mut checks = 0;

    let mut correct_emoji = String::new();
    let mut correct_word = String::new();

    // check for fat finger errors

    for word in shortcode_set.iter() {
        correct_word = word.to_string();
        correct_emoji = match lookup(word, &mut buf) {
            Some(v) => v.into(),
            None => continue,
        };

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
                if lookup(&bad_word, &mut buf) == Some(&correct_emoji) {
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
                if lookup(&bad_word, &mut buf) == Some(&correct_emoji) {
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
    for word in shortcode_set.iter() {
        correct_word = word.to_string();
        correct_emoji = match lookup(word, &mut buf) {
            Some(v) => v.into(),
            None => continue,
        };

        let mut word = word.as_bytes().to_vec();

        let pick_two = (0..word.len()).choose_multiple(&mut rng, 2);
        let mut pick_two = pick_two.iter();

        let first = pick_two.next().cloned().unwrap();

        let orig = word[first];
        for key in (' '..='~').filter(|c| c.is_ascii_alphanumeric()) {
            if key as u8 == orig {
                continue;
            }
            word[first] = key as u8;

            let second = match pick_two.clone().next() {
                Some(v) => *v,
                None => continue,
            };

            let orig = word[second];
            for key in (' '..='~').filter(|c| c.is_ascii_alphanumeric()) {
                if key as u8 == orig {
                    continue;
                }
                word[second] = key as u8;

                let bad_word = String::from_utf8(word.clone()).unwrap();

                checks += 1;
                if lookup(&bad_word, &mut buf) == Some(&correct_emoji) {
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
