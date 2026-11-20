use std::{env, fs};

const WINDOWS: usize = 22;
const W: u32 = 12;
const RADIX: u16 = 1 << W;
const MODULUS: Words = [
    0x43e1_f593_f000_0001,
    0x2833_e848_79b9_7091,
    0xb850_45b6_8181_585d,
    0x3064_4e72_e131_a029,
];
type Words = [u64; 4];
type Digits = [i16; WINDOWS];

fn add(a: Words, b: Words) -> Option<Words> {
    let mut carry = false;
    let sum = std::array::from_fn(|i| {
        let (partial, high) = a[i].overflowing_add(b[i]);
        let (value, extra) = partial.overflowing_add(u64::from(carry));
        carry = high || extra;
        value
    });
    (!carry).then_some(sum)
}

fn subtract(a: Words, b: Words) -> Option<Words> {
    let mut borrow = false;
    let difference = std::array::from_fn(|i| {
        let (partial, low) = a[i].overflowing_sub(b[i]);
        let (value, extra) = partial.overflowing_sub(u64::from(borrow));
        borrow = low || extra;
        value
    });
    (!borrow).then_some(difference)
}

fn chunks(words: Words) -> [u16; WINDOWS] {
    std::array::from_fn(|i| {
        let start = i * W as usize;
        let shift = start % 64;
        let limb = start / 64;
        let low = words[limb] >> shift;
        let high = if shift + W as usize > 64 && limb + 1 < 4 {
            words[limb + 1] << (64 - shift)
        } else {
            0
        };
        ((low | high) & u64::from(RADIX - 1)) as u16
    })
}

fn unsigned(words: Words) -> Digits {
    chunks(words).map(|value| value as i16)
}

/// Minimum nonzero digits over all in-range signed radix-4096 recodings.
/// At each window, a carry can only stay at zero or advance by one.
fn minimum_signed(words: Words, top_limit: u16) -> Digits {
    let base = chunks(words);
    let mut decisions = [[0; 2]; WINDOWS - 1];
    let invalid = u16::MAX / 2;
    let mut cost = std::array::from_fn(|incoming| {
        let value = base[WINDOWS - 1] + incoming as u16;
        if value > top_limit {
            invalid
        } else {
            u16::from(value != 0)
        }
    });
    for i in (0..WINDOWS - 1).rev() {
        let mut next_cost = [0; 2];
        for incoming in 0..2 {
            let value = base[i] + incoming as u16;
            let unsigned_cost = if value < RADIX {
                u16::from(value != 0) + cost[0]
            } else {
                invalid
            };
            let carry_cost = if value > 0 {
                u16::from(value != RADIX) + cost[1]
            } else {
                invalid
            };
            if carry_cost < unsigned_cost {
                next_cost[incoming] = carry_cost;
                decisions[i][incoming] = 1;
            } else {
                next_cost[incoming] = unsigned_cost;
            }
        }
        cost = next_cost;
    }
    let mut output = [0; WINDOWS];
    let mut incoming = 0;
    for i in 0..WINDOWS - 1 {
        let outgoing = decisions[i][incoming];
        output[i] = base[i] as i16 + incoming as i16 - (RADIX as i16) * outgoing as i16;
        incoming = outgoing;
    }
    output[WINDOWS - 1] = base[WINDOWS - 1] as i16 + incoming as i16;
    assert!(output[WINDOWS - 1] <= top_limit as i16);
    assert_eq!(nonzero(output) as u16, cost[0]);
    output
}

fn nonzero(digits: Digits) -> usize {
    digits.iter().filter(|&&d| d != 0).count()
}

/// Construct the exact integer represented by the digits, before applying
/// the sign of the chosen point. The carry logic is independent of selection.
fn recover(digits: Digits) -> Words {
    let mut words = [0; 4];
    let mut carry = 0;
    for (i, digit) in digits.into_iter().enumerate() {
        let value = digit + carry;
        let chunk = u64::from(value.rem_euclid(RADIX as i16) as u16);
        carry = value.div_euclid(RADIX as i16);
        let start = i * W as usize;
        let shift = start % 64;
        let limb = start / 64;
        words[limb] |= chunk << shift;
        if shift + W as usize > 64 {
            if limb + 1 < 4 {
                words[limb + 1] |= chunk >> (64 - shift);
            } else {
                assert_eq!(chunk >> (64 - shift), 0);
            }
        }
    }
    assert_eq!(carry, 0);
    words
}

fn reduce(mut words: Words) -> Words {
    while let Some(difference) = subtract(words, MODULUS) {
        words = difference;
    }
    words
}

fn verify(original: Words, negative: bool, magnitude: Words, digits: Digits) {
    assert_eq!(recover(digits), magnitude);
    let modulo = reduce(magnitude);
    if negative && modulo != [0; 4] {
        assert_eq!(subtract(MODULUS, modulo).unwrap(), original);
    } else {
        assert_eq!(modulo, original);
    }
}

#[derive(Clone, Copy)]
struct Choice {
    magnitude: Words,
    negative: bool,
    digits: Digits,
}

impl Choice {
    fn score(self) -> (usize, bool, u16) {
        (
            nonzero(self.digits),
            self.negative,
            self.digits[WINDOWS - 1].unsigned_abs(),
        )
    }
}

