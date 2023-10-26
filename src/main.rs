use std::env::args;
use std::fs::{self, read_dir};
use std::path::Path;
use std::process::exit;

fn main() {
    let mut args = args().skip(1);
    let data_dir = args.next().expect("specify path to data directory");
    let options_dir = args.next().expect("specify path to the options directory");
    println!("finding example song in {data_dir}");
    let example_xml = Path::new(&data_dir).join("A000/music/music2153/Music.xml");
    assert!(example_xml.exists() && example_xml.is_file());
    let portion = fs::read_to_string(example_xml).expect("failed to read example song");
    let start = portion.find("<stageName>").unwrap();
    let end = portion.find("</stageName>").unwrap() + "</stageName>".len();
    let stage = &portion[start..end];
    println!("going to copy this stage to all songs:");
    println!("{stage}");
    println!("Continue? [y/d(ebug)/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let debug = match &*input.trim().to_ascii_lowercase() {
        "y" => false,
        "d" => true,
        _ => {
            println!("aborting");
            return;
        }
    };
    process_opt(&Path::new(&data_dir).join("A000"), stage, debug);
    for entry in read_dir(&options_dir).expect("failed to read options directory") {
        let entry = entry.expect("failed to read entry");
        let path = entry.path();
        if path.is_dir() {
            process_opt(&path, stage, debug);
        }
    }
}

fn process_opt(p: &Path, portion: &str, debug: bool) {
    let music = p.join("music");
    if !music.is_dir() {
        return;
    }

    for entry in read_dir(&music).expect("failed to read music directory") {
        let entry = entry.expect("failed to read entry");
        let path = entry.path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("music") {
            process_song(&path, portion, debug);
        }
    }
}

fn process_song(p: &Path, portion: &str, debug: bool) {
    let music_xml = p.join("Music.xml");
    if !music_xml.is_file() {
        return;
    }

    let mut contents = fs::read_to_string(&music_xml).expect("failed to read music xml");
    let start = contents.find("<stageName>").unwrap();
    let end = contents.find("</stageName>").unwrap() + "</stageName>".len();
    contents.insert_str(end, "\n-->");
    contents.insert_str(start, "\n<!-- PWNED BY FPSFIXER!!\n");
    contents.insert_str(start, portion);
    fs::write(&music_xml, contents).expect("failed to write music xml");
    println!("wrote to {}", music_xml.display());
    if debug {
        exit(0);
    }
}


