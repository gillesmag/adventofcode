#[derive(Debug)]
enum Operand {
    W,
    X,
    Y,
    Z,
    Number(i64),
}


#[derive(Debug)]
enum Instruction {
    INP(Operand),
    ADD(Operand, Operand),
    MUL(Operand, Operand),
    DIV(Operand, Operand),
    MOD(Operand, Operand),
    EQL(Operand, Operand),
}

struct CPU {
    register_w: i64,
    register_x: i64,
    register_y: i64,
    register_z: i64,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register_w: 0,
            register_x: 0,
            register_y: 0,
            register_z: 0,
        }
    }

    fn reset(&mut self) {
        self.register_w = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.register_z = 0;
    }

    fn run(&mut self, instructions: &Vec<Instruction>, data: Vec<i64>) {
        let mut data_pointer = 0;
        for instruction in instructions.iter() {
            match instruction {
                Instruction::INP(reg) => {
                    match reg {
                        Operand::W => {
                            self.register_w = data[data_pointer];
                        },
                        Operand::X => {
                            self.register_x = data[data_pointer];
                        },
                        Operand::Y => {
                            self.register_w = data[data_pointer];
                        },
                        Operand::Z => {
                            self.register_z = data[data_pointer];
                        },
                        _ => (),
                    }
                    data_pointer += 1;
                },
                Instruction::ADD(left, right) => {
                    let left_value = match left {
                        Operand::W => Some(self.register_w),
                        Operand::X => Some(self.register_x),
                        Operand::Y => Some(self.register_w),
                        Operand::Z => Some(self.register_z),
                        _ => None,
                    }.unwrap();

                    let right_value = match right {
                        Operand::W => self.register_w,
                        Operand::X => self.register_x,
                        Operand::Y => self.register_w,
                        Operand::Z => self.register_z,
                        Operand::Number(value) => *value,
                    };

                    match left {
                        Operand::W => self.register_w = left_value + right_value,
                        Operand::X => self.register_x = left_value + right_value,
                        Operand::Y => self.register_w = left_value + right_value,
                        Operand::Z => self.register_z = left_value + right_value,
                        _ => todo!(),
                    };
                },
                Instruction::MUL(left, right) => {
                    let left_value = match left {
                        Operand::W => Some(self.register_w),
                        Operand::X => Some(self.register_x),
                        Operand::Y => Some(self.register_w),
                        Operand::Z => Some(self.register_z),
                        _ => None,
                    }.unwrap();

                    let right_value = match right {
                        Operand::W => self.register_w,
                        Operand::X => self.register_x,
                        Operand::Y => self.register_w,
                        Operand::Z => self.register_z,
                        Operand::Number(value) => *value,
                    };

                    match left {
                        Operand::W => self.register_w = left_value * right_value,
                        Operand::X => self.register_x = left_value * right_value,
                        Operand::Y => self.register_w = left_value * right_value,
                        Operand::Z => self.register_z = left_value * right_value,
                        _ => todo!(),
                    };
                },
                Instruction::DIV(left, right) => {
                    let left_value = match left {
                        Operand::W => Some(self.register_w),
                        Operand::X => Some(self.register_x),
                        Operand::Y => Some(self.register_w),
                        Operand::Z => Some(self.register_z),
                        _ => None,
                    }.unwrap();

                    let right_value = match right {
                        Operand::W => self.register_w,
                        Operand::X => self.register_x,
                        Operand::Y => self.register_w,
                        Operand::Z => self.register_z,
                        Operand::Number(value) => *value,
                    };

                    match left {
                        Operand::W => self.register_w = left_value / right_value,
                        Operand::X => self.register_x = left_value / right_value,
                        Operand::Y => self.register_w = left_value / right_value,
                        Operand::Z => self.register_z = left_value / right_value,
                        _ => todo!(),
                    };
                },
                Instruction::MOD(left, right) => {
                    let left_value = match left {
                        Operand::W => Some(self.register_w),
                        Operand::X => Some(self.register_x),
                        Operand::Y => Some(self.register_w),
                        Operand::Z => Some(self.register_z),
                        _ => None,
                    }.unwrap();

                    let right_value = match right {
                        Operand::W => self.register_w,
                        Operand::X => self.register_x,
                        Operand::Y => self.register_w,
                        Operand::Z => self.register_z,
                        Operand::Number(value) => *value,
                    };

                    match left {
                        Operand::W => self.register_w = left_value % right_value,
                        Operand::X => self.register_x = left_value % right_value,
                        Operand::Y => self.register_w = left_value % right_value,
                        Operand::Z => self.register_z = left_value % right_value,
                        _ => todo!(),
                    };
                },
                Instruction::EQL(left, right) => {
                    let left_value = match left {
                        Operand::W => Some(self.register_w),
                        Operand::X => Some(self.register_x),
                        Operand::Y => Some(self.register_w),
                        Operand::Z => Some(self.register_z),
                        _ => None,
                    }.unwrap();

                    let right_value = match right {
                        Operand::W => self.register_w,
                        Operand::X => self.register_x,
                        Operand::Y => self.register_w,
                        Operand::Z => self.register_z,
                        Operand::Number(value) => *value,
                    };

                    match left {
                        Operand::W => self.register_w = (left_value == right_value) as i64,
                        Operand::X => self.register_x = (left_value == right_value) as i64,
                        Operand::Y => self.register_w = (left_value == right_value) as i64,
                        Operand::Z => self.register_z = (left_value == right_value) as i64,
                        _ => todo!(),
                    };

                },
            }
        }
    }
}

