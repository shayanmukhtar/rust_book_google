mod arrays;
mod references;
mod user_types;
mod pattern_matching;

use std::char;

use user_types::Point;

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

    let p1: Point = user_types::Point(3, 5);
    let p2: Point = user_types::Point(4, 3);
    let p3 = user_types::add_points(&p1, &p2);
    println!("New Point: ({}, {})", p3.0, p3.1);

    let task_cadence = user_types::get_task_cadence();
    println!("The task cadence is: {}ms", task_cadence.0);

    let player_move: user_types::PlayerMove = user_types::create_player_move();
    println!("Created player move was {player_move:?}");

    println!("The constant cadence was {}", user_types::CADENCE_MS);

    println!("Elevator exercise");
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        user_types::lobby_call_button_pressed(0, user_types::Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", user_types::car_arrived(0));
    println!("The car door opened: {:?}", user_types::car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        user_types::car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", user_types::car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", user_types::car_arrived(3));

    println!("Pattern Matching");
    pattern_matching::simple_match();

    pattern_matching::destructure_foo();

    pattern_matching::match_on_enum();
}
