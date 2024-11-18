// for generic functions (similar to templates in other languages), the syntax is
// fn name<T>(args) -> ReturnType
// the <T> at the beginning indicates this will be a generic function parameterized for
// type T, and then T can be used in the function as the args or the ReturnType
// e.g.

pub fn pick<T>(even_odd: i8, even: T, odd: T) -> T {
    if even_odd % 2 == 0 {
        even
    }
    else {
        odd
    }
}

// this should be noted that this is a zero cost abstraction, and the exact
// same machine code would be generated as if you had two functions, one 
// that took strings for even and odd, and another that took integers

// now let's look at making types generic - the syntax here is 
// struct Name<T> - best to remember that <T> comes after the name, be it 
// function or type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Now we can choose what type of impl we want for this generic type - we can
// give it a concrete impl for a specific type, or we can also make the impl
// generic too, for example
impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    fn set_x(&mut self, x: T) {
        self.x = x;
    }
}

// now the above is a generic implementation for a generic type, if we wanted to,
// we could make an impl just for a specific type, like this:
impl Point<u32> {
    fn coords_u32(&self) -> (&u32, &u32) {
        (&self.x, &self.y)
    }

    fn set_coords(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

pub fn test_generic_types() {
    // we can either specifically specify point as type f64, or we can
    // let the compiler infer based on the fields
    let floating_point_coords: Point<f64> = Point{x: 21.0, y: -12.0};

    // here we let the compiler infer, and it used Point<i32>
    let mut signed_point_coords = Point{x: 100, y: 20};

    let mut unsigned_point_coords = Point{x: 25_u32, y: 41_u32};

    // notice now that set_coords, which is only specificied for type of u32
    // is available for unsigned_point_coords, not the rest
    unsigned_point_coords.set_coords(11_u32, 15_u32);

    // the other method doesn't appear
    signed_point_coords.set_x(33);
}

// of course, like functions and types, traits to can be made generic. A good example 
// is From, which is an interface that must be defined for types to be converted to
// other types
// the syntax for generic trait definitions are
// trait Name<T> { }
// e.g. from the standard docs the definition for `From` is
// pub trait From<T>: Sized {}
#[derive(Debug)]
struct Foo(String);
impl From<u32> for Foo {
    fn from(value: u32) -> Self {
        Foo(format!("Converting from u32 to Foo: {}", value))
    }
}

impl From<bool> for Foo {
    fn from(value: bool) -> Self {
        Foo(format!("Converting from bool to Foo: {}", value))
    }
}

pub fn test_generic_impl() {
    let foo: Foo = Foo::from(true);
    println!("Created foo with value: {}", foo.0);
}

// trait bounds - sometimes when we make a generic function, we need
// to make sure that the type T that goes into a function implements
// a certain trait - we can specify which trait that generic must 
// implement with this syntax:
// fn name<T: Trait>(args) -> ReturnType
// what this does is ensure T, when made concrete, implements Trait (e.g clone)
// e.g.
fn duplicate<T: Clone>(input: &T) -> (T, T) {
    (input.clone(), input.clone())
}

pub fn test_duplicate() {
    let foo = String::from("something");
    let foobar = duplicate(&foo);
    println!("Duplicate of {} is {:?}", foo, foobar);
}

// there is also some syntax sugar we could use to clean up the function
// signature a bit, so instead of fn add_42<T: Into<i32>> (value: T) -> i32
// we could do the following:
pub fn add_42(val: impl Into<i32>) -> i32 {
    val.into() + 42_i32
}

// you can also apply this syntax sugar to return types, defining a return
// type that implements a trait
pub fn pair_of(x: i32) -> impl std::fmt::Debug {
    // this function is generic for any type T that can return
    // a type that implements std::fmt::Debug
    (x+1, x-1)
}