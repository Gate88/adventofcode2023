use std::collections::HashMap;

const _DAY08_SIMPLE_INPUT: &str = include_str!(r"..\input\day08_simple.txt");
const DAY08_INPUT: &str = include_str!(r"..\input\day08.txt");

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn is_valid_name(name: &str) -> bool {
    name.len() == 3
        && name
            .chars()
            .all(|c| c >= 'A' && c <= 'Z' || c >= '0' && c <= '9')
}

impl<'a> Node<'a> {
    fn new(line: &'a str) -> Self {
        let mut s = line.split(" = ");

        let name = s.next().expect("could not find name");
        let pair = s.next().expect("could not find pair");

        let left = pair
            .get(1..=3)
            .unwrap_or_else(|| panic!("could not find left: {}", pair));
        assert!(is_valid_name(left), "left is not a valid name: {}", left);

        let right = pair
            .get(6..=8)
            .unwrap_or_else(|| panic!("could not find right: {}", pair));
        assert!(is_valid_name(right), "right is not valid name: {}", right);

        Node { name, left, right }
    }
}

fn read_input(input: &str) -> (HashMap<&str, Node>, Vec<Direction>) {
    use Direction::*;

    let mut lines = input.lines();
    let directions = lines
        .next()
        .expect("could not find directions")
        .chars()
        .map(|c| match c {
            'L' => Left,
            'R' => Right,
            x => panic!("unknown direction: {}", x),
        })
        .collect();
    lines
        .next()
        .expect("could not find empty line after directions");

    let nodes = lines.fold(HashMap::new(), |mut hash_map, line| {
        let node = Node::new(line);
        hash_map
            .entry(node.name)
            .and_modify(|_| panic!("entry already existed: {}", node.name))
            .or_insert(node);
        return hash_map;
    });

    (nodes, directions)
}

fn get_node<'a>(graph: &'a HashMap<&str, Node>, name: &str) -> &'a Node<'a> {
    graph
        .get(name)
        .unwrap_or_else(|| panic!("node missing: {}", name))
}

fn calculate_steps(graph: &HashMap<&str, Node>, directions: Vec<Direction>) -> usize {
    let mut current_name = "AAA";
    let mut steps = 0;
    for d in directions.iter().cycle() {
        let n = get_node(graph, current_name);

        current_name = follow_direction(n, d);
        steps += 1;

        if current_name == "ZZZ" {
            break;
        }
    }

    return steps;
}

fn follow_direction<'a>(node: &Node<'a>, direction: &Direction) -> &'a str {
    use Direction::*;
    match direction {
        Left => node.left,
        Right => node.right,
    }
}

#[derive(Debug)]
struct WalkState<'a> {
    current_name: &'a str,
    loop_length: Option<usize>,
}

impl<'a> WalkState<'a> {
    fn new(current_name: &'a str) -> Self {
        WalkState {
            current_name,
            loop_length: None,
        }
    }
}

fn calculate_ghost_steps(graph: &HashMap<&str, Node>, directions: Vec<Direction>) -> u128 {
    let mut walk_states: Vec<_> = graph
        .keys()
        .filter_map(|&name| (name.chars().nth(2) == Some('A')).then_some(WalkState::new(name)))
        .collect();

    let mut walk_states_done = Vec::new();

    let mut steps = 0;
    for d in directions.iter().cycle() {
        steps += 1;

        for walk in walk_states.iter_mut() {
            let n = get_node(graph, walk.current_name);
            walk.current_name = follow_direction(n, d);
            if let Some(loop_length) = walk.loop_length.as_mut() {
                *loop_length += 1
            }
            if walk.current_name.chars().nth(2) == Some('Z') {
                walk.loop_length = Some(steps);
            }
        }
        let mut partitions = walk_states
            .into_iter()
            .partition(|w| w.loop_length.is_none());
        walk_states = partitions.0;
        walk_states_done.append(&mut partitions.1);
        if walk_states.is_empty() {
            break;
        }
    }

    walk_states_done
        .windows(2)
        .filter_map(|w| TryInto::<&[_; 2]>::try_into(w).ok())
        .fold(0, |total, [a, b]| {
            if total == 0 {
                lcm(
                    a.loop_length.unwrap() as u128,
                    b.loop_length.unwrap() as u128,
                )
            } else {
                lcm(total, b.loop_length.unwrap() as u128)
            }
        })
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    if a == b {
        return a;
    }

    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    a
}

fn lcm(a: u128, b: u128) -> u128 {
    a * (b / gcd(a, b))
}

pub fn part1() -> usize {
    let (graph, directions) = read_input(DAY08_INPUT);
    calculate_steps(&graph, directions)
}

pub fn part2() -> u128 {
    let (graph, directions) = read_input(DAY08_INPUT);
    calculate_ghost_steps(&graph, directions)
}
