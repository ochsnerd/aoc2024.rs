use std::fs;
use std::path::PathBuf;

pub fn day03(mut input_path: PathBuf) {
    input_path.push("03.txt");
    let content = fs::read_to_string(input_path).unwrap();

    println!(
        "{:?}",
        parse_muls(&content).iter().map(|(a, b)| a * b).sum::<u32>()
    );
}

type ParseState<'a> = (&'a str, bool);

fn parse_muls(data: &str) -> Vec<(u32, u32)> {
    let mut results = Vec::new();
    let mut todo: ParseState = (data, true);
    while !todo.0.is_empty() {
        match parse_next(todo) {
            Ok((remaining, pair)) => {
                results.push(pair);
                todo = remaining;
            }
            Err(remaining) => {
                todo = remaining;
            }
        };
    }
    results
}

fn parse_next(data: ParseState) -> Result<(ParseState, (u32, u32)), ParseState> {
    let r1 = drop_until_part2(data, "mul(");
    let (r2, num1) = parse_int(r1)?;
    let r3 = parse_prefix(r2, ',')?;
    let (r4, num2) = parse_int(r3)?;
    let r5 = parse_prefix(r4, ')')?;
    Ok((r5, if r5.1 { (num1, num2) } else { (0, 0) }))
}

#[allow(dead_code)]
fn drop_until_part1<'a>(data: ParseState<'a>, prefix: &str) -> ParseState<'a> {
    match data.0.split_once(prefix) {
        Some((_, rest)) => (rest, true),
        None => ("", true),
    }
}

fn drop_until_part2<'a>(data: ParseState<'a>, prefix: &str) -> ParseState<'a> {
    match data.0.split_once(prefix) {
        Some((dropped, rest)) => match (dropped.rfind("do()"), dropped.rfind("don't()")) {
            (Some(last_do), Some(last_dont)) => (rest, last_dont < last_do),
            (Some(_), None) => (rest, true),
            (None, Some(_)) => (rest, false),
            (None, None) => (rest, data.1),
        },
        None => ("", data.1),
    }
}

fn parse_prefix(data: ParseState, prefix: char) -> Result<ParseState, ParseState> {
    if data.0.starts_with(prefix) {
        Ok((&data.0[1..], data.1))
    } else {
        Err(data)
    }
}

fn parse_int(data: ParseState) -> Result<(ParseState, u32), ParseState> {
    let digits_end = data.0.chars().take_while(|c| c.is_ascii_digit()).count();
    match data.0[..digits_end].parse::<u32>() {
        Ok(n) => Ok(((&data.0[digits_end..], data.1), n)),
        Err(_) => Err(data),
    }
}
