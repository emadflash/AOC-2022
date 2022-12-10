#![allow(non_snake_case)]

use std::error::Error;

const INPUT: &str = include_str!("../input.txt");

struct Cpu {
    reg_x: i32,
    cycles: i32,
    signal_strength: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg_x: 1,
            cycles: 1,
            signal_strength: 0,
        }
    }

    #[inline]
    fn is_next_40th_cycle(&self) -> bool {
        self.cycles == 20
            || self.cycles == 60
            || self.cycles == 100
            || self.cycles == 140
            || self.cycles == 180
            || self.cycles == 220
    }

    fn tick(&mut self, asm: &str) {
        if self.is_next_40th_cycle() {
            self.signal_strength += self.reg_x * self.cycles;
        }

        match asm.chars().next() {
            Some('n') => self.cycles += 1,
            Some('a') => {
                let (_, operand) = asm.split_once(" ").unwrap();
                let operand = operand.parse::<i32>().unwrap();
                self.cycles += 1;

                if self.is_next_40th_cycle() {
                    self.signal_strength += self.reg_x * self.cycles;
                }

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

fn part2(input: &str) -> u32 {
    todo!();
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}
