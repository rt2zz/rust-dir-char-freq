extern crate walkdir;

use std::collections::btree_map::BTreeMap;
use std::{env, process};
use std::io::{self, Read, Write};
use std::fs::{self, File};
use walkdir::WalkDir;
 
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

  let mut count = BTreeMap::new();
  match is_file {
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
    false => println!("Found Dir")
  }
  
  return Ok(count);
}