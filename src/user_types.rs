// structs are the same as c/c++ - they group together values
// into a block of memory. No typedef needed when defining a struct

// the declaration of a struct is as follows:
// struct TypeName {
//    field1: Type1,
//    field2: Type2,
//    ...
// } 

// remember since we're in another module, every single field, and the 
// struct itself must be marked as public if we want to use it in other modules
pub struct Person{
    pub name: String,
    pub age: u8,
}

///
/// This function takes a reference to a Person struct and prints out the name and age
pub fn describe(person: &Person) {
    println!("This person is {} years old and their name is {}", person.age, person.name);
}

// we can have tuple structs if the field names themselves are not important
pub struct Point (pub i32, pub i32); // why does this have the semicolon but not the other
//one? Tuples surround the declaration with brackets and not braces

pub fn add_points(p1: &Point, p2: &Point) -> Point {
    let new_point: Point = Point(p1.0 + p2.0, p1.1 + p2.1);
    new_point
}

// you new use something called newtypes to surround a primitive with a named type
// to make the intention clearer for the variable
pub struct TaskCadenceMs (pub u32);

pub fn get_task_cadence() -> TaskCadenceMs {
    TaskCadenceMs(10)
}

// enums
// there are two factors that go into an enum, the variant, and the discriminant
// the variant is simply how many items in an enum there are, and this is obvious
// on inspection
// the discriminant, on the other hand, is something the compiler stores with
// the instantation of a type of the enum (and is therefore part of the size 
// of the enum itself) that stores what type a particular variant is. This is 
// because the variants of an enum can all have a different "type"
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}
// so here Direction has two variants, Direction::Left and Direction::Right
// but there isn't really a discriminant here, because Direction enums all 
// have the same "type"

#[derive(Debug)]
pub enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport{ x: i32, y: i32},
}
// PlayerMove is more interesting - here we have three variants: Pass,
// Run(Direction::Whatever) and Teleport(x: something, y: something)
// Therefore there is a discriminant stored with this enum of minimal
// size to distinguish what type the variant is
// actually on my macbook air m3, it shows as having a sizeof 12 bytes
// with 4 byte alignment
pub fn create_player_move() -> PlayerMove {
    let player_move: PlayerMove = PlayerMove::Run(Direction::Left);
    player_move
}

// constants are inlined wherever they are used in rust - so 
// not necessarily part of const memory like they are in c.
// Does this waste a lot of space? 
pub const CADENCE_MS: u32 = 100;