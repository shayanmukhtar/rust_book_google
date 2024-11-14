mod arrays;
mod references;
mod user_types;

use std::char;

// simple mapping of the function from arrays.rs
// note we don't need crate here
use crate::arrays::declare_array;

fn main() {
    println!("Hello, world!");

    // call the function from arrays.rs
    declare_array();

    // this one isn't mapped using the `use` word, so we have to be explicit
    arrays::declare_tuple();

    arrays::iterate_over_array();

    arrays::destructure_tuples();

    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let transpose_matrix = arrays::transpose(matrix);
    println!("{:?}", transpose_matrix);

    println!("References");
    references::example();

    references::exclusing_reference();

    references::slices();

    references::strings();

    println!("Magnitude of a unit vector: {}", references::magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", references::magnitude(&v));
    references::normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", references::magnitude(&v));

    println!("User Types");
    let alice = user_types::Person {
        name: String::from("Alice"),
        age: 30,
    };

    user_types::describe(&alice);

    let mut bob = user_types::Person {
        name: String::from("Bob"),
        age: 25,
    };
    user_types::describe(&bob);

    // we can change the name and age of bob
    bob.name = String::from("Robert");
    user_types::describe(&bob);

    // we can create a new person, copying everything from a previous 
    // struct of the same type, save for what we want to make different
    // by adding ..otherStruct as the last element in the struct declaration

    let charlie = user_types::Person {
        name: String::from("Charlie"),
        ..bob
    };

    user_types::describe(&charlie);
}
