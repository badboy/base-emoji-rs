#[macro_use]
extern crate serde_derive;
extern crate phf_codegen;
extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

#[derive(Deserialize)]
struct Emoji {
    #[serde(rename(deserialize = "char"))]
    character: String,
    name: String,
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("emojis.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut emoji_file = File::open("emojis.json").expect("Can't open emoji file");
    let mut buffer = String::new();
    emoji_file
        .read_to_string(&mut buffer)
        .expect("Can't read emoji file");

    let emojis: Vec<Emoji> = serde_json::from_str(&buffer).expect("Invalid JSON in emoji file");

    write!(
        &mut file,
        "/// Compile time generated lookup table for Emojis.\n"
    )
    .unwrap();
    write!(&mut file, "/// \n").unwrap();
    write!(
        &mut file,
        "static EMOJIS: phf::Map<u8, (&'static str, &'static str)> = "
    )
    .unwrap();

    let mut m = phf_codegen::Map::new();

    assert!(emojis.len() >= 255);

    for (idx, emoji) in emojis.iter().enumerate().take(256) {
        let out = format!("({:?}, {:?})", emoji.character, emoji.name);
        m.entry(idx as u8, &out);
    }

    m.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(
        &mut file,
        "/// Compile time generated reverse lookup table for Emojis.\n"
    )
    .unwrap();
    write!(&mut file, "/// \n").unwrap();
    write!(
        &mut file,
        "static EMOJIS_REVERSE: phf::Map<&'static str, u8> = "
    )
    .unwrap();

    let mut m = phf_codegen::Map::new();

    assert!(emojis.len() >= 255);

    for (idx, emoji) in emojis.iter().enumerate().take(256) {
        m.entry(emoji.character.as_str(), &format!("{}u8", idx as u8));
    }

    m.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
