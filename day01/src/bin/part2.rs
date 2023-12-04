fn main() {
    let input = include_str!("./input2.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let sum = input.lines().map(resolve_line).sum();

    return sum;
}

fn resolve_line(line: &str) -> u32 {
    let mut i = 0;

    let resolved_line = std::iter::from_fn(move || {
        let line_slice = &line[i..];
        i += 1;
        if line_slice.starts_with("one") {
            Some('1')
        } else if line_slice.starts_with("two") {
            Some('2')
        } else if line_slice.starts_with("three") {
            Some('3')
        } else if line_slice.starts_with("four") {
            Some('4')
        } else if line_slice.starts_with("five") {
            Some('5')
        } else if line_slice.starts_with("six") {
            Some('6')
        } else if line_slice.starts_with("seven") {
            Some('7')
        } else if line_slice.starts_with("eight") {
            Some('8')
        } else if line_slice.starts_with("nine") {
            Some('9')
        } else {
            line_slice.chars().next()
        }
    });

    let mut it = resolved_line.filter_map(|c| c.to_digit(10));

    let first = it.next().expect("should be a number");
    let last = it.last();

     let res = match last {
        Some(num) => format!("{first}{num}").parse::<u32>(),
        None => format!("{first}{first}").parse(),
    }
    .expect("should be a number");

    dbg!(res)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_sample() {
        let result = solve(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}