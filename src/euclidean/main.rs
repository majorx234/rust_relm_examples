use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use std::cmp;

fn greatest_common_deivsor(a_in: u32, b_in: u32) -> u32 {
    let mut a = cmp::max(a_in, b_in);
    let mut b = cmp::min(a_in, b_in);

    while b != 0 {
        let t = b;
        b = a.rem_euclid(b);
        a = t;
    }
    return a;
}

fn main() {
    let a: u32 = 16;
    let b: u32 = 24;
    let c: u32 = greatest_common_deivsor(a, b);
    println!("gcd of {} and {} is {}", a, b, c);
}
