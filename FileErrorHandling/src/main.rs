/////////////////////////////////////////////////////////////
// file_error_handline::main.rs                            //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Apr 2020  //
/////////////////////////////////////////////////////////////
 
#[allow(unused_imports)]
use std::fs::{File};
use std::io::prelude::*;

#[allow(dead_code)]
struct FileOptions { 
    create:u8, append:u8, read:u8, write:u8, 
}
impl Default for FileOptions {
  fn default() -> Self {
      Self {
          create:1, append:2, read:4, write:8, 
      }
  }
}

fn open_file(file_name:&str, opt: u8) -> std::io::Result<File> {
    use std::fs::OpenOptions;
   // use std::os::windows::prelude::*;
    let fo = FileOptions::default();
    let mut f = OpenOptions::new();
    if opt & fo.write != 0 {
        f.write(true);
    }
    if opt & fo.read != 0 {
        f.read(true);
    }
    if opt & fo.create != 0 {
        f.create(true);
    }
    if opt & fo.append != 0 {
        f.append(true);
    }
    let rslt = f.open(file_name);
    rslt
}

fn main() -> std::io::Result<()> {
    let fn1 = "file1.txt";
    let fo = FileOptions::default();
    let rslt = open_file(fn1, fo.write | fo.create | fo.append);
    if rslt.is_ok() {
        let mut f1 = rslt.unwrap();
        f1.write(b"abc")?;
        print!("\n  open and write {:?} succeeded", fn1);
    }
    else {
        print!("\n  open {:?} failed", fn1)
    }

    let fn2 = "does_not_exist.txt";
    let rslt = open_file(fn2, fo.write | fo.append);
    if rslt.is_ok() {
        print!("\n  open {:?} no create succeeded", fn2);
    }
    else {
        let error = rslt.unwrap_err();
        print!("\n  error: {:#?} {:?}", error.kind(), fn2);
    }
    // match &rslt {
    //     Ok(_file) => print!("\n  open {:?} no create succeeded", fn2),
    //     Err(error) => print!("\n  error: {:#?} {:?}", error.kind(), fn2),
    // }
    
    // https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/std/io/enum.ErrorKind.html
    // https://blog.yoshuawuyts.com/error-handling-survey/

    println!("\n\n  That's all Folks!\n");
    Ok(())
}
