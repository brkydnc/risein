enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    let stdin = std::io::stdin();

    println!("Enter the first number: ");
    stdin.read_line(&mut input)?;
    let a: f64 = input.trim().parse()?;

    input.clear();

    println!("Enter the second number: ");
    stdin.read_line(&mut input)?;
    let b: f64 = input.trim().parse()?;

    input.clear();

    println!("Enter the operation: ");
    stdin.read_line(&mut input)?;

    let operation = match input.trim() {
        "+" => Operation::Add(a, b),
        "-" => Operation::Subtract(a, b),
        "*" => Operation::Multiply(a, b),
        "/" => Operation::Divide(a, b),
        _ => panic!("Unknown operation."),
    };

    let result = calculate(operation);
    println!("The result of your calculation is: {}", result);

    Ok(())
}