fn parse_register(register: &str) -> Operand {
    if let Ok(num) = register.parse::<i64>() {
        return Operand::Number(num);
    }

    match register {
        "w" => Operand::W,
        "x" => Operand::X,
        "y" => Operand::Y,
        "z" => Operand::Z,
        &_ => todo!(),
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let components: Vec<&str> = line.split(" ").collect();
        match components[0] {
            "inp" => Instruction::INP(parse_register(components[1])),
            "add" => Instruction::ADD(parse_register(components[1]), parse_register(components[2])),
            "mul" => Instruction::MUL(parse_register(components[1]), parse_register(components[2])),
            "div" => Instruction::DIV(parse_register(components[1]), parse_register(components[2])),
            "mod" => Instruction::MOD(parse_register(components[1]), parse_register(components[2])),
            "eql" => Instruction::EQL(parse_register(components[1]), parse_register(components[2])),
            &_ => todo!(),
        }
    }).collect::<Vec<Instruction>>()
}

fn number_to_vec(number: i64) -> Vec<i64> {
    let mut data: Vec<i64> = vec![];
    let mut copy_number = number;
    for _ in 1..=14 {
        data.insert(0, copy_number % 10);
        copy_number = copy_number / 10;
    }
    data
}

pub fn day24(input: &str) -> (String, String) {
    let instructions = parse(&input);
    let mut cpu = CPU::new();
    let mut number: i64 = 100000000000000;
    let mut counter = 0;
    while number > 10000000000000 {
        counter += 1;
        number -= 1;
        let data = number_to_vec(number);
        if data.iter().any(|&v| v == 0) {
            continue;
        }
        cpu.run(&instructions, data);
        // println!("{}: {} {} {} {}", number, cpu.register_w, cpu.register_x, cpu.register_y, cpu.register_z);
        if counter % 1000000 == 0 {
            println!("{}", number);
        }
        if cpu.register_z == 0 {
            println!("z = {}", cpu.register_z);
            break;
        }
        cpu.reset();
    }
    (
        cpu.register_z.to_string(),
        "".to_string()
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    // use aoc::read_file;

    #[test]
    fn test_example_small() {
        let input = "inp x
mul x -1";
        let instructions = parse(&input);
        let mut cpu = CPU::new();
        let mut data: Vec<i64> = vec![24];
        cpu.run(instructions, data);
        assert_eq!(cpu.register_x, -24);
    }

    #[test]
    fn test_example_medium() {
        let input = "inp z
inp x
mul z 3
eql z x";
        let instructions = parse(&input);
        let mut cpu = CPU::new();
        let data: Vec<i64> = vec![2, 6];
        cpu.run(instructions, data);
        assert_eq!(cpu.register_z, 1);
    }
}
