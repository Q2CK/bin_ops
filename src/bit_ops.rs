#![allow(dead_code)]

use std::cmp::min;
use std::ops;

type ContentType = usize;

#[derive(Debug, Copy, Clone)]
pub struct ALUFlags {
    pub carry:        bool,
    pub not_carry:    bool,
    pub overflow:     bool,
    pub not_overflow: bool,
    pub zero:         bool,
    pub not_zero:     bool,
    pub even:         bool,
    pub odd:          bool
}

#[derive(Debug, Copy, Clone)]
pub struct CMPFlags {
    pub greater:       bool,
    pub less_equal:    bool,
    pub less:          bool,
    pub greater_equal: bool,
    pub equal:         bool,
    pub not_equal:     bool,
    pub always_false:  bool,
    pub always_true:   bool
}

#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub content: ContentType,
    pub width:   u32,
    pub flags:   ALUFlags,
}

#[allow(arithmetic_overflow)]
impl ops::Add for Data {
    type Output = Data;

    fn add(self, rhs: Data) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint).wrapping_add(rhs.content % constraint);

        let properties = ALUFlags {
            carry:        (0 as ContentType).wrapping_sub(constraint) & result != 0,
            not_carry:    (0 as ContentType).wrapping_sub(constraint) & result == 0,
            overflow:     ((0 as ContentType).wrapping_sub(constraint) & result != 0)
                          ^ ((0 as ContentType).wrapping_sub(2 * constraint) & result != 0),
            not_overflow: !((0 as ContentType).wrapping_sub(constraint) & result != 0)
                          ^ ((0 as ContentType).wrapping_sub(2 * constraint) & result != 0),
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::Sub for Data {
    type Output = Data;

    fn sub(self, rhs: Data) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint).wrapping_sub(rhs.content % constraint);

        let properties = ALUFlags {
            carry:        (0 as ContentType).wrapping_sub(constraint) & result != 0,
            not_carry:    (0 as ContentType).wrapping_sub(constraint) & result == 0,
            overflow:     ((0 as ContentType).wrapping_sub(constraint) & result != 0)
                          ^ ((0 as ContentType).wrapping_sub(2 * constraint) & result != 0),
            not_overflow: !((0 as ContentType).wrapping_sub(constraint) & result != 0)
                          ^ ((0 as ContentType).wrapping_sub(2 * constraint) & result != 0),
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::BitAnd for Data {
    type Output = Data;

    fn bitand(self, rhs: Self) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) & (rhs.content % constraint);

        let properties = ALUFlags {
            carry:        false,
            not_carry:    false,
            overflow:     false,
            not_overflow: false,
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::BitOr for Data {
    type Output = Data;

    fn bitor(self, rhs: Self) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) | (rhs.content % constraint);

        let properties = ALUFlags {
            carry:        false,
            not_carry:    false,
            overflow:     false,
            not_overflow: false,
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::BitXor for Data {
    type Output = Data;

    fn bitxor(self, rhs: Self) -> Data {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint) ^ (rhs.content % constraint);

        let properties = ALUFlags {
            carry:        false,
            not_carry:    false,
            overflow:     false,
            not_overflow: false,
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::Not for Data {
    type Output = Data;

    fn not(self) -> Data {
        let bit_width = self.width;
        let constraint = ContentType::pow(2, bit_width);

        let result = !self.content % constraint;

        let properties = ALUFlags {
            carry:        false,
            not_carry:    false,
            overflow:     false,
            not_overflow: false,
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        Data {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl Data {
    pub(crate) fn from(mut content: ContentType, width: u32) -> Data {
        let constraint = ContentType::pow(2, width);

        content %= constraint;

        Data {
            content,
            width,
            flags: ALUFlags {
                carry:        false,
                not_carry:    false,
                overflow:     false,
                not_overflow: false,
                zero:         content == 0,
                not_zero:     content != 0,
                even:         content % 2 == 0,
                odd:          content % 2 != 0
            }
        }
    }

    fn cmp(&self, rhs: &Data) -> CMPFlags {
        CMPFlags {
            greater: self.content > rhs.content,
            less_equal: self.content <= rhs.content,
            less: self.content < rhs.content,
            greater_equal: self.content >= rhs.content,
            equal: self.content == rhs.content,
            not_equal: self.content != rhs.content,
            always_false: false,
            always_true: true,
        }
    }
}