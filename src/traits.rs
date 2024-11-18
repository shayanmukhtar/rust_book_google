// traits are basically interfaces that define methods that must be fulfilled.
// Kind of like header files defining a method, but the underlying implementation
// would be dependent on the fulfilling function. 
trait Pet {
    // notice this is just the function signature, not the implementation itself.
    // This, however, doesn't have to be the case - we can provide an implementation
    // here, and this would serve as the default implementation for anyone implementing
    // for this trait
    fn talk(&self) -> String;

    // again, just the signature, takes shared read-only reference and returns ()
    // but it provides a default implementation
    fn greet(&self)
    {
        println!("Hey - {}", self.talk());
    }
}

// to implement a trait for a type, the syntax is
// impl `Trait` for `Type`
// e.g.
struct Dog {
    name: String,
    age: i8
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof woof, my name is {} and I'm {} years old", self.name, self.age)
    }
}

pub fn create_dog() {
    let fido: Dog = Dog {name: "Fido".to_string(), age: 11};
    fido.greet();
}

// quick note here - its not enough to have an impl block that 
// has a function called `talk` in this case. We must make it part
// of the block that specifies impl `Trait` for `Type`

// we can have something akin to trait inheritence, where one 
// trait can "extend" another trait, and then the implementor
// has to fulfill both traits 
// e.g.
trait Animal {
    fn number_legs(&self) -> u8;
}

// the book cautions against thinking of this as classical OO 
// inheritence - it just specifies another trait that must be
// implemented for these types
trait Companion: Animal {
    fn talk(&self) -> String;
}

struct Cat(String);

impl Animal for Cat {
    fn number_legs(&self) -> u8 {
        4
    }
}

impl Companion for Cat {
    fn talk(&self) -> String {
        // apparently we need the .clone here for a tuple struct
        format!("Meow! - my name is {}", self.0.clone())
    }
}

pub fn create_cat() {
    let bailey: Cat = Cat("Bailey".to_string());
    println!("{}", bailey.talk());
}

// associated types - sometimes in the trait definition we don't know
// what a specific type will be, so we leave it up to the implementor
// of the trait. This is often done in the standard lib
// e.g.
#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    // this is an associated type - we don't know the value of this
    // at this time, the implementor decides what it will be
    type Output;

    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    // now we must specify exactly what type `Output` shall be
    type Output = MetersSquared;
    
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

pub fn test_multiply() {
    let mulitplicand: Meters = Meters(40);
    let multiplier: Meters = Meters(100);

    let product = mulitplicand.multiply(&multiplier);

    println!("{} x {} = {}", mulitplicand.0, multiplier.0, product.0);
}

// we've seen the #[derive(Debug)] statement a few times now, what this
// is actually doing is using a macro that automatically impl's a trait
// for the type you just declare underneath it. In this case, we do 
// impl Debug for `Type`. We can also use other derived traits in the 
// same way, like Clone, or Default (names should be self evident for 
// what they achieve)
#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    health: u8,
}

pub fn test_derived_traits() {
    // uses the trait Default to fill in default values for the fields
    let player_1 = Player::default();

    // the Clone trait adds the clone method as an implementation
    let mut player_2 = player_1.clone();

    player_2.name = String::from("Billy");

    // finally the debug trait adds the `:?` format
    println!("{player_1:?} versus {player_2:?}");
}