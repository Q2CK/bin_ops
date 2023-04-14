#![allow(dead_code)]

use std::cmp::min;
use std::ops;

type ContentType = usize;

#[derive(Debug, Copy, Clone)]
pub struct ALUFlags {
    pub carry:         bool,
    pub not_carry:     bool,
    pub overflow:      bool,
    pub not_overflow:  bool,
    pub zero:          bool,
    pub not_zero:      bool,
    pub even:          bool,
    pub odd:           bool
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
pub struct DataWord {
    pub content: ContentType,
    pub width:   u32,
    pub flags:   ALUFlags,
}

#[allow(arithmetic_overflow)]
impl ops::Add for DataWord {
    type Output = DataWord;

    fn add(self, rhs: DataWord) -> DataWord {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint).wrapping_add(rhs.content % constraint);
        let result_cut_msb = (self.content % (constraint / 2)).wrapping_add(rhs.content % (constraint / 2));

        let properties = ALUFlags {
            carry:        constraint & result != 0,
            not_carry:    constraint & result == 0,
            overflow:     (constraint & result != 0)
                          ^ ((constraint / 2) & result_cut_msb != 0),
            not_overflow: (constraint & result == 0)
                          ^ ((constraint / 2) & result_cut_msb != 0),
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        DataWord {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::Sub for DataWord {
    type Output = DataWord;

    fn sub(self, rhs: DataWord) -> DataWord {
        let bit_width = min(self.width, rhs.width);
        let constraint = ContentType::pow(2, bit_width);

        let result = (self.content % constraint).wrapping_add(!rhs.content % constraint).wrapping_add(1);
        let result_cut_msb = (self.content % (constraint / 2)).wrapping_add(rhs.content % (constraint / 2));

        let properties = ALUFlags {
            carry:        constraint & result != 0,
            not_carry:    constraint & result == 0,
            overflow:     (constraint & result != 0)
                          ^ ((constraint / 2) & result_cut_msb != 0),
            not_overflow: (constraint & result == 0)
                          ^ ((constraint / 2) & result_cut_msb != 0),
            zero:         result % constraint == 0,
            not_zero:     result % constraint != 0,
            even:         result % 2 == 0,
            odd:          result % 2 != 0
        };

        DataWord {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::BitAnd for DataWord {
    type Output = DataWord;

    fn bitand(self, rhs: Self) -> DataWord {
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

        DataWord {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::BitOr for DataWord {
    type Output = DataWord;

    fn bitor(self, rhs: Self) -> DataWord {
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

        DataWord {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::BitXor for DataWord {
    type Output = DataWord;

    fn bitxor(self, rhs: Self) -> DataWord {
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

        DataWord {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl ops::Not for DataWord {
    type Output = DataWord;

    fn not(self) -> DataWord {
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

        DataWord {
            content: result % constraint,
            width:   bit_width,
            flags:   properties
        }
    }
}

#[allow(arithmetic_overflow)]
impl DataWord {
    pub fn from(mut content: ContentType, width: u32) -> DataWord {
        let constraint = ContentType::pow(2, width);

        content %= constraint;

        DataWord {
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

    pub fn rsh(val: DataWord) -> DataWord {
        let constraint = ContentType::pow(2, val.width);

        let temp_overflow = val.content % 2 == 1;

        let result = (val.content >> 1) % constraint;

        DataWord {
            content: result,
            width: val.width,
            flags: ALUFlags {
                carry:        false,
                not_carry:    false,
                overflow:     temp_overflow,
                not_overflow: false,
                zero:         result == 0,
                not_zero:     result != 0,
                even:         result % 2 == 0,
                odd:          result % 2 != 0,
            }
        }
    }

    pub fn cmp(&self, rhs: &DataWord) -> CMPFlags {
        CMPFlags {
            greater:       self.content > rhs.content,
            less_equal:    self.content <= rhs.content,
            less:          self.content < rhs.content,
            greater_equal: self.content >= rhs.content,
            equal:         self.content == rhs.content,
            not_equal:     self.content != rhs.content,
            always_false:  false,
            always_true:   true,
        }
    }
}