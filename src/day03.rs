pub fn day03(input: &str) -> (usize, usize) {
    (
        parse_muls(&input, drop_until_part1)
            .iter()
            .map(|(a, b)| a * b)
            .sum::<u32>() as usize,
        parse_muls(&input, drop_until_part2)
            .iter()
            .map(|(a, b)| a * b)
            .sum::<u32>() as usize,
    )
}

type ParseState<'a> = (&'a str, bool);

fn parse_muls<F>(data: &str, drop_until_start: F) -> Vec<(u32, u32)>
where
    F: for<'a> Fn(ParseState<'a>, &str) -> ParseState<'a>,
{
    let mut results = Vec::new();
    let mut todo: ParseState = (data, true);
    while !todo.0.is_empty() {
        let after_drop = drop_until_start(todo, "mul(");
        match parse_next(after_drop) {
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
    let (r1, num1) = parse_int(data)?;
    let r2 = parse_prefix(r1, ',')?;
    let (r3, num2) = parse_int(r2)?;
    let r4 = parse_prefix(r3, ')')?;
    Ok((r4, if r4.1 { (num1, num2) } else { (0, 0) }))
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
