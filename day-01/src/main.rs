use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_01() {
        assert_eq!(part1("()()".to_owned()).unwrap(), 0);
        assert_eq!(part1("(())".to_owned()).unwrap(), 0);
        assert_eq!(part1("(())".to_owned()), part1("()()".to_owned()));
    }
    #[test]
    fn p1_02() {
        assert_eq!(part1("(((".to_owned()).unwrap(), 3);
        assert_eq!(part1("(()(()(".to_owned()).unwrap(), 3);
        assert_eq!(part1("(((".to_owned()), part1("(()(()(".to_owned()));
    }
    #[test]
    fn p1_03() {
        assert_eq!(part1("))(((((".to_owned()).unwrap(), 3);
    }
    #[test]
    fn p1_04() {
        assert_eq!(part1("())".to_owned()).unwrap(), -1);
        assert_eq!(part1("))(".to_owned()).unwrap(), -1);
        assert_eq!(part1("())".to_owned()), part1("))(".to_owned()));
    }
    #[test]
    fn p1_05() {
        assert_eq!(part1(")))".to_owned()).unwrap(), -3);
        assert_eq!(part1(")())())".to_owned()).unwrap(), -3);
        assert_eq!(part1(")))".to_owned()), part1(")())())".to_owned()));
    }
}

fn running_sum(input_string: String) -> Vec<i32> {
    input_string
        .into_bytes()
        .into_iter()
        .map(|i| -2 * (i as i32) + 81)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

fn part1(input_string: String) -> Option<i32> {
    running_sum(input_string).last().cloned()
}

fn main() {
    let contents_part1 = fs::read_to_string("inputs/part1.txt").expect("Unable to read file");
    part1(contents_part1.trim().to_owned());
    fs::write(
        "outputs/part1.txt",
        part1(contents_part1.trim().to_owned()).unwrap().to_string(),
    )
    .expect("Unable to write to file");
}
