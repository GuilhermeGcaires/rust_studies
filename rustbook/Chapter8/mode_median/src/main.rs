use std::io::BufRead;
use std::collections::HashMap;

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn main() {
    let mut input = read_vec::<usize>();
    input.sort();
    let len = input.len();
    if len % 2 == 0 {
        println!("{}" ,(input[len / 2] + input[(len / 2) - 1]) / 2);
    }
    println!("{}", input[input.len() / 2]);

    let mut map = HashMap::new();

    for val in input.iter() {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    let max_value = map.iter().max_by_key(|x| x.1).unwrap();

    println!("{:?}", max_value);
}
