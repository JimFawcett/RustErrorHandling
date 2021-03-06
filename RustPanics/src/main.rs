/////////////////////////////////////////////////////////////
// rust_panics::main.rs                                    //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 27 Apr 2020  //
/////////////////////////////////////////////////////////////
/*
   Demonstrates some of the many ways to induce a panic.
   - indexing, overflow, divide by zero, non utf8 string
   - provides wrapper that traps panic without terminating
*/
use std::panic;
use std::panic::*;
use std::io::*;
use std::fmt::*;

#[allow()]
/*-- panics in both debug and release builds --*/
#[allow(dead_code)]
fn index_out_of_bounds() {
    print!("\n  ");
    let v = vec![1,2,3];
    let _item = v[3];
}
/*-- panics in debug build, does not in release build --*/
#[allow(dead_code)]
fn integer_overflow() {
    print!("\n  size of u8 = {}", std::mem::size_of::<u8>());
    let mut i:u8 = 255;
    print!("\n  i:u8 = {}", i);
    print!("\n  ");
    i += 1;
    print!("i:u8 = {}\n  ", i);
}
/*-- panics in both debug and release builds --*/
#[allow(dead_code)]
fn divide_by_zero() {
    print!("\n  ");
    let zero = 0;
    let calc = 1/zero;
    print!("\ncalc = {}", calc);
}
/*-- panics in both debug and release builds --*/
#[allow(dead_code)]
fn initialize_str_with_non_utf8() {
    print!("\n  ");
    let _invalid = vec![0xED, 0xA0, 0x80];
    let cvt_str = String::from_utf8(_invalid).unwrap();
    print!("\ncvt_str = {:?}", cvt_str);
}
#[allow(dead_code)]
fn do_panic() {
    print!("\n  ");
    panic!("\nintentional panic");
}
fn convert_string_to_int(s:&str) -> i32 {
    // print!("\n  ");
    s.parse::<i32>().unwrap()
}
/*-- traps panic, execution continues --*/
#[allow(dead_code)]
fn trap_panic(f:fn(), name:&str) -> std::io::Result<()> {
    let default_hook = panic::take_hook();
    set_panic_hook();
    let rslt = panic::catch_unwind(|| { f() });
    panic::set_hook(default_hook);
    match rslt {
        Ok(()) => {
            Ok(())
        }
        Err(_) => {
            let arg = format!("{:?} panic", name);
            let error = std::io::Error::new(ErrorKind::Other, arg);
            Err(error)
        }
    }
}
/*-------------------------------------------------
   traps panic, execution continues
   - takes function with return value
   - supply input arguments with closure
     - see example at end of main
*/
#[allow(dead_code)]
fn trap_panic_return<F: FnOnce() -> R + UnwindSafe, R>(f:F, name:&str) -> std::io::Result<R>
        where R:Debug + Clone {
    let default_hook = panic::take_hook();
    set_panic_hook();
    let rslt = panic::catch_unwind(|| -> R { f() });
    panic::set_hook(default_hook);
    match &rslt {
        Ok(r) => {
            Ok(r.clone())
        },
        Err(_) => {
            let arg = format!("{:?} panic", name);
            let error = std::io::Error::new(ErrorKind::Other, arg);
            Err(error)
        }
    }
}
/*-- elides default panic message --*/
#[allow(dead_code)]
fn set_panic_hook() {
    panic::set_hook(Box::new(|_| print!("")));
}
fn show_result(r:std::io::Result<()>) {
    match r {
        Ok(()) => print!("\n  {}", &"call succeeded"),
        Err(error) => print!("\n  call failed: {}", error),
    }
}
/*-------------------------------------------------
   tests some of the many ways to panic
   - view a case by uncommenting
*/
fn main() {
    print!("\n  {}",&"-- testing panics --");
    let _ = std::io::stdout().flush();
    // do_panic();
    // let r = trap_panic(do_panic, "do_panic()");
    // show_result(r);
    // index_out_of_bounds();
    let r = trap_panic(index_out_of_bounds, "index_out_of_bounds()");
    show_result(r);
    // divide_by_zero();
    // let r = trap_panic(divide_by_zero, "divide_by_zero()");
    // show_result(r);
    // integer_overflow();
    // let r = trap_panic(integer_overflow, "integer_overflow");
    // show_result(r);
    // initialize_str_with_non_utf8();
    // let fp = initialize_str_with_non_utf8;
    // let r = trap_panic(fp, "initialize_str_with_non_utf8");
    // show_result(r);
    
    // --------------------------------------------
    // trap panic for string to int conversion
    // uses return computed value
    // --------------------------------------------
    let s = String::from("-3");
    // let s = String::from("-3.5");
    let l = || -> i32 { convert_string_to_int(&s) };
    let name = "convert_string_to_int";
    print!("\n  ");
    let rslt = trap_panic_return(l, name);
    if rslt.is_ok() {
        print!("\n  {:?}\n  returned {}", name, rslt.unwrap());
    }
    else {
        print!("\n  {}", rslt.unwrap_err());
    }
    //---------------------------------------------
    println!("\n\n  That's all Folks!\n");
}
