//Quick and dirty app that will make a simple html file from an .rs source file

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
// use std::process::Command;

//main with passed filename
fn main() {

if env::args().len() < 2 {
    println!("Usage: rs2html <filename.rs>");
    return;
  }

  //get the passed filename
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  //get the file contents
  let mut file = fs::File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  //create the html file
  let mut html_file = fs::File::create(Path::new(filename).with_extension("html")).unwrap();

  //write the html file
  html_file.write_all(b"<!DOCTYPE html>").unwrap();

  // header
  html_file.write_all(b"<HEAD>").unwrap();
  html_file.write_all(b"<TITLE>").unwrap();
  html_file.write_all(filename.as_bytes()).unwrap();
  html_file.write_all(b"</TITLE>").unwrap();
  html_file.write_all(b"</HEAD>").unwrap();

  // body
  html_file.write_all(b"<BODY>").unwrap();
  html_file.write_all(b"<PRE>").unwrap();
  html_file.write_all(contents.as_bytes()).unwrap();
  html_file.write_all(b"</PRE>").unwrap();
  html_file.write_all(b"</BODY>").unwrap();
  
  html_file.write_all(b"\n").unwrap();
  html_file.write_all(b"</html>").unwrap();

  //done
  println!("Done!");
}
