use rust_macros_learning_record::make_answer;
use rust_macros_learning_record::MyDerive;
use rust_macros_learning_record::my_attribute;
use rust_macros_learning_record::show_streams;
use rust_macros_learning_record::HelperAttr;
use rust_macros_learning_record::run_time;
use std::time::Duration;
use std::thread;

make_answer!();

#[derive(MyDerive, Debug)]
struct World;

#[my_attribute{ delimiters }]
struct AAA;

#[show_streams]
fn invoke1() {}

#[show_streams("bar")]
fn invoke2() {}

#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams { delimiters }]
fn invoke4() {}

#[derive(HelperAttr)]
struct Struct {
    #[helper] field: ()
}

#[run_time]
fn deco(t: u64) {
    let secs = Duration::from_secs(t);
    thread::sleep(secs);
}

fn main() {
    println!("{}", answer());
    deco(3);
    let a = Hello2;
    println!("{:?}", a);
    invoke1();
}
