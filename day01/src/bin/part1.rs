fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {

    let sum = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));

            let first = it.next().expect("should be a number");
            let last = it.last();

            match last {
                Some(num) => format!("{first}{num}").parse::<u32>(),
                None => format!("{first}{first}").parse(),
            }
            .expect("should be a number")
        })
        .sum();

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_sample() {
        let result = solve(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
