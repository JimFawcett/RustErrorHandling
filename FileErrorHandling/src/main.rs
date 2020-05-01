/////////////////////////////////////////////////////////////
// file_error_handline::main.rs                            //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Apr 2020  //
/////////////////////////////////////////////////////////////
 
#[allow(unused_imports)]
use std::fs::{File};
use std::io::prelude::*;

#[allow(dead_code)]
struct FileOption;
impl FileOption {
    const CREATE:u8 = 1; const APPEND:u8 = 2; 
    const READ:u8 = 4; const WRITE:u8 = 8;
}

fn open_file(file_name:&str, opt: u8) -> std::io::Result<File> {
    use std::fs::OpenOptions;
    let mut f = OpenOptions::new();
    type FO = FileOption;
    if opt & FO::WRITE != 0 {
        f.write(true);
    }
    if opt & FO::READ != 0 {
        f.read(true);
    }
    if opt & FO::CREATE != 0 {
        f.create(true);
    }
    if opt & FO::APPEND != 0 {
        f.append(true);
    }
    let rslt = f.open(file_name);
    rslt
}

fn main() -> std::io::Result<()> {

    let fn1 = "file1.txt";
    type FO = FileOption;
    let rslt = open_file(fn1, FO::WRITE | FO::CREATE | FO::APPEND);
    if rslt.is_ok() {
        let mut f1 = rslt.unwrap();
        f1.write(b"abc")?;
        print!("\n  open and write {:?} succeeded", fn1);
    }
    else {
        print!("\n  open {:?} failed", fn1)
    }

    let fn2 = "does_not_exist.txt";
    let rslt = open_file(fn2, FO::WRITE | FO::APPEND);
    if rslt.is_ok() {
        print!("\n  open {:?} no create succeeded", fn2);
    }
    else {
        let error = rslt.unwrap_err();
        print!("\n  error: {:#?} {:?}", error.kind(), fn2);
    }
    // https://blog.yoshuawuyts.com/error-handling-survey/

    println!("\n\n  That's all Folks!\n");
    Ok(())
}
