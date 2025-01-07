use std::fs;
use std::path::PathBuf;

pub fn day04(mut input_path: PathBuf) {
    input_path.push("04.txt");
    let input = fs::read_to_string(input_path).unwrap();

    let data = parse_input(&input);

    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}

// parse input into vector-of-vectors for easier transpose etc.
// use u8 instead of char because we don't need utf-8
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect()
}

fn part1(data: &Vec<Vec<u8>>) -> usize {
    let horizontal: usize = data.iter().map(|l| count_occurences_fw_bw("XMAS", l)).sum();
    let vertical: usize = transpose(&data)
        .iter()
        .map(|l| count_occurences_fw_bw("XMAS", l))
        .sum();
    let diagonal: usize = diagonals(&data)
        .iter()
        .map(|l| count_occurences_fw_bw("XMAS", l))
        .sum();
    let anti_diagonal: usize = diagonals(&transpose(&data))
        .iter()
        .map(|l| count_occurences_fw_bw("XMAS", l))
        .sum();
    horizontal + vertical + diagonal + anti_diagonal
}

fn count_occurences_fw_bw(word: &str, text: &Vec<u8>) -> usize {
    let fw = word.as_bytes();
    let bw: Vec<u8> = fw.iter().copied().rev().collect();
    text.windows(word.len())
        .filter(|&w| w == fw || w == bw)
        .count()
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let rows = v.len();
    let cols = v[0].len();
    (0..cols)
        .map(|col| (0..rows).map(|row| v[row][col]).collect())
        .collect()
}

fn diagonals<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!v.is_empty());
    let n = v[0].len();
    let m = v.len();
    // require rectangle
    assert!(v.iter().filter(|l| l.len() == n).count() == m);

    let mut diags = vec![Vec::<T>::new(); m + n - 1];
    for d in 0..m {
        let mut i = 0;
        let mut j = d;
        while j < m && i < n {
            diags[d].push(v[j][i]);
            i += 1;
            j += 1;
        }
    }

    for d in 1..m {
        let mut i = d;
        let mut j = 0;
        while j < m && i < n {
            diags[d + m - 1].push(v[j][i]);
            i += 1;
            j += 1;
        }
    }
    diags
}

fn tile<T>(grid: &Vec<Vec<T>>) -> Vec<Vec<Vec<T>>>
where
    T: Copy,
{
    let rows = grid.len();
    let cols = grid[0].len();
    let mut subtiles = Vec::new();

    // interesting: with saturating_sub we can 'easily' use usize as slice-bounds
    for i in 0..=rows.saturating_sub(3) {
        for j in 0..=cols.saturating_sub(3) {
            let mut subtile = Vec::new();
            for row in i..i + 3 {
                let mut subtile_row = Vec::new();
                for col in j..j + 3 {
                    subtile_row.push(grid[row][col]);
                }
                subtile.push(subtile_row);
            }
            subtiles.push(subtile);
        }
    }

    subtiles
}

fn is_xmas(t: &Vec<Vec<u8>>) -> bool {
    t[0][0] == b'M' && t[0][2] == b'M' && t[1][1] == b'A' && t[2][0] == b'S' && t[2][2] == b'S'
}

// mirror vertically
fn mirror(t: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    t.iter()
        .map(|row| row.iter().rev().copied().collect::<Vec<u8>>())
        .collect()
}

// Idea:
// M M   M M
//  A  =  A
// S S	 S S

// M M             M S
//  A  = transpose  A
// S S	           M S

// M M                     S M
//  A  = transpose (mirror  A  )
// S S	                   S M

// M M                                S S
//  A  = transpose (mirror (transpose  A  )  (this is actually just horizontal mirror)
// S S	                              M M

// It would actually be better transform a XMAS (at compile time) and check against those
fn is_any_xmas(t: &Vec<Vec<u8>>) -> bool {
    is_xmas(t)
        || is_xmas(&transpose(t))
        || is_xmas(&transpose(&mirror(t)))
        || is_xmas(&transpose(&mirror(&transpose(t))))
}

fn part2(data: &Vec<Vec<u8>>) -> usize {
    tile(data).iter().filter(|&t| is_any_xmas(t)).count()
}
