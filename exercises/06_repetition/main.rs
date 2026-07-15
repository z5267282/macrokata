////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `if_any!()` macro.
macro_rules! if_any {
    ($( $boolean:expr ),+; $code:block) => {
        if $( $boolean )||+ $code
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })

    /*
    if false || 0 == 1 || true {
        print_sucess
    }

    */
}