fn choose(words: Words) -> [Choice; 5] {
    let base = Choice {
        magnitude: words,
        negative: false,
        digits: unsigned(words),
    };
    let mut options = [base; 5];
    let opposite = subtract(MODULUS, words).unwrap();
    let centered = subtract(opposite, words).is_none();
    if centered {
        options[1] = Choice {
            magnitude: opposite,
            negative: true,
            digits: unsigned(opposite),
        };
    }
    let mut representative = Some(words);
    for negative in [false, true] {
        if negative {
            representative = Some(opposite);
        }
        while let Some(magnitude) = representative {
            let raw = Choice {
                magnitude,
                negative,
                digits: unsigned(magnitude),
            };
            if raw.digits[WINDOWS - 1] <= 3 && raw.score() < options[2].score() {
                options[2] = raw;
            }
            if raw.score() < options[3].score() {
                options[3] = raw;
            }
            let signed = Choice {
                digits: minimum_signed(magnitude, 15),
                ..raw
            };
            if signed.score() < options[4].score() {
                options[4] = signed;
            }
            representative = add(magnitude, MODULUS);
        }
    }
    for option in options {
        verify(words, option.negative, option.magnitude, option.digits);
    }
    options
}

struct Counts {
    arrivals: [u64; WINDOWS],
    buckets: Vec<bool>,
    negative: u64,
    changed: u64,
}
impl Counts {
    fn new() -> Self {
        Self {
            arrivals: [0; WINDOWS],
            buckets: vec![false; WINDOWS * RADIX as usize],
            negative: 0,
            changed: 0,
        }
    }
    fn record(&mut self, base: Words, choice: Choice) {
        self.negative += u64::from(choice.negative);
        self.changed += u64::from(base != choice.magnitude || choice.negative);
        for (window, digit) in choice.digits.into_iter().enumerate() {
            let absolute = digit.unsigned_abs() as usize;
            if absolute != 0 {
                self.arrivals[window] += 1;
                self.buckets[window * RADIX as usize + absolute] = true;
            }
        }
    }
    fn print(&self, name: &str) {
        let occupied: Vec<_> = self
            .buckets
            .chunks_exact(RADIX as usize)
            .map(|window| window.iter().filter(|&&v| v).count() as u64)
            .collect();
        let arrivals = self.arrivals.iter().sum::<u64>();
        let live = occupied.iter().sum::<u64>();
        println!(
            "{name}\t{arrivals}\t{live}\t{}\t{}\t{}\t{}\t{}",
            arrivals - live,
            self.negative,
            self.changed,
            self.arrivals[WINDOWS - 1],
            occupied[WINDOWS - 1]
        );
    }
}

fn run(data: &[u8]) {
    let word = |s: &[u8]| u64::from_le_bytes(s.try_into().unwrap());
    assert_eq!(&data[..8], b"ZPH91500");
    assert_eq!(word(&data[8..16]), u64::from(W));
    assert_eq!(word(&data[16..24]), 2);
    let length = word(&data[24..32]) as usize;
    assert_eq!(data.len(), 32 + 33 * length);
    let mut counts: [Counts; 5] = std::array::from_fn(|_| Counts::new());
    let mut actual = 0;
    for row in data[32..].chunks_exact(33) {
        assert!(row[0] <= 1);
        if row[0] == 0 {
            continue;
        }
        actual += 1;
        let original = std::array::from_fn(|i| word(&row[1 + 8 * i..9 + 8 * i]));
        assert!(subtract(original, MODULUS).is_none());
        for (count, option) in counts.iter_mut().zip(choose(original)) {
            count.record(original, option);
        }
    }
    assert_eq!(actual, length);
    println!("input\t{actual}");
    for (mode, count) in [
        "ordinary",
        "centered",
        "unsigned254",
        "unsigned256",
        "signed256",
    ]
    .into_iter()
    .zip(counts)
    {
        count.print(mode);
    }
}

fn main() {
    let mut args = env::args_os().skip(1);
    let input = args.next().expect("H capture");
    let data = fs::read(input).unwrap();
    if args.next().is_some_and(|mode| mode == "--sample") {
        for (i, row) in data[32..].chunks_exact(33).enumerate().step_by(389) {
            assert_eq!(row[0], 1);
            let words = std::array::from_fn(|j| {
                u64::from_le_bytes(row[1 + 8 * j..9 + 8 * j].try_into().unwrap())
            });
            let counts = choose(words).map(|choice| nonzero(choice.digits));
            println!("{i} {} {}", counts[3], counts[4]);
        }
    } else {
        run(&data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representatives_reconstruct_at_carry_and_modulus_edges() {
        for x in [
            [0; 4],
            [1, 0, 0, 0],
            subtract(MODULUS, [1, 0, 0, 0]).unwrap(),
            [u64::MAX, u64::MAX, u64::MAX, 0x1fff_ffff_ffff_ffff],
        ] {
            let all = choose(x);
            assert!(nonzero(all[4].digits) <= nonzero(all[3].digits));
            assert!(nonzero(all[3].digits) <= nonzero(all[2].digits));
            assert!(nonzero(all[2].digits) <= nonzero(all[0].digits));
        }
    }

    #[test]
    fn dynamic_program_matches_complete_three_column_enumeration() {
        for lo in [0, 1, 2, 2047, 4094, 4095] {
            for middle in [0, 1, 2047, 4094, 4095] {
                for high in [0, 1, 3, 13, 4095] {
                    let value = lo + middle * 4096 + high * 4096 * 4096;
                    let expected = minimum_signed([value, 0, 0, 0], 15);
                    let mut best = 4;
                    for first in [0i64, 1] {
                        for second in [0i64, 1] {
                            for third in [0i64, 1] {
                                let bends = [
                                    lo as i64 - 4096 * first,
                                    middle as i64 + first - 4096 * second,
                                    high as i64 + second - 4096 * third,
                                    third,
                                ];
                                if bends.iter().all(|k| k.abs() < 4096) {
                                    best = best.min(bends.iter().filter(|&&k| k != 0).count());
                                }
                            }
                        }
                    }
                    assert_eq!(nonzero(expected), best);
                    assert_eq!(recover(expected), [value, 0, 0, 0]);
                }
            }
        }
    }
}
