use std::cmp::min;
use std::ops;

type ContentType = usize;

struct Properties {
    carry:        bool,
    not_carry:    bool,
    overflow:     bool,
    not_overflow: bool,
    zero:         bool,
    not_zero:     bool,
    even:         bool,
    odd:          bool
}

#[allow(arithmetic_overflow)]
struct Data {
    content: ContentType,
    width:   u32,
    flags:   Properties,
}

impl ops::Add for Data {
    type Output = Data;

    fn add(self, rhs: Data) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) + (rhs.content % constraint);

        let properties = Properties {
            carry:        (0xffffffffffffffff + 1 - constraint) & result != 0,
            not_carry:    (0xffffffffffffffff + 1 - constraint) & result == 0,
            overflow:     ((0xffffffffffffffff + 1 - constraint) & result != 0)
                ^ ((0xffffffffffffffff + 1 - (2 * constraint)) & result != 0),
            not_overflow: !((0xffffffffffffffff + 1 - constraint) & result != 0)
                ^ ((0xffffffffffffffff + 1 - (2 * constraint)) & result != 0),
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width: bit_width,
            flags: properties
        }
    }
}

impl ops::Sub for Data {
    type Output = Data;

    fn sub(self, rhs: Data) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) - (rhs.content % constraint);

        let properties = Properties {
            carry: (ContentType::MAX + 1 - constraint) & result != 0,
            not_carry: (ContentType::MAX + 1 - constraint) & result == 0,
            overflow: ((ContentType::MAX + 1 - constraint) & result != 0)
                ^ ((ContentType::MAX + 1 - (2 * constraint)) & result != 0),
            not_overflow: !((ContentType::MAX + 1 - constraint) & result != 0)
                ^ ((ContentType::MAX + 1 - (2 * constraint)) & result != 0),
            zero: result % constraint == 0,
            not_zero: result % constraint != 0,
            even: result % 2 == 0,
            odd: result % 2 != 0
        };

        Data {
            content: result % constraint,
            width: bit_width,
            flags: properties
        }
    }
}

impl ops::BitAnd for Data {
    type Output = Data;

    fn bitand(self, rhs: Self) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) & (rhs.content % constraint);

        let properties = Properties {
            carry: false,
            not_carry: false,
            overflow: false,
            not_overflow: false,
            zero: result % constraint == 0,
            not_zero: result % constraint != 0,
            even: result % 2 == 0,
            odd: result % 2 != 0
        };

        Data {
            content: result % constraint,
            width: bit_width,
            flags: properties
        }
    }
}

impl ops::BitOr for Data {
    type Output = Data;

    fn bitor(self, rhs: Self) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) | (rhs.content % constraint);

        let properties = Properties {
            carry: false,
            not_carry: false,
            overflow: false,
            not_overflow: false,
            zero: result % constraint == 0,
            not_zero: result % constraint != 0,
            even: result % 2 == 0,
            odd: result % 2 != 0
        };

        Data {
            content: result % constraint,
            width: bit_width,
            flags: properties
        }
    }
}

impl ops::BitXor for Data {
    type Output = Data;

    fn bitxor(self, rhs: Self) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) ^ (rhs.content % constraint);

        let properties = Properties {
            carry: false,
            not_carry: false,
            overflow: false,
            not_overflow: false,
            zero: result % constraint == 0,
            not_zero: result % constraint != 0,
            even: result % 2 == 0,
            odd: result % 2 != 0
        };

        Data {
            content: result % constraint,
            width: bit_width,
            flags: properties
        }
    }
}

impl ops::Not for Data {
    type Output = Data;

    fn not(self) -> Data {
        let bit_width = self.width;
        let constraint = ContentType::pow(2, bit_width);

        let result = !self.content % constraint;

        let properties = Properties {
            carry: false,
            not_carry: false,
            overflow: false,
            not_overflow: false,
            zero: result % constraint == 0,
            not_zero: result % constraint != 0,
            even: result % 2 == 0,
            odd: result % 2 != 0
        };

        Data {
            content: result % constraint,
            width: bit_width,
            flags: properties
        }
    }
}

fn main() {
    println!("Hello, world!");
}
