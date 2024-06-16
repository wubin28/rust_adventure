fn main() {
    // The return value of the if expression
    let condition = true;
    let value = if condition { 10 } else { 20 };
    // value 的值是 10
    println!("The return value of the if expression is: {}", value);

    // The return value of the match expression
    let number = 2;
    let description = match number {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    // description 的值是 "two"
    println!(
        "The return value of the match expression is: {}",
        description
    );

    // The return value of the loop expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    // result 的值是 20
    println!("The return value of the loop expression is: {}", result);

    // The return value of the while expression
    let mut count = 0;
    let while_result = while count < 5 {
        count += 1;
        println!("count: {}", count);
        // count * 2 // error[E0308]: mismatched types. expected `()`, found integer
        // break count * 2; // error[E0571]: `break` with value from a `while` loop. can only break with a value inside `loop` or breakable block
    };
    // while_result 的值是 ()
    println!(
        "The return value of the while expression is: {:?}",
        while_result
    );

    // The return value of the for expression
    let numbers = [1, 2, 3, 4, 5];
    let for_result = for number in numbers.iter() {
        println!("number: {}", number);
        // number * 2 // error[E0308]: mismatched types。 expected `()`, found integer
        // break number * 2; // error[E0571]: `break` with value from a `for` loop. can only break with a value inside `loop` or breakable block
    };
    // for_result 的值是 ()
    println!(
        "The return value of the for expression is: {:?}",
        for_result
    );
}
// The output is as follows:
// The return value of the if expression is: 10
// The return value of the match expression is: two
// The return value of the loop expression is: 20
// count: 1
// count: 2
// count: 3
// count: 4
// count: 5
// The return value of the while expression is: ()
// number: 1
// number: 2
// number: 3
// number: 4
// number: 5
// The return value of the for expression is: ()
