use std::fmt;
use std::iter::Sum;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Digit {
    Equal = -2,
    Minus = -1,
    Zero = 0,
    One = 1,
    Two = 2,
}
use Digit::*;

impl Digit {
    fn parse(c: char) -> Digit {
        match c {
            '=' => Equal,
            '-' => Minus,
            '0' => Zero,
            '1' => One,
            '2' => Two,
            _ => panic!(),
        }
    }

    fn carrying_add(self, rhs: Digit, carry: Digit) -> (Digit, Digit) {
        match (self as i8) + (rhs as i8) + (carry as i8) {
            -5 => (Zero, Minus),
            -4 => (One, Minus),
            -3 => (Two, Minus),
            -2 => (Equal, Zero),
            -1 => (Minus, Zero),
            0 => (Zero, Zero),
            1 => (One, Zero),
            2 => (Two, Zero),
            3 => (Equal, One),
            4 => (Minus, One),
            5 => (Zero, One),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Equal => write!(f, "="),
            Minus => write!(f, "-"),
            Zero => write!(f, "0"),
            One => write!(f, "1"),
            Two => write!(f, "2"),
        }
    }
}

#[derive(Debug)]
struct Number {
    d: Vec<Digit>,
}

impl Number {
    fn parse(s: &str) -> Number {
        Number {
            d: s.chars().rev().map(Digit::parse).collect(),
        }
    }
}

impl Add for &Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut d = vec![];
        let mut carry = Zero;
        let mut si = self.d.iter();
        let mut ri = rhs.d.iter();
        loop {
            let so = si.next().copied();
            let ro = ri.next().copied();
            if so == None && ro == None {
                break;
            }
            let (v, c) = so.unwrap_or(Zero).carrying_add(ro.unwrap_or(Zero), carry);
            d.push(v);
            carry = c;
        }
        if carry != Zero {
            d.push(carry);
        }
        Number { d }
    }
}

impl<'a> Sum<&'a Number> for Number {
    fn sum<I: Iterator<Item = &'a Number>>(iter: I) -> Self {
        let mut r = Number { d: vec![Zero] };
        for n in iter {
            r = &r + n;
        }
        r
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in self.d.iter().rev() {
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}

fn main() {
    let nums: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .map(|l| Number::parse(&l))
        .collect();
    println!("Part 1: {}", nums.iter().sum::<Number>());
}
