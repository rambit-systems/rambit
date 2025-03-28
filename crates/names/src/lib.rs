//! Generates names.
//!
//! Really, this crate is delightfully simple. I stole all the names from
//! Zellij. I'm not even sorry; they're great names. (Zellij is MIT licensed,
//! so I'm allowed to do that.)
//!
//! This is mainly used to generate random worker names at runtime.

const ADJECTIVES: &[&str] = &[
  "adamant",
  "adept",
  "adventurous",
  "arcadian",
  "auspicious",
  "awesome",
  "blossoming",
  "brave",
  "charming",
  "chatty",
  "circular",
  "considerate",
  "cubic",
  "curious",
  "delighted",
  "didactic",
  "diligent",
  "effulgent",
  "erudite",
  "excellent",
  "exquisite",
  "fabulous",
  "fascinating",
  "friendly",
  "glowing",
  "gracious",
  "gregarious",
  "hopeful",
  "implacable",
  "inventive",
  "joyous",
  "judicious",
  "jumping",
  "kind",
  "likable",
  "loyal",
  "lucky",
  "marvellous",
  "mellifluous",
  "nautical",
  "oblong",
  "outstanding",
  "polished",
  "polite",
  "profound",
  "quadratic",
  "quiet",
  "rectangular",
  "remarkable",
  "rusty",
  "sensible",
  "sincere",
  "sparkling",
  "splendid",
  "stellar",
  "tenacious",
  "tremendous",
  "triangular",
  "undulating",
  "unflappable",
  "unique",
  "verdant",
  "vitreous",
  "wise",
  "zippy",
];

const NOUNS: &[&str] = &[
  "aardvark",
  "accordion",
  "apple",
  "apricot",
  "bee",
  "brachiosaur",
  "cactus",
  "capsicum",
  "clarinet",
  "cowbell",
  "crab",
  "cuckoo",
  "cymbal",
  "diplodocus",
  "donkey",
  "drum",
  "duck",
  "echidna",
  "elephant",
  "foxglove",
  "galaxy",
  "glockenspiel",
  "goose",
  "hill",
  "horse",
  "iguanadon",
  "jellyfish",
  "kangaroo",
  "lake",
  "lemon",
  "lemur",
  "magpie",
  "megalodon",
  "mountain",
  "mouse",
  "muskrat",
  "newt",
  "oboe",
  "ocelot",
  "orange",
  "panda",
  "peach",
  "pepper",
  "petunia",
  "pheasant",
  "piano",
  "pigeon",
  "platypus",
  "quasar",
  "rhinoceros",
  "river",
  "rustacean",
  "salamander",
  "sitar",
  "stegosaurus",
  "tambourine",
  "tiger",
  "tomato",
  "triceratops",
  "ukulele",
  "viola",
  "weasel",
  "xylophone",
  "yak",
  "zebra",
];

use nanorand::Rng;

fn pick_from(options: &[&'static str]) -> &'static str {
  let mut rng = nanorand::tls_rng();
  let index = rng.generate::<usize>() % options.len();
  options[index]
}

/// Picks a random noun from the list.
pub fn noun() -> &'static str { pick_from(NOUNS) }
/// Picks a random adjective from the list.
pub fn adjective() -> &'static str { pick_from(ADJECTIVES) }

/// Assembles a name from a random noun and adjective.
pub fn name() -> String {
  let mut rng = nanorand::tls_rng();
  let adjective_index = rng.generate::<usize>() % ADJECTIVES.len();
  let noun_index = rng.generate::<usize>() % NOUNS.len();
  format!("{}-{}", ADJECTIVES[adjective_index], NOUNS[noun_index])
}
