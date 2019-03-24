extern crate walkdir;
extern crate glob;

use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;
use std::{env, process};
use std::io::{self, Read, Write};
use std::fs::{self, File};
use walkdir::WalkDir;

// use glob::glob;

fn main() {
    let dir = env::args().nth(1)
        .ok_or("Please supply a dir")
        .unwrap_or_else(|e| {
          writeln!(&mut io::stderr(), "{}", e).expect("Could not write to stderr");
          process::exit(1)
        });

    let mut count = BTreeMap::new();

    for entry in WalkDir::new(dir) {
        let f_count = check_is_file(entry);
        if let Ok(f_count) = f_count {
            for (i, j) in f_count {
              // println!("{} {}", i, j);
              *count.entry(i).or_insert(0) += j;
            }
        }
        
    }

    println!("Number of occurences per character");
    for (ch, count) in &count {
        println!("{:?}: {}", ch, count);
    }
}

fn check_is_file (entry: Result<walkdir::DirEntry, walkdir::Error>) -> Result<BTreeMap<char, usize>, std::io::Error> {
  let entry = entry.unwrap();
  let path = entry.path();
  let metadata = fs::metadata(&path)?;
  let is_file = metadata.is_file();

  let mut accepted_exts = HashSet::new();
  accepted_exts.insert("js");
  accepted_exts.insert("ts");
  accepted_exts.insert("tsx");

  let mut is_matching_file = false;
  if is_file {
    let ext = path.extension();
    match ext {
      None => {}
      _ => {
        let ext_str = ext.unwrap().to_str().unwrap();
        is_matching_file = accepted_exts.contains(&ext_str);
      },
    }
    
  }

  let mut count = BTreeMap::new();
  match is_file && is_matching_file {
    true => {
      let mut buf = String::new();
      let path = entry.path();
      let file = File::open(&path);

      try!(file.unwrap().read_to_string(&mut buf));
  
      for c in buf.chars() {
          let clow = c.to_ascii_lowercase();
          *count.entry(clow).or_insert(0) += 1;
      }

    }
    false => println!("Found Dir Or Non-Matching File {} {}", is_file, is_matching_file)
  }
  
  return Ok(count);
}