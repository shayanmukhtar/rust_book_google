
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

// the keyword Self (capital S) is a type alias for the struct
// type that we're referencing this impl block for. We can, for
// example, use Race instead of Self with no repurcussions. The
// keyword self is short for a binding `self` of type `Self`. 
// `self` follows the same rules as any variables in a method,
// even as they come to ownership. For example, a method that
// takes no argument of `Self` means there is no instantation of 
// a struct that this method operates on. By convention (but this
// isn't necessary) a function called `new` can take no argument of
// self and return a `Self`. 
// otherwise `&mut self` means we have an exclusive read-write
// reference to that instantation.
// &self means we have a shared read-only reference to that 
// instantiation. self means we consume the object and take ownership
// of it (but its still not mutable), and finally mut self takes
// ownership and keeps it mutable.
impl Race {
    // there's no receiver for this method - it does not operate
    // on an instantiation 
    fn new(name: &str) -> Self {
        Self{name: name.to_string(), laps: Vec::new()}
    }

    // this is an exclusive reference to this instantation
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // this is a shared read only reference
    fn print_laps(&self) {
        for (index, lap) in self.laps.iter().enumerate() {
            println!("Lap: {index} - Time {lap}")
        }
    }

    // this consumes the reference, thereby making it unusable
    fn finish(self) {
        println!("Finished race for {}", self.name);
    }
}

#[test]
fn test_new() {
    let mut my_race = Race::new("Shayan");
    my_race.add_lap(43);
    my_race.add_lap(42);
    my_race.add_lap(45);

    my_race.print_laps();

    my_race.finish();

    // after this we cannot use `my_race`
    // my_race.add_lap(33);
}