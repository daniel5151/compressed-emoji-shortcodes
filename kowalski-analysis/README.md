# kowalski-analysis

![](https://i.imgur.com/ilpDjuN.jpg)

A quick and _very_ dirty crate to run various experiments on the raw and compressed datasets (e.g: checking for invalid mappings, false positives, rough compression rates, etc...).

A typical output might look something like this:

```
average shortcode length: 10.127165
average emoji length: 6.004329

total (shortcode, emoji) pairs: 1848
raw emoji char data: 11096

raw shortcode char data: 18715
total checksums bytes: 1848
total displacement-table data (in bytes): 2976

trimmed tables: 10 (roughly -68 * n = 680)
total padding: 409
   net: 271

fixup table overhead: 228
  bowing_woman | 🙇‍♀️
  couplekiss_man_man | 👨‍❤️‍💋‍👨
  detective | 🕵️
  malawi | 🇲🇼


-------------------------------

(no invalid mappings found)


-------------------------------


collision! ⛹️‍♂️ | basketball_man vs. basketbkzll_man
collision! 👷‍♀️ | construction_worker_woman vs. constructiopz_worker_woman
collision! 🇬🇶 | equatorial_guinea vs. equabzorial_guinea
collision! ➖ | heavy_minus_sign vs. heavy_minus_simzn
collision! 🧘‍♂️ | lotus_position_man vs. lofzus_position_man
collision! 🏩 | love_hotel vs. lazve_hotel
collision! 🇲🇲 | myanmar vs. myaxzmar
collision! 🇵🇦 | panama vs. panawza
collision! 🇷🇴 | romania vs. roszania
collision! ☃️ | snowman_with_snow vs. snowman_wozth_snow
collision! 🌤️ | sun_behind_small_cloud vs. sun_bszhind_small_cloud
collision! 👩‍🦯 | woman_with_probing_cane vs. woman_with_probing_canwz


-------------------------------


collision! ⚓ | anchor vs. amchoc
collision! 🗳️ | ballot_box vs. ballotybex
collision! ⛵ | boat vs. ioar
collision! 📚 | books vs. blovs
collision! 👨‍👨‍👦 | family_man_man_boy vs. fomily_man_manyboy
collision! 🐬 | flipper vs. flbpppr
collision! 🇬🇧 | gb vs. uk
collision! 🏌️‍♀️ | golfing_woman vs. eolfing_wyman
collision! 🪦 | headstone vs. heapstoni
collision! ⛸️ | ice_skate vs. oce_smate
collision! 📥 | inbox_tray vs. inboy_trpy
collision! 🤦‍♂️ | man_facepalming vs. mhk_facepalming
collision! 👨‍💼 | man_office_worker vs. uan_office_workej
collision! 🚵‍♂️ | mountain_biking_man vs. vountain_bhking_man
collision! 〽️ | part_alternation_mark vs. part_altnrnationvmark
collision! ⛄ | snowman vs. suowban
collision! 🧍‍♀️ | standing_woman vs. standqngwwoman
collision! 📏 | straight_ruler vs. straight_qulec
collision! ☎️ | telephone vs. teleporne
collision! 🎟️ | tickets vs. micvets
collision! 🇬🇧 | uk vs. gb
```
