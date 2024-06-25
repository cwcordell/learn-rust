// Unit-Like Structs are structs without any fields.
// They are useful in situations in which you need to implement a trait on some type but don't have any data to store in the type itself.
// The behave similarly to unit types empty tuple, which are written as ().

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}