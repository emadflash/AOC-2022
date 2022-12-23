#![allow(non_snake_case)]

use std::error::Error;

const INPUT: &str = include_str!("../input.txt");

struct Crt {
    crt: [[char; 40]; 6],
    col: usize,
    row: usize,
}

impl Crt {
    fn new() -> Self {
        Self {
            crt: [['.'; 40]; 6],
            col: 0,
            row: 0,
        }
    }

    fn draw(&mut self, reg_x: &i32) {
        if self.col == 40 {
            self.col = 0;
            self.row += 1;
        }

        let col = self.col as i32;
        let (lo, hi) = (reg_x - 1, reg_x + 1);

        if col >= lo && col <= hi {
            self.crt[self.row][self.col] = '█';
        }
        self.col += 1;
    }
}

struct Cpu {
    reg_x: i32,
    cycles: i32,
    signal_strength: i32,
    crt: Crt,
    next_epoch_cycle: u16,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg_x: 1,
            cycles: 1,
            signal_strength: 0,
            crt: Crt::new(),
            next_epoch_cycle: 20,
        }
    }

    fn update_signal_strength(&mut self) {
        if self.cycles == self.next_epoch_cycle as i32 {
            self.signal_strength += self.reg_x * self.cycles;
            self.next_epoch_cycle += 40;
        }
    }

    fn tick(&mut self, asm: &str) {
        self.update_signal_strength();
        self.crt.draw(&self.reg_x);

        match asm.chars().next() {
            Some('n') => self.cycles += 1,
            Some('a') => {
                let (_, operand) = asm.split_once(" ").unwrap();
                let operand = operand.parse::<i32>().unwrap();
                self.cycles += 1;

                self.update_signal_strength();
                self.crt.draw(&self.reg_x);

                self.reg_x += operand;
                self.cycles += 1;
            }

            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut cpu = Cpu::new();
    for asm in input.split("\n").filter(|l| !l.is_empty()) {
        cpu.tick(asm);
    }

    cpu.signal_strength
}

fn part2(input: &str) {
    let mut cpu = Cpu::new();
    for asm in input.split("\n").filter(|l| !l.is_empty()) {
        cpu.tick(asm);
    }

    for l in cpu.crt.crt {
        println!("{:?}", l.iter().collect::<String>());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: ");
    part2(INPUT);

    assert_eq!(part1(INPUT), 15880);
    Ok(())
}

// Part 2 answer, cool af :)
//
// "███..█.....██..████.█..█..██..████..██.."
// "█..█.█....█..█.█....█.█..█..█....█.█..█."
// "█..█.█....█....███..██...█..█...█..█...."
// "███..█....█.██.█....█.█..████..█...█.██."
// "█....█....█..█.█....█.█..█..█.█....█..█."
// "█....████..███.█....█..█.█..█.████..███."
