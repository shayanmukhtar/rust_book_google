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
pub enum Directional {
    Left,
    Right,
}
// so here Direction has two variants, Direction::Left and Direction::Right
// but there isn't really a discriminant here, because Direction enums all 
// have the same "type"

#[derive(Debug)]
pub enum PlayerMove {
    Pass,
    Run(Directional),
    Teleport{ x: i32, y: i32},
}
// PlayerMove is more interesting - here we have three variants: Pass,
// Run(Direction::Whatever) and Teleport(x: something, y: something)
// Therefore there is a discriminant stored with this enum of minimal
// size to distinguish what type the variant is
// actually on my macbook air m3, it shows as having a sizeof 12 bytes
// with 4 byte alignment
pub fn create_player_move() -> PlayerMove {
    let player_move: PlayerMove = PlayerMove::Run(Directional::Left);
    player_move
}

// constants are inlined wherever they are used in rust - so 
// not necessarily part of const memory like they are in c.
// Does this waste a lot of space? 
pub const CADENCE_MS: u32 = 100;

// elevator exercise
#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
pub enum Event {
    Arrived(i32),           // the car has arrived on the given floor
    DoorOpen,               // the car doors have opened
    DoorClosed,             // the car doors have closed
    Call(i32, Direction),   // a directional button was pressed in an elevator lobby on the given floor
    FloorButton(i32),       // a floor button was pressed in the elevator car
}

/// A direction of travel.
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
pub fn car_arrived(floor: i32) -> Event {
    Event::Arrived(floor)
}

/// The car doors have opened.
pub fn car_door_opened() -> Event {
    Event::DoorOpen
}

/// The car doors have closed.
pub fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
pub fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::Call(floor, dir)
}

/// A floor button was pressed in the elevator car.
pub fn car_floor_button_pressed(floor: i32) -> Event {
    Event::FloorButton(floor)
}