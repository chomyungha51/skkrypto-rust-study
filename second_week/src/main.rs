use std::io;

struct MathOp {
    op1: u32,
    op2: u32,
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

fn main() {
    println!("Enter the first number");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a: u32 = a.trim().parse().expect("The first number is not a number");

    println!("Enter the second number");
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let b: u32 = b.trim().parse().expect("The second number is not a number");

    let target = MathOp { op1: a, op2: b };
    let res = target.add();
    println!("Add Result: {res}");
    if (target.op1 > target.op2) {
        let res = target.sub();
        println!("Sub Result: {res}");
    }

    let res = target.mul();
    println!("Mul Result: {res}");

    let res = target.div();
    println!("Div Result: {res}");

    let res = target.rem();
    println!("Rem Result: {res}");
}
