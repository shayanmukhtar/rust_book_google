// match works like the switch statement in c
// it goes down the list, and the first match wins
pub fn simple_match() {
    let input = 'x';

    match input {
        'q'                      => println!("Quit"),
        'a' | 's' | 'd' | 'w'    => println!("Move"),
        '0'..='9'                => println!("Number"), // the = in the range means include the number 9 too
        key if key.is_lowercase() => println!("Lowercase {key}"), // here we used a variable in the match arm
        _                        => println!("Unknown"),// catch all wildcard - a match statement must be exhaustive
    }

    // the type of the match expression above is ()
    // the internal variables that were created int the match arms are
    // of the same type as what the match expression is testing against I think.
    // e.g. in this case input is of type char, and so is key

}

// we can also match on structs by destructuring them
struct Foo {
    x: (i32, i32),
    y: i32,
}

pub fn destructure_foo() {
    let foo = Foo { x: (2, 2), y: 1 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }

    // from here we can see how we can take a struct, destructure it, and 
    // match on it completely. We can also ignore fields by simply not 
    // including them in the match arm
}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

pub fn match_on_enum() {
    let n = 101;
    match divide_in_two(n) {
        Result::Ok(half)        => {println!("Divided in two equals: {half}")},
        Result::Err(error)   => {println!("Something went wrong: {error}")},
    }
}

// figuring out `if let` way of doing things
fn foo(number: i32) {
    println!("Foo called with {number}");
}

pub fn exploring_if_let() {
    // let's say we're using the Result type again from above,
    // and we want to call a function if the result is Ok, otherwise
    // not
    let our_result: Result = Result::Ok(42);

    // now we can do the whole match thing and call foo
    // if the result is ok
    match our_result {
        Result::Err(_error)      => {},
        Result::Ok(number)         => {foo(number)}
    }

    // but there is a cleaner way - in cases like this, where
    // a single match arm produces the results we want, we can
    // combine an if and a pattern match together in one statement
    // like this:
    let our_result: Result = Result::Ok(42);
    
    if let Result::Ok(number) = our_result {
        foo(number);
    }
    // basically this was a pattern match - if `our_result` matches
    // the pattern `Result::Ok(number)` then bind number to a variable,
    // and enter the if condition
}