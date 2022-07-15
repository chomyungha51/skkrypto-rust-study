use std::io;

struct MathOp {
    op1: u32,
    op2: u32,
    op: String,
}

impl MathOp {
    fn add(&self) -> u32 {
        self.op1 + self.op2
    }

    fn sub(&self) -> u32 {
        self.op1 - self.op2
    }

    fn mul(&self) -> u32 {
        self.op1 * self.op2
    }

    fn div(&self) -> u32 {
        self.op1 / self.op2
    }

    fn rem(&self) -> u32 {
        self.op1 % self.op2
    }
}

const PLUS:&str = "+";
const MIN:&str = "-";
const MULT:&str = "*";
const DIV:&str = "/";
const REM:&str = "%";

fn main() {
    

    println!("Enter the first number");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a: u32 = a.trim().parse().expect("The first number is not a number");

    println!("Enter the operation");
    let mut op = String::new();
    io::stdin().read_line(&mut op);
    let op:&str = op.trim();

    println!("Enter the second number");
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let b: u32 = b.trim().parse().expect("The second number is not a number");

    let target = MathOp { op1: a, op2: b, op: String::from(op) };

    let mut result = 0;

    match op {
        PLUS => {
            result = target.add();
        },
        MIN => {
            if (target.op1 > target.op2) {
                result = target.sub();
                
            }else {
                println!("The second operand is too big");
                return;
            }
        },
        MULT => {
            result = target.mul();
        },
        DIV => {
            result = target.div();
        },
        REM => {
            result = target.rem();
        },
        _ => {
            println!("Not supported operation");
            return;
        },
    }
    println!("Result: {}", result);
}
