use std::time;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidWidth { arg: &'static str },
    InvalidHeight { arg: &'static str },
    InvalidPercentage { arg: &'static str },
    TooManyArguments,
    NotEnoughArguments,
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn is_alive(self) -> bool {
        self == Cell::Alive
    }

    fn is_dead(self) -> bool {
        self == Cell::Dead
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[allow(dead_code)]
impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self {
        let mut b = Board {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        };
        let alive_count = ((percentage as f32 / 100.0) * b.cells.len() as f32) as usize;

        for _ in 0..alive_count {
			//give it 3 tries if the cell is already alive
            for _ in 0..3 {
                let index = ftkit::random_number(0..b.cells.len() as i32) as usize;
                if b.cells[index].is_dead() {
                    b.cells[index] = Cell::Alive;
                    break;
                }
            }
        }

        b
    }

    fn get_neighbour_count(&self, index: usize) -> u32 {
        let mut count = 0;
        let width = self.width as isize;
        let height = self.height as isize;
        let x = (index % self.width) as isize;
        let y = (index / self.width) as isize;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    let n_index = (ny * width + nx) as usize;
                    if self.cells[n_index].is_alive() {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn from_args() -> Result<Self, ParseError> {
		match ftkit::ARGS.len().cmp(&4) {
			Ordering::Less => return Err(ParseError::NotEnoughArguments),
			Ordering::Greater => return Err(ParseError::TooManyArguments),
            _ => (),
		}

		let width: usize;
		let height: usize;
		let perc: u32;

		if let Ok(x) = ftkit::ARGS[1].parse::<usize>() {
			width = x;
		} else {
            return Err(ParseError::InvalidWidth { arg: &ftkit::ARGS[1] });
		}
		if let Ok(x) = ftkit::ARGS[2].parse::<usize>() {
			height = x;
		} else {
			return Err(ParseError::InvalidHeight { arg: &ftkit::ARGS[2] });
		}
		if let Ok(x) = ftkit::ARGS[3].parse::<u32>() {
			perc = x;
		} else {
			return Err(ParseError::InvalidPercentage { arg: &ftkit::ARGS[3] });
		}
        Ok(Board::new(width, height, perc))
    }

    fn step(&mut self) {
        let mut cpy = self.cells.clone();
		for (i, item) in self.cells.iter().enumerate() {
            let n = self.get_neighbour_count(i);
            if item.is_alive() && !(2..=3).contains(&n) {
                cpy[i] = Cell::Dead;
            }
            if item.is_alive() && (n == 2 || n == 3) {
                cpy[i] = Cell::Alive;
            }
            if item.is_dead() && n == 3 {
                cpy[i] = Cell::Alive;
            }
        }
        self.cells = cpy;
    }

    fn print(&self, clear: bool) {
        if clear {
            print!("\x1B[2J\x1B[H");
        }
        for y in 0..self.height {
            for x in 0..self.width {
                match self.cells[x + y * self.width] {
                    Cell::Alive => print!("#"),
                    Cell::Dead => print!("."),
                }
            }
            println!();
        }
        print!("\x1B[{}A\x1B[0G", self.height);
    }
}

fn main() {
	let arg = Board::from_args();
	let mut t: Board = Board::new(0, 0, 0);

	if let Ok(x) = arg {
		t = x;
	} else if let Err(x) = arg {
		match x {
            ParseError::InvalidWidth { arg } => std::println!("error: '{}' is not a valid width", arg),
            ParseError::InvalidHeight { arg } => std::println!("error: '{}' is not a valid height", arg),
            ParseError::InvalidPercentage { arg } => std::println!("error: '{}' is not a valid percentage", arg),
            ParseError::TooManyArguments => std::println!("error: too many arguments"),
            ParseError::NotEnoughArguments => std::println!("error: not enough arguments")
		}
		return;
	}

    loop {
        t.print(true);
        t.step();
        std::thread::sleep(time::Duration::from_millis(50));
    }
}
