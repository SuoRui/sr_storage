use std::{thread, time};

mod disk;

macro_rules! mod_init {
    ($($m:tt),*) => {
        $(
            print!("\tStart init {} ... ", stringify!($m));
            match $m::init() {
                Ok(_) => println!("[ OK ]"),
                Err(e) => {
                    println!("[ {e} ]");
                    return
                }
            }
        )*
    };
}

fn main() {
    println!("Init System:");

    mod_init![disk];

    println!("Done!!!");
    // let sleep_time = time::Duration::from_secs(1);
    // loop {
    //     thread::sleep(sleep_time);
    // }
}
