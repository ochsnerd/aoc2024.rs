pub fn day09(input: &str) -> (usize, usize) {
    let numbers = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut disk = make_explicit_disk_map(&numbers);
    compact_p1(&mut disk);
    let p1 = checksum(&disk);

    let mut disk = make_explicit_disk_map(&numbers);
    compact_p2(&mut disk);
    let p2 = checksum(&disk);
    (p1, p2)
}

type ExplicitDiskMap = Vec<Option<usize>>;

// 2333133121414131402 -> 00...111...2...333.44.5555.6666.777.888899
// where empty = . = None
fn make_explicit_disk_map(input: &[u32]) -> ExplicitDiskMap {
    let mut blocks = Vec::new();
    for (i, chunk) in input.chunks(2).enumerate() {
        match chunk {
            [file, free] => {
                blocks.extend(vec![Some(i); *file as usize]);
                blocks.extend(vec![None; *free as usize]);
            }
            [file] => {
                blocks.extend(vec![Some(i); *file as usize]);
            }
            _ => panic!(),
        };
    }
    blocks
}

fn compact_p1(disk_map: &mut ExplicitDiskMap) {
    // C++ iterators would be really cool here
    let (mut front, mut back) = (0, disk_map.len() - 1);
    while front != back {
        match (disk_map[front], disk_map[back]) {
            (Some(_), _) => {
                front += 1;
            }
            (_, None) => {
                back -= 1;
            }
            (None, Some(_)) => {
                disk_map.swap(front, back);
            }
        }
    }
}

fn compact_p2(disk_map: &mut ExplicitDiskMap) {
    // sadly O(n^2), see failed O(n logn) attempt below
    for i in (1..disk_map.len()).rev() {
        if disk_map[i].is_none() || disk_map[i] == disk_map[i - 1] {
            continue;
        }

        let file_size = disk_map[i..]
            .iter()
            .take_while(|&c| *c == disk_map[i])
            .count();

        let end_of_first_spot = disk_map
            .iter()
            // ok these scan semantics are surprising (read: ass)
            .scan(0, |run, c| match c {
                Some(_) => {
                    *run = 0;
                    Some(0)
                }
                None => {
                    *run += 1;
                    Some(*run)
                }
            })
            .enumerate()
            .skip_while(|(_, run)| *run < file_size)
            .skip_while(|(j, _)| *j > i)
            .map(|(j, _)| j)
            .next();

        if let Some(last_empty) = end_of_first_spot {
            for k in 0..file_size {
                disk_map.swap(last_empty - k, i + k);
            }
        }
    }
}

fn checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, f)| f.map(|f| i * f))
        .sum()
}

// fn compact_p2(disk_map: &mut ExplicitDiskMap) {
//     // mapping size to starts
//     let mut empties = BTreeMap::new();
//     let mut run = 0;
//     for (i, block) in disk_map.iter().enumerate() {
//         match block {
//             Some(_) => {
//                 if run > 0 {
//                     empties
//                         .entry(run)
//                         .or_insert(BinaryHeap::new())
//                         .push(Reverse(i - run)); // Reverse the ordering
//                     run = 0;
//                 }
//             }
//             None => run += 1,
//         }
//     }

//     let mut file_id = 0;
//     let mut file_size = 0;
//     for i in (0..disk_map.len()).rev() {
//         match disk_map[i] {
//             Some(f) => {
//                 if f == file_id {
//                     file_size += 1;
//                 } else {
//                     if file_size > 0 {
//                         // move the file to the first spot large enough to hold it
//                         let maybe_next = empties
//                             .range_mut(file_size..)
//                             .map_while(|(size, starts)| starts.pop().map(|start| (size, start)))
//                             .next();

//                         if let Some((size, Reverse(start))) = maybe_next {
//                             // move file
//                             for j in 0..file_size {
//                                 disk_map.swap(start + j, i + j);
//                             }
//                             // update empties
//                             empties
//                                 .entry(size - file_size)
//                                 .or_insert(BinaryHeap::new())
//                                 .push(Reverse(start + file_size));
//                         }
//                     }
//                     file_id = f;
//                     file_size = 1;
//                 }
//             }
//             None => {}
//         }
//     }
// }
