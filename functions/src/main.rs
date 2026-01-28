fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
    expression_returns_value();
    let x = function_returns_value();
    println!("The value returned from function_returns_value is: {x}");
    println!("Quiz answer is: ");
    quiz();
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

// Errors: missing type for parameter `x` in function `f`
// fn missing_type_for_parameter(x){
//     println!("{x}");
// }

// Errors: let statements do not return values
// fn statements_do_not_return_values() {
//     let x = (let y = 6);
// }

fn expression_returns_value() {
    let y = {
        let x = 3;
        x + 1 // No semicolon means this is an expression that returns a value
    };

    println!("The value of y is: {y}");
}

fn function_returns_value() -> i32 {
    5 // No semicolon means this is an expression that returns a value
}

// Errors: functions with return type must return a value
// The semi colon turns the expression into a statement, which does not return a value
// fn function_with_no_return_value(x: i32) -> i32 {
//     x + 1;
// }


fn f(x: i32) -> i32 { x + 1 }
fn quiz() {
    println!("{}", f({
        let y = 1;
        y + 1
    }));
}