use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::time::Instant;

/// Rust version of Python's itertools.product().
/// /// It returns the cartesian product of the input iterables, and it is
/// /// semantically equivalent to `repeat` nested for loops.
/// ///
/// /// # Arguments
/// ///
/// /// * `it` - An iterator over a cloneable data structure
/// /// * `repeat` - Number of repetitions of the given iterator
pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    std::iter::repeat(it).take(repeat).multi_cartesian_product()
}

struct ALU {
    variables: HashMap<char, i64>,
}

impl ALU {
    fn new(vars: Vec<char>) -> ALU {
        let mut variables: HashMap<char, i64> = HashMap::new();
        for var in vars {
            variables.insert(var, 0);
        }
        ALU { variables }
    }

    fn run(&mut self, inputs: Vec<i64>, instructions: &Vec<Instruction>) {
        let mut inputs = inputs.iter();
        for instruction in instructions {
            match instruction {
                Instruction::Inp(var) => {
                    if let Operand::Variable(v) = var {
                        self.variables
                            .entry(*v)
                            .and_modify(|val| *val = *inputs.next().unwrap());
                    } else {
                        panic!("Unsupported INP operand");
                    }
                }
                Instruction::Add(left, right) => {
                    if let Operand::Variable(l) = left {
                        // get value of right
                        let right_val = match &right {
                            Operand::Variable(r) => *self.variables.get(r).unwrap(),
                            Operand::Literal(r) => *r,
                        };
                        self.variables.entry(*l).and_modify(|val| *val += right_val);
                    } else {
                        panic!("Left operand of ADD cannot be literal");
                    }
                }
                Instruction::Mul(left, right) => {
                    if let Operand::Variable(l) = left {
                        // get value of right
                        let right_val = match &right {
                            Operand::Variable(r) => *self.variables.get(r).unwrap(),
                            Operand::Literal(r) => *r,
                        };
                        self.variables.entry(*l).and_modify(|val| {
                            //println!("{} {}", *val, right_val);
                            *val *= right_val
                        });
                    } else {
                        panic!("Left operand of ADD cannot be literal");
                    }
                }
                Instruction::Div(left, right) => {
                    if let Operand::Variable(l) = left {
                        // get value of right
                        let right_val = match &right {
                            Operand::Variable(r) => *self.variables.get(r).unwrap(),
                            Operand::Literal(r) => *r,
                        };
                        self.variables.entry(*l).and_modify(|val| *val /= right_val);
                    } else {
                        panic!("Left operand of ADD cannot be literal");
                    }
                }
                Instruction::Mod(left, right) => {
                    if let Operand::Variable(l) = left {
                        // get value of right
                        let right_val = match &right {
                            Operand::Variable(r) => *self.variables.get(r).unwrap(),
                            Operand::Literal(r) => *r,
                        };
                        self.variables.entry(*l).and_modify(|val| *val %= right_val);
                    } else {
                        panic!("Left operand of ADD cannot be literal");
                    }
                }
                Instruction::Eql(left, right) => {
                    if let Operand::Variable(l) = left {
                        // get value of right
                        let right_val = match &right {
                            Operand::Variable(r) => *self.variables.get(r).unwrap(),
                            Operand::Literal(r) => *r,
                        };
                        self.variables
                            .entry(*l)
                            .and_modify(|val| *val = (*val == right_val) as i64);
                    } else {
                        panic!("Left operand of ADD cannot be literal");
                    }
                }
            }
            //println!("{:?}", instruction);
        }
    }

    fn get_var(&self, var: char) -> i64 {
        *self.variables.get(&var).unwrap()
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Literal(i64),
    Variable(char),
}

// Implement parse on str?
impl Operand {
    fn parse(value: &str) -> Operand {
        if let Ok(value) = value.parse::<i64>() {
            Operand::Literal(value)
        } else {
            Operand::Variable(value.chars().collect::<Vec<char>>()[0])
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

fn program(inputs: &Vec<i64>) -> i64 {
    let mut inputs = inputs.iter();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut w = 0;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 12;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 10;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += 0;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 3;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 15;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 10;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 13;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 6;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -12;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 13;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -15;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -15;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 1;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -4;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 7;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 6;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -5;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 9;
    y *= x;
    z += y;

    w = *inputs.next().unwrap();
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -12;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 9;
    y *= x;
    z += y;

    return z;
}

fn main() {
    //let filename = "test.txt";
    //let filename = "test2.txt";
    //let filename = "test3.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let lines = file.lines().collect::<Vec<&str>>();
    //println!("{:?}", lines);

    let instructions: Vec<Instruction> = lines
        .into_iter()
        .map(|instr| {
            let tokens = instr.split(" ").collect::<Vec<&str>>();
            match tokens[0] {
                "inp" => Instruction::Inp(Operand::parse(tokens[1])),
                "add" => Instruction::Add(Operand::parse(tokens[1]), Operand::parse(tokens[2])),
                "mul" => Instruction::Mul(Operand::parse(tokens[1]), Operand::parse(tokens[2])),
                "div" => Instruction::Div(Operand::parse(tokens[1]), Operand::parse(tokens[2])),
                "mod" => Instruction::Mod(Operand::parse(tokens[1]), Operand::parse(tokens[2])),
                "eql" => Instruction::Eql(Operand::parse(tokens[1]), Operand::parse(tokens[2])),
                _ => {
                    panic!("Unknown instruction");
                }
            }
        })
        .collect();
    //println!("{:?}", instructions);
    //let inputs = vec![13];
    //let mut alu = ALU::new(vec!['w', 'x', 'y', 'z']);
    //alu.run(inputs, &instructions);
    //println!("{:?}", alu.get_var('x'));
    //println!("{:?}", alu.get_var('y'));
    //println!("{:?}", alu.get_var('z'));
    //println!("{:?}", alu.get_var('w'));

    let serial_numbers = product_repeat((1..=9).rev(), 14);
    for sn in serial_numbers {
        //let mut alu = ALU::new(vec!['w', 'x', 'y', 'z']);
        //alu.run(sn.clone(), &instr1);
        //if alu.get_var('z') == 0 {
        if program(&sn) == 0 {
            println!("{:?}", sn);
            break;
        }
    }
}
