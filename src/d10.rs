use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    let mut cpu = Cpu::default();
    let mut crt = Crt::default();

    for line in input.lines() {
        let instruction = Instruction::from_str(line)?;
        cpu.do_instruction(&instruction);
        crt.draw(&cpu);
    }

    let mut result = format!("Day 10/01: {}\n", cpu.signal_strength);
    result += &format!("Day 10/02:\n{}", crt.pixels_to_string());
    Ok(result)
}

#[derive(Debug)]
struct Cpu {
    n_cycles_start: usize,
    n_cycles_stop: usize,
    next_measurement_at: usize,
    register_x_start: i64,
    register_x_stop: i64,
    signal_strength: i64,
}

impl Cpu {
    fn do_instruction(&mut self, instruction: &Instruction) {
        let (delta_x, delta_cycle) = match instruction {
            Instruction::Noop => (0, 1),
            Instruction::AddX(a) => (*a, 2),
        };

        self.register_x_start = self.register_x_stop;
        self.n_cycles_start = self.n_cycles_stop;

        self.register_x_stop = self.register_x_start + delta_x;
        self.n_cycles_stop = self.n_cycles_start + delta_cycle;

        if self.n_cycles_start < self.next_measurement_at
            && self.n_cycles_stop >= self.next_measurement_at
        {
            self.signal_strength += self.register_x_start * (self.next_measurement_at as i64);
            self.next_measurement_at += 40;
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            n_cycles_start: 0,
            n_cycles_stop: 0,
            next_measurement_at: 20, // during cycle
            register_x_start: 1,
            register_x_stop: 1,
            signal_strength: 0,
        }
    }
}

const CRT_HEIGHT: usize = 6;
const CRT_WIDTH: usize = 40;

struct Crt {
    pixels: [bool; CRT_HEIGHT * CRT_WIDTH],
}

impl Crt {
    fn draw(&mut self, cpu: &Cpu) {
        for c in cpu.n_cycles_start..cpu.n_cycles_stop {
            let c = c % (CRT_HEIGHT * CRT_WIDTH);
            let c_mod = (c % CRT_WIDTH) as i64;
            let lower = cpu.register_x_start - 1;
            let upper = cpu.register_x_start + 1;

            if lower <= c_mod && c_mod <= upper {
                self.pixels[c] = true;
            }
        }
    }

    fn pixels_to_string(&self) -> String {
        let mut j = 0;
        let mut result = String::new();
        for p in self.pixels.iter() {
            if *p {
                result += "■";
            } else {
                result += " ";
            }
            j += 1;
            if j == CRT_WIDTH {
                result.push('\n');
                j = 0;
            }
        }
        result
    }
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            pixels: [false; CRT_HEIGHT * CRT_WIDTH],
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn from_str(s: &str) -> Result<Self, StringError> {
        let s = s.trim();
        let mut splitted = s.split_whitespace();
        let first = splitted.next().ok_or("d10: Invalid input.")?;

        if first == "noop" {
            return Ok(Self::Noop);
        }

        if first != "addx" {
            return Err("d10: Invalid input. First column must either be addx or noop".into());
        }

        let second = splitted
            .next()
            .ok_or("d10: Invalid input. Need second column.")?;
        let amount = second
            .parse::<i64>()
            .map_err(|_| "d10: Invalid input. Second column not a number")?;

        Ok(Self::AddX(amount))
    }
}
