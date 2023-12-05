use std::collections::HashSet;

use glam::IVec2;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

#[derive(Debug)]
struct Number {
    value: u32,
    start: u16,
    len: u16,
    included: bool,
}

fn parse_line(input: &str, line_no: usize) -> (Vec<Number>, Vec<IVec2>) {
    let mut num_vec = Vec::<Number>::new();
    let mut symbol_vec = Vec::<IVec2>::new();

    let mut cur_num = String::new();

    for (index, c) in input.char_indices() {
        if c.is_digit(10) {
            cur_num.push(c);
        } else if !cur_num.is_empty() {
            num_vec.push(Number {
                value: cur_num.parse().unwrap(),
                start: (index - cur_num.len()) as u16,
                len: cur_num.len() as u16,
                included: false,
            });
            cur_num.clear();
        }

        if c != '.' && !c.is_digit(10) {
            symbol_vec.push(IVec2 {
                x: index as i32,
                y: line_no as i32,
            });
        }
    }

    (num_vec, symbol_vec)
}

fn compute_check_positions(sym_vec: &Vec<IVec2>, bounds: IVec2) -> HashSet<IVec2> {
    let check_pos = vec![
        IVec2 { x: -1, y: -1 },
        IVec2 { x: -1, y: 0 },
        IVec2 { x: -1, y: 1 },
        IVec2 { x: 0, y: -1 },
        IVec2 { x: 0, y: 1 },
        IVec2 { x: 1, y: -1 },
        IVec2 { x: 1, y: 0 },
        IVec2 { x: 1, y: 1 },
    ];

    let mut check_set = HashSet::<IVec2>::new();

    for sym in sym_vec {
        for pos in &check_pos {
            if (sym.x + pos.x >= 0)
                && (sym.x + pos.x < bounds.x)
                && (sym.y + pos.y >= 0)
                && (sym.y + pos.y < bounds.y)
            {
                check_set.insert(IVec2 {
                    x: sym.x + pos.x,
                    y: sym.y + pos.y,
                });
            }
        }
    }

    check_set
}

fn find_parts(num_vec: &mut Vec<Vec<Number>>, check_pos: &HashSet<IVec2>) -> u32 {
    for pos in check_pos {
        let nums = &mut num_vec[pos.y as usize];
        for number in nums {
            if pos.x >= number.start.into() && pos.x < (number.start + number.len).into() {
                number.included = true;
            }
        }
    }

    num_vec
        .into_iter()
        .map(|nums| {
            nums.into_iter()
                .map(|num| if num.included { num.value } else { 0 })
                .sum::<u32>()
        })
        .sum()
}

fn solve(input: &str) -> u32 {
    let mut num_vec = Vec::<Vec<Number>>::new();
    let mut sym_vec = Vec::<IVec2>::new();
    for (index, line) in input.lines().enumerate() {
        let (nums, syms) = parse_line(line, index);
        num_vec.push(nums);
        sym_vec.extend(syms);
    }

    let check_pos = compute_check_positions(
        &sym_vec,
        IVec2 {
            x: input.lines().next().unwrap().len() as i32,
            y: input.lines().count() as i32,
        },
    );

    find_parts(&mut num_vec, &check_pos)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_sample() {
        let result = solve(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(result, 4361);
    }
}
