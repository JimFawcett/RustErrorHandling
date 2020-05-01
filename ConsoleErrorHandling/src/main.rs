/////////////////////////////////////////////////////////////
// console_error_handlingg::main.rs                         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Apr 2020  //
/////////////////////////////////////////////////////////////
 
// https://users.rust-lang.org/t/how-to-get-user-input/5176/2

use std::panic;
use std::io::*;
/*-- traps panic, execution continues --*/
#[allow(dead_code)]
fn strip_newline(s:&mut String) {
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
}
#[allow(dead_code)]
fn test() {
    panic!("\n  test panicked");
}
fn main() -> Result<()> {    

    /*-- reading from stdin --*/
    let mut s=String::new();
    use std::io::*;
    print!("\n  Please enter some text: ");
    let _ = std::io::stdout().flush();
    //std::io::stdout().flush();
    let rslt = std::io::stdin().read_line(&mut s);
    match rslt {
        Ok(bytes) => {
            strip_newline(&mut s);
            print!("\n  you typed {:?} using {} bytes\n", s, bytes);
        },
        Err(error) => print!("\n  your input failed with error: {:?}\n", error),
    }

    print!("\n  --- testing output string ---");
    let _valid = vec![0x61, 0x62, 0x63];
    let _invalid = vec![0xED, 0xA0, 0x80];
    let arg = _valid;
    /*-- to see both cases try _valid and _invalid --*/
    let _result;
    let cvt_str_rslt = String::from_utf8(arg);
    if cvt_str_rslt.is_ok() {
        let s:String = cvt_str_rslt.unwrap();
        let bytes = s.as_bytes();
        std::io::stdout().write_all(b"\n  writing: ")?;
        std::io::stdout().write_all(&bytes)?;
        _result = Ok(());
    }
    else {
        let error = cvt_str_rslt.unwrap_err();
        print!("\n  {}", error);
        _result = Err(std::io::Error::new(ErrorKind::Other, "console write error"));
    }
    /////////////////////////////////////////////////////////
    // Using _invalid in code below panics at write_all, 
    // never returns Result.
    //   That is a bug in std::io::stdout() for Windows
    //-------------------------------------------------------
    print!("\n\n  --- testing write result ---\n");
    let _valid = &[0x61, 0x62, 0x63];
    let _invalid = &[0xED, 0xA0, 0x80];
    std::io::stdout().write(b"\n  writing: ")?;
    let arg = _valid;  
    // setting arg = _invalid
    // results in untrappable panic, e.g., panic while 
    // panicing 
    /////////////////////////////////////////////////////////
    // The code below traps panics in Rust code, but 
    // apparently not when calling into foreign code,
    // like Windows console.
    //-------------------------------------------------------
    // let _result = panic::catch_unwind(
    //     || -> std::io::Result<()> {
    //         {
    //             std::io::stdout().write_all(arg)
    //         }
    //     }
    // );
    let _result = std::io::stdout().write_all(arg);
    if _result.is_err() {
        let error = _result.unwrap_err();
        print!("\n  could not write invalid, error: {:?}", error);
    }
    else {
        print!("\n  wrote {:?}", arg);
    }

    println!("\n\n  That's all Folks!\n");
    Ok(())
}
