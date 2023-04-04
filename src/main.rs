mod bit_ops;

use bit_ops::*;

fn main() {
    let data1 = Data::from(5, 8);
    let data2 = Data::from(5, 8);
    println!("{:?}", data1 + data2);
    println!("{:?}", data1 - data2);
    println!("{:?}", data1 & data2);
    println!("{:?}", !(data1 | data2));
    println!("{:?}", data1 ^ data2);
}
