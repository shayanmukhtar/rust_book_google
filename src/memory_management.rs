// a few important notes:
fn test_copy_strings() {
    let s1 = String::from("Hello!");
    let s2 = s1;

    // what just happened? Unlike C++, s2 = s1 does not perform a copy
    // rather, s1 and s2 as structs both live on the stack. Inside the 
    // String struct is a pointer to heap allocated memory that holds 
    // "Hello!". When you say s2 = s1, what happens is you make a new 
    // String struct on the stack, and the pointer inside that struct 
    // now points to the same heap location s1's pointer did. However, 
    // at this point, s1 is no longer accessible (the compiler won't 
    // compile code like that). 
}

fn print_string(s1: String) {
    println!("String was {s1}");
}

fn print_string_ref(s1: &String) {
    println!("String was {s1}");
}

pub fn test_print_string() {
    let s1: String = String::from("World");
    print_string(s1);
    // the above function pointer `print_string` now has ownership
    // of s1. Using s1 after this would not compile
    // e.g println!("Still using s1: {s1}");

    // if you want to continue using s1, you have two choices - pass
    // by reference (called borrowing in rust)
    let s1: String = String::from("World");
    print_string_ref(&s1);

    // or you can perform a clone of s1 - this will be a deep clone, 
    // and it will not just add a new stack variable, but will instead
    // incur a new heap allocation
    print_string(s1.clone());

    println!("String still works: {s1}");
}

// on primitive data types, the management is a bit different. Even 
// structs that only contain primitive types can use a trait called 
// copy. This will produce similar results to the c++ copy on assignment
// mechanism. 

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn test_copy_trait() {
    let p1: Point = Point{x: 10, y: 11};
    let p2 = p1;

    // with strings, there was no additional allocation when we did the
    // assignment. However, because Point implements the copy trait, 
    // p2 now has its own copy of p1, and p1 is still accessible after 
    // the assignment to p2
    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);

    // if we added String to Point, this wouldn't compile. That's because 
    // String does not implement the Copy trait.
}

// similar to other languages, we can specify what happens when a value is dropped
// by going out of scope by implementing the the Drop trait
#[derive(Debug)]
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    // why does drop take a mutable reference and not just consume `self`?
    // Think about it, if it consumed `self` then at the end of the drop
    // function, we'd technically drop the value, so we'd have to call 
    // the drop function again. This would create an infinite recursion. 
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

pub fn test_drop() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    // if we didn't specifically call drop here, then a would be dropped
    // after the function exited. drop is just a function that consumes
    // the value, and then the drop impl is called, if one exists. 
    drop(a);
    println!("Exiting test_drop");
}