use std::collections::HashMap;

const DAY03_INPUT: &str = include_str!(r"input\day03.txt");

struct EngineSchematic {
    data: Vec<char>,
    width: usize,
    _height: usize,
}

struct Part {
    id: i32,
    start_x: i32,
    end_x: i32,
    y: i32,
}

struct Gear<'a> {
    _pos: (i32, i32),
    ratio: i32,
    adjacent_parts: Vec<&'a Part>,
}

impl EngineSchematic {
    fn new(input: &str) -> EngineSchematic {
        let mut width: usize = 0;
        let mut height: usize = 0;

        let data: Vec<char> = input
            .lines()
            .map(|l| {
                height += 1;
                let b: Vec<char> = l.trim().as_bytes().iter().map(|b| *b as char).collect();
                if width == 0 {
                    width = b.len()
                }
                b
            })
            .flatten()
            .collect();

        EngineSchematic {
            width,
            _height: height,
            data,
        }
    }

    fn get(&self, x: i32, y: i32) -> &char {
        self._get_with_index(self.to_index(x, y))
    }

    fn get_with_pos(&self, pos: &(i32, i32)) -> &char {
        self.get(pos.0, pos.1)
    }

    fn _get_with_index(&self, i: i32) -> &char {
        if i < 0 {
            return &'.';
        }
        self.data.get(i as usize).unwrap_or(&'.')
    }

    fn get_parts(&self) -> Vec<Part> {
        let mut result: Vec<Part> = Vec::new();
        let mut start_x = -1;

        for i in 0..self.data.len() as i32 {
            let (x, y) = self.from_index(i);
            let c = self.get(x, y);
            let n = self.get(x + 1, y);

            if is_digit(c) && start_x == -1 {
                start_x = self.from_index(i).0
            }

            if !is_digit(n) && start_x != -1 {
                if self.check_is_part(start_x, x, y) {
                    result.push(Part {
                        id: self.get_id(start_x, x, y),
                        start_x,
                        end_x: x,
                        y,
                    });
                }
                start_x = -1
            }
        }

        result
    }

    fn get_gears<'a>(&self, parts: &'a Vec<Part>) -> Vec<Gear<'a>> {
        let mut result: HashMap<(i32, i32), Gear> = HashMap::new();
        for part in parts {
            for pos in self
                .adjacent_positions(part.start_x, part.end_x, part.y)
                .filter(|pos| is_gear(self.get_with_pos(pos)))
            {
                if let Some(gear) = result.get_mut(&pos) {
                    gear.adjacent_parts.push(&part);
                } else {
                    result.insert(
                        pos,
                        Gear {
                            _pos: pos,
                            ratio: -1,
                            adjacent_parts: vec![&part],
                        },
                    );
                }
            }
        }
        for gear in result.values_mut() {
            if gear.adjacent_parts.len() == 2 {
                gear.ratio = gear.adjacent_parts[0].id * gear.adjacent_parts[1].id
            }
        }
        result
            .into_values()
            .filter(|g| g.adjacent_parts.len() == 2)
            .collect()
    }

    fn check_is_part(&self, start_x: i32, end_x: i32, y: i32) -> bool {
        self.adjacent_positions(start_x, end_x, y)
            .any(|pos| is_symbol(self.get(pos.0, pos.1)))
    }

    fn adjacent_positions(
        &self,
        start_x: i32,
        end_x: i32,
        y: i32,
    ) -> impl Iterator<Item = (i32, i32)> {
        let mut result = Vec::new();
        for x in start_x - 1..=end_x + 1 {
            result.push((x, y - 1));
            result.push((x, y + 1));
        }
        result.push((start_x - 1, y));
        result.push((end_x + 1, y));
        result.into_iter()
    }

    fn get_id(&self, start_x: i32, end_x: i32, y: i32) -> i32 {
        let i0 = self.to_index(start_x, y) as usize;
        let i1 = self.to_index(end_x, y) as usize;
        self.data[i0..=i1]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn to_index(&self, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self._height as i32 {
            -1
        } else {
            x + y * self.width as i32
        }
    }

    fn from_index(&self, i: i32) -> (i32, i32) {
        (i % self.width as i32, i / self.width as i32)
    }
}

fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

fn is_symbol(c: &char) -> bool {
    !is_digit(c) && *c != '.'
}

fn is_gear(c: &char) -> bool {
    *c == '*'
}

pub fn part1() -> i32 {
    EngineSchematic::new(DAY03_INPUT)
        .get_parts()
        .into_iter()
        .map(|p| p.id)
        .sum()
}

pub fn part2() -> i32 {
    let es = EngineSchematic::new(DAY03_INPUT);
    let parts = es.get_parts();
    es.get_gears(&parts).into_iter().map(|g| g.ratio).sum()
}
