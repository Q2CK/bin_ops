mod bit_ops;

use bit_ops::*;

fn main() {
    let data1 = DataWord::from(127, 8);
    let data2 = DataWord::from(1, 8);
    println!("{:?}", data1 + data2);
    println!("{:?}", data1 - data2);
    println!("{:?}", data1 & data2);
    println!("{:?}", !(data1 | data2));
    println!("{:?}", data1 ^ data2);
    println!("{:?}", DataWord::rsh(data1));
}
