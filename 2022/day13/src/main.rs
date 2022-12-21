use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
enum Token {
    ListStart,
    ListEnd,
    Integer(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum List {
    List(Vec<List>),
    Integer(u32),
}

impl List {
    fn parse<'a, I>(tokens: &mut I) -> Option<List>
    where
        I: Iterator<Item = Token>,
    {
        match tokens.next()? {
            Token::ListStart => {
                let mut items = vec![];
                while let Some(item) = List::parse(tokens) {
                    items.push(item);
                }
                Some(List::List(items))
            }
            Token::ListEnd => None,
            Token::Integer(value) => Some(List::Integer(value)),
        }
    }
}

// List needs a custom implementation of PartialOrd because integer and list
// values do not compare traditionally.
impl PartialOrd for List {
    fn partial_cmp(&self, other: &List) -> Option<Ordering> {
        match (self, other) {
            (List::Integer(s), List::Integer(o)) => s.partial_cmp(o),
            (List::Integer(s), List::List(o)) => vec![List::Integer(*s)].partial_cmp(o),
            (List::List(s), List::Integer(o)) => s.partial_cmp(&vec![List::Integer(*o)]),
            (List::List(s), List::List(o)) => s.partial_cmp(o),
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &List) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn tokenize(s: &str) -> Vec<Token> {
    s.split_inclusive(&['[', ']', ','])
        .flat_map(|segment| {
            let mut chars = segment.chars();
            let tail = match chars.next_back().unwrap() {
                '[' => Some(Token::ListStart),
                ']' => Some(Token::ListEnd),
                _ => None,
            };
            let head = str::parse::<u32>(chars.as_str())
                .ok()
                .map(|v| Token::Integer(v));
            [head, tail]
        })
        .flatten()
        .collect()
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .flat_map(|line| List::parse(&mut tokenize(&line).into_iter()))
        .collect();
    let pairs = input.iter().step_by(2).zip(input.iter().skip(1).step_by(2));

    let part1: u32 = (1u32..)
        .zip(pairs)
        .filter(|(_, (l, r))| l <= r)
        .map(|(i, _)| i)
        .sum();
    println!("Part 1: {}", part1);

    let packet1 = List::List(vec![List::List(vec![List::Integer(2)])]);
    let packet2 = List::List(vec![List::List(vec![List::Integer(6)])]);
    let mut sorted = input.clone();
    sorted.push(packet1.clone());
    sorted.push(packet2.clone());
    sorted.sort();

    let index1 = sorted.binary_search(&packet1).unwrap() + 1;
    let index2 = sorted.binary_search(&packet2).unwrap() + 1;
    println!("Part 2: {}", index1 * index2);
}
