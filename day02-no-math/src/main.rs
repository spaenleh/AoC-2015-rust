use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn compute_wrapping_area(&self) -> u32 {
        let triangles = [self.l * self.w, self.w * self.h, self.h * self.l];
        let area: u32 = triangles.iter().sum();
        let slack = triangles.iter().min().unwrap();
        (2 * area) + slack
    }

    fn smallest_face_perimeter(&self) -> u32 {
        let half_perimeters = [self.l + self.h, self.h + self.w, self.w + self.l];
        let smallest_perimeter = half_perimeters.iter().min().unwrap();
        smallest_perimeter * 2
    }

    fn volume(&self) -> u32 {
        self.w * self.h * self.l
    }

    fn ribbon_length(&self) -> u32 {
        self.volume() + self.smallest_face_perimeter()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "input_01.txt";
    let input = get_input(file_name)?;
    let dims = parse_input(&input);
    let total_area = compute_total_area(&dims);
    let ribbon_length = compute_total_ribbon_length(&dims);
    println!("area: {}, length: {}", total_area, ribbon_length);
    Ok(())
}

fn get_input(file_name: &str) -> io::Result<String> {
    fs::read_to_string(file_name)
}

fn parse_input(input: &str) -> Vec<Present> {
    let mut dims = Vec::new();
    for line in input.lines() {
        let mut parts = line
            .split('x')
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap());
        dims.push(Present {
            l: parts.next().unwrap(),
            w: parts.next().unwrap(),
            h: parts.next().unwrap(),
        });
    }
    dims
}

fn compute_total_area(dim_list: &Vec<Present>) -> u32 {
    dim_list.iter().map(|dim| dim.compute_wrapping_area()).sum()
}

fn compute_total_ribbon_length(dim_list: &Vec<Present>) -> u32 {
    dim_list.iter().map(|dim| dim.ribbon_length()).sum()
}

#[cfg(test)]
mod test_first {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "2x3x4";
        let dim = parse_input(input);
        assert_eq!(dim, vec![Present { l: 2, w: 3, h: 4 }])
    }

    #[test]
    fn test_area_1() {
        let dims = vec![Present { l: 2, w: 3, h: 4 }];
        let area = compute_total_area(&dims);
        assert_eq!(area, 58)
    }

    #[test]
    fn test_area_2() {
        let dims = vec![Present { l: 1, w: 1, h: 10 }];
        let area = compute_total_area(&dims);
        assert_eq!(area, 43)
    }

    #[test]
    fn test_volume() {
        let present = Present { l: 2, w: 3, h: 4 };
        let volume = present.volume();
        assert_eq!(volume, 24);
    }

    #[test]
    fn test_face_perimeter() {
        let present = Present { l: 2, w: 3, h: 4 };
        let perimeter = present.smallest_face_perimeter();
        assert_eq!(perimeter, 10);
    }

    #[test]
    fn test_ribbon_length() {
        let presents = [
            (Present { l: 2, w: 3, h: 4 }, 34),
            (Present { l: 1, w: 1, h: 10 }, 14),
        ];
        for (present, expected_ribbon_length) in presents {
            let perimeter = present.ribbon_length();
            assert_eq!(perimeter, expected_ribbon_length);
        }
    }
}
