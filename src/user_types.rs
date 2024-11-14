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