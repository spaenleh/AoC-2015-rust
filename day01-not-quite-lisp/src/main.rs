use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input_from_file("input_01.txt")?;
    let result = find_floor_simple(&input);
    let pos = find_basement_position(&input);
    println!("floor: {result}, pos: {pos}");
    Ok(())
}

fn read_input_from_file(file_name: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

fn find_floor_simple(input: &str) -> i32 {
    let mut dict = HashMap::from([('(', 0), (')', 0)]);

    for c in input.chars() {
        dict.entry(c).and_modify(|e| *e += 1);
    }
    dict[&'('] - dict[&')']
}

fn find_basement_position(input: &str) -> usize {
    let mut floor = 0;

    for (pos, c) in input.char_indices() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return pos + 1;
        }
    }
    input.len()
}

#[cfg(test)]
mod test_first {
    use super::*;

    #[test]
    fn test_floor_0() {
        let inputs = ["(())", "()()"];
        for input in inputs {
            let res = find_floor_simple(input);
            assert_eq!(res, 0);
        }
    }

    #[test]
    fn test_floor_3() {
        let inputs = ["(((", "))(((((", "(()(()("];
        for input in inputs {
            let res = find_floor_simple(input);
            assert_eq!(res, 3);
        }
    }

    #[test]
    fn test_floor_negative_1() {
        let inputs = ["())", "))("];
        for input in inputs {
            let res = find_floor_simple(input);
            assert_eq!(res, -1);
        }
    }

    #[test]
    fn test_floor_negative_3() {
        let inputs = [")))", ")())())"];
        for input in inputs {
            let res = find_floor_simple(input);
            assert_eq!(res, -3);
        }
    }
}

#[cfg(test)]
mod test_second {
    use super::*;

    #[test]
    fn test_enter_basement_at_pos_1() {
        let input = ")";
        let pos = find_basement_position(input);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_enter_basement_at_pos_5() {
        let input = "()())";
        let pos = find_basement_position(input);
        assert_eq!(pos, 5);
    }
}
