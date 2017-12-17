extern crate advent2017;
use advent2017::file::Input;

use std::collections::HashMap;
use std::cmp;

fn main() {
    let instructions = Input::read("day08").into_lines().iter()
        .map(|it| Instruction::from_string(it))
        .collect::<Vec<Instruction>>();

    let mut registers = HashMap::new();

    let mut largest_during = 0;
    for it in &instructions {
        if it.should_execute(&registers) {
            it.execute(&mut registers);
        }

        let mut current = 0;
        if registers.contains_key(&it.register) {
            current = registers[&it.register];
        }

        largest_during = cmp::max(largest_during, current);
    }

    let mut largest = 0;
    for (_, it) in registers {
        largest = cmp::max(largest, it);
    }

    println!("largest value is {}", largest);
    println!("largest value during computation was {}", largest_during);
}

enum Operation {
    Inc(i32),
    Dec(i32),
}

enum ConditionOp {
    Eq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    NtEq,
}

struct Condition {
    register: String,
    value: i32,
    operation: ConditionOp,
}

impl Condition {
    fn from_string(from: &str) -> Condition {
        let parts = from.split(" ")
            .map(|it| it.trim())
            .collect::<Vec<&str>>();

        assert!(parts.len() == 3);

        let register = String::from(parts[0]);
        let value = parts[2].parse::<i32>()
            .expect("cond value was not a number");
        let operation = match parts[1] {
            "==" => ConditionOp::Eq,
            ">" => ConditionOp::Gt,
            "<" => ConditionOp::Lt,
            ">=" => ConditionOp::GtEq,
            "<=" => ConditionOp::LtEq,
            "!=" => ConditionOp::NtEq,
            _ => panic!("invalid cond code"),
        };

        Condition { register, value, operation }
    }
}

struct Instruction {
    register: String,
    operation: Operation,
    condition: Condition,
}

impl Instruction {
    fn from_string(from: &str) -> Instruction {
        let parts = from.split("if")
            .map(|it| it.trim())
            .collect::<Vec<&str>>();

        assert!(parts.len() == 2);

        let op_str = parts[0];
        let cond_str = parts[1];

        let op_parts = op_str.split(" ")
            .map(|it| it.trim())
            .collect::<Vec<&str>>();

        assert!(op_parts.len() == 3);

        let register = String::from(op_parts[0]);
        let value = op_parts[2].parse::<i32>()
            .expect("op value was not a number");
        let operation = match op_parts[1] {
            "dec" => Operation::Dec(value),
            "inc" => Operation::Inc(value),
            _ => panic!("invalid op code"),
        };

        let condition = Condition::from_string(cond_str);

        Instruction { register, operation, condition }
    }

    fn should_execute(&self, registers: &HashMap<String, i32>) -> bool {
        let mut reg_value = 0;
        if registers.contains_key(&self.condition.register) {
            reg_value = registers[&self.condition.register];
        }

        let value = self.condition.value;

        match self.condition.operation {
            ConditionOp::Eq => reg_value == value,
            ConditionOp::Gt => reg_value > value,
            ConditionOp::Lt => reg_value < value,
            ConditionOp::GtEq => reg_value >= value,
            ConditionOp::LtEq => reg_value <= value,
            ConditionOp::NtEq => reg_value != value,
        }
    }

    fn execute(&self, registers: &mut HashMap<String, i32>) {
        let mut result = 0;
        if registers.contains_key(&self.register) {
            result = registers[&self.register];
        }

        match self.operation {
            Operation::Inc(x) => result += x,
            Operation::Dec(x) => result -= x,
        }

        registers.insert(self.register.clone(), result);
    }
}
