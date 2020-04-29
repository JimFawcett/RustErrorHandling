/////////////////////////////////////////////////////////////
// error_handling_ops::main.rs                             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 27 Apr 2020  //
/////////////////////////////////////////////////////////////
/*
   Demonstrates error handling techniques.
   - is_ok, match, if let, ?
   - illustrates creation of custom error type
*/
use std::io::*;

#[derive(Debug, Clone)]
struct CustomError {
    msg:String,
}
#[allow(dead_code)]
impl CustomError {
    fn new(s:&str) -> Self {
        Self {
            msg:s.to_string(),
        }
    }
    fn error(&self) -> &str {
        &self.msg
    }
}
#[allow(dead_code)]
fn always_fails() -> std::result::Result<(),CustomError> {
    let error = CustomError::new("failure test");
    Err(error)  // return error
}
#[allow(dead_code)]
fn always_succeeds() -> std::result::Result<(),CustomError> {
    Ok(())      // return unit result
}
/*-- main's return type supports try operator ? --*/
fn main() -> std::result::Result<(),CustomError> {

    /*-- uses is_ok --*/
    print!("\n  using is_ok()");
    let rslt = always_succeeds();
    if rslt.is_ok() {
        print!("\n  function always_succeeds succeeded");
    }
    else {
        print!("\n  function always_succeeds failed!");
    }
    let _ = std::io::stdout().flush();

    /*-- uses match --*/
    let rslt = always_fails();
    print!("\n\n  using match:");
    match rslt {
        Ok(()) => print!("\n  function always_fails succeeded!\n"),
        Err(error) => {
            print!("\n  function always_fails failed");
            print!("\n    - error message: {:?}\n", error.msg)
        }
    }
    let _ = std::io::stdout().flush();

    /*-- uses if let --*/
    let _rslt = always_fails();
    print!("\n  using if let:");
    /*--  "=" is match operator, not assignment --*/
    if let Ok(()) = _rslt {
        print!("\n  function always_fails succeeded");
    }
    else {
        let error:CustomError = _rslt.unwrap_err();
        print!("\n  function always_fails failed with message:\n  {:#?}", error.msg);
    }
    let _ = std::io::stdout().flush();

    /*-- uses try operator ? to bubble up error --*/
    print!("\n\n  using try operator ?\n");
    always_fails()?;

    println!("\n\n  That's all Folks!\n");
    Ok(())
}
