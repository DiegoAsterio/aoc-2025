fn parse_content_to_puzzle_input(content: String) -> Vec<Movement>{
    let lines = content.lines();
    lines.map(Movement::new).collect()
}

enum Movement {
    Left(i32),
    Right(i32),
}

impl Movement {
    fn new(line: &str) -> Movement {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let number_of_steps = chars.collect::<String>().parse::<i32>().unwrap();
        if direction == 'L' {
            Movement::Left(number_of_steps)
        } else if direction == 'R' {
            Movement::Right(number_of_steps)
        } else {
            panic!("The input line is incorrect it does not start by L or R. {}", line);
        }
    }

    fn act_on_dial(&self, position: i32) -> i32 {
        match self {
            Movement::Left(distance) => position - distance, // Should always be > 0
            Movement::Right(distance) => position + distance, // Should always be > 0
        }

    }
}

struct Safe {
    dial_position: i32
}

impl Safe {
    fn new() -> Safe{
        Safe {
            dial_position: 50
        }
    }

    fn is_pointing_at_zero(&self) -> bool {
        self.dial_position % 100 == 0
    }

    fn turns_to_the_dial(&self) -> i32 {
        if self.dial_position >= 0 {
            self.dial_position / 100
        }
        else {
            self.dial_position / 100 - 1
        }
    }
}

fn move_dial(safe: &Safe, movement: Movement) -> Safe {
    Safe {
        dial_position: movement.act_on_dial(safe.dial_position)
    }
}

fn zeroes_between_safe_positions(prev_safe: &Safe, curr_safe: &Safe) -> i32{
    let mut response = 0;
    if prev_safe.dial_position < curr_safe.dial_position{
        let mut next_hundred = (prev_safe.turns_to_the_dial() + 1) * 100;
        if next_hundred == prev_safe.dial_position {
            next_hundred += 100;
        }
        while next_hundred <= curr_safe.dial_position {
            response += 1;
            next_hundred += 100;
        }
    } else if curr_safe.dial_position < prev_safe.dial_position{
        let mut prev_hundred = prev_safe.turns_to_the_dial() * 100;
        if prev_hundred == prev_safe.dial_position {
            prev_hundred -= 100;
        }
        while prev_hundred >= curr_safe.dial_position {
            response += 1;
            prev_hundred -= 100;
        }
    }
    response
}

pub fn solve_fst(content: String) -> String{
    let mut safe = Safe::new();
    let movements = parse_content_to_puzzle_input(content);

    let mut response = 0;

    for movement in movements {
        safe = move_dial(&safe, movement);
        if safe.is_pointing_at_zero() {
            response += 1;
        }
    }

    response.to_string()
}

pub fn solve_snd(content: String) -> String{
    let mut curr_safe_position = Safe::new();
    let movements = parse_content_to_puzzle_input(content);

    let mut response = 0;

    for movement in movements {
        let prev_safe_position = curr_safe_position;
        curr_safe_position = move_dial(&prev_safe_position, movement);

        response += zeroes_between_safe_positions(
            &prev_safe_position,
            &curr_safe_position
        );
    }

    response.to_string()
}
