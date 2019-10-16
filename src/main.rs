use crate::gauss::{gauss_elimination, Mat};

mod zero;
mod gauss;

fn main() {
    let mut mat = [
        1.0, 2.0, 3.0, 4.0,
        2.0, 3.0, 4.0, 5.0,
        3.0, 4.0, 5.0, 6.0,
    ];
    let m = Mat::new(&mut mat, 4);
    gauss_elimination(m);
    println!("{:#?}", mat);
}