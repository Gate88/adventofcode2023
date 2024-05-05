const DAY03_INPUT: &str = include_str!(r"..\input\day03.txt");

struct EngineSchematic {
    data: Vec<char>,
    width: usize,
    _height: usize,
}

struct Part {
    id: i32,
    _start_x: i32,
    _end_x: i32,
    _y: i32,
}

impl EngineSchematic {
    fn new(input: &str) -> EngineSchematic {
        let mut width: usize = 0;
        let mut height: usize = 0;

        let data: Vec<char> = input.lines().map(|l| {
            height += 1;
            let b: Vec<char> = l.trim().as_bytes().iter().map(|b| *b as char).collect();
            if width == 0 {
                width = b.len()
            }
            b
        }).flatten().collect();

        EngineSchematic {
            width,
            _height: height,
            data,
        }
    }

    fn get(&self, x: i32, y: i32) -> &char {
        self._get_with_index(self.to_index(x, y))
    }

    fn _get_with_index(&self, i: i32) -> &char {
        if i < 0 { return &'.' };
        self.data.get(i as usize).unwrap_or(&'.')
    }

    fn to_part_iter(&self) -> impl Iterator<Item = Part> {
        let mut result: Vec<Part> = Vec::new();
        let mut start_x = -1;

        for i in 0..self.data.len() as i32{
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
                        _start_x: start_x,
                        _end_x: i,
                        _y: y
                    });
                }
                start_x = -1
            }
        }

        result.into_iter()
    }

    fn check_is_part(&self, start_x: i32, end_x: i32, y: i32) -> bool {
        for x in start_x - 1..=end_x + 1 {
            // check above and below
            if is_symbol(self.get(x, y-1)) || is_symbol(self.get(x, y+1)) {
                return true
            }
        }
        //check same line
        if is_symbol(self.get(start_x-1, y)) || is_symbol(self.get(end_x+1, y)) {
            return true
        }
        return false
    }

    fn get_id(&self, start_x: i32, end_x: i32, y: i32) -> i32 {
        let i0 = self.to_index(start_x, y) as usize;
        let i1 = self.to_index(end_x, y) as usize;
        self.data[i0..=i1].iter().collect::<String>().parse().unwrap()
    }

    fn to_index(&self, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self._height as i32{
            -1
        } else {
            x + y * self.width as i32
        }
    }

    fn from_index(&self, i: i32) -> (i32, i32) {
        (
            i % self.width as i32,
            i / self.width as i32,
        )
    }
}

fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

fn is_symbol(c: &char) -> bool {
    !is_digit(c) && *c != '.'
}

pub fn part1() {
    let p1: i32 = EngineSchematic::new(DAY03_INPUT).to_part_iter().map(|p| p.id).sum();
    println!("{p1}");
}