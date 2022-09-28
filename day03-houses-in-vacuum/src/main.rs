use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_to(&mut self, movement: &char) {
        let new_pos = match movement {
            '^' => Self {
                y: self.y + 1,
                x: self.x,
            },
            '>' => Self {
                x: self.x + 1,
                y: self.y,
            },
            '<' => Self {
                x: self.x - 1,
                y: self.y,
            },
            'v' => Self {
                y: self.y - 1,
                x: self.x,
            },
            _ => Self {
                x: self.x,
                y: self.y,
            },
        };
        *self = new_pos;
    }
}

struct DeliveryRoute {
    current_pos: Pos,
    pos_count: HashMap<Pos, u32>,
}

impl DeliveryRoute {
    fn new() -> Self {
        Self {
            current_pos: Pos::new(),
            pos_count: HashMap::new(),
        }
    }

    fn move_to(&mut self, movement: &char) {
        *self.pos_count.entry(self.current_pos.clone()).or_insert(0) += 1;
        self.current_pos.move_to(movement);
    }

    fn finish(&mut self) {
        *self.pos_count.entry(self.current_pos.clone()).or_insert(0) += 1;
    }
}

struct DeliveryRouteRoboSanta {
    current_pos: [Pos; 2],
    pos_count: HashMap<Pos, u32>,
}

impl DeliveryRouteRoboSanta {
    fn new() -> Self {
        Self {
            current_pos: [Pos::new(), Pos::new()],
            pos_count: HashMap::new(),
        }
    }

    fn move_to(&mut self, movement: &char, move_index: usize) {
        let index = move_index % 2;
        let position = self.current_pos[index];
        *self.pos_count.entry(position).or_insert(0) += 1;
        self.current_pos[index].move_to(movement);
    }

    fn finish(&mut self, move_index: usize) {
        let index = move_index % 2;
        let position = self.current_pos[index];
        *self.pos_count.entry(position).or_insert(0) += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let res = number_of_houses_with_presents(&input);
    let res2 = number_of_houses_with_robo(&input);
    println!("houses (santa): {}", res);
    println!("houses (santa+robo): {}", res2);
    Ok(())
}

fn number_of_houses_with_presents(input: &str) -> usize {
    let mut delivery = DeliveryRoute::new();
    for c in input.chars() {
        delivery.move_to(&c);
    }
    delivery.finish();
    delivery.pos_count.len()
}

fn number_of_houses_with_robo(input: &str) -> usize {
    let mut delivery = DeliveryRouteRoboSanta::new();
    for (idx, c) in input.char_indices() {
        delivery.move_to(&c, idx);
    }
    // add one because santa also delivers where he is at the end
    delivery.finish(input.len());
    delivery.pos_count.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_move() {
        let input = ">";
        let res = number_of_houses_with_presents(input);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_complex_move() {
        let input = "^>v<";
        let res = number_of_houses_with_presents(input);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_up_down_move() {
        let input = "^v^v^v^v^v";
        let res = number_of_houses_with_presents(input);
        assert_eq!(res, 2);
    }
}
