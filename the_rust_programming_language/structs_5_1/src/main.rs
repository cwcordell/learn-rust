// Tuple structs allow adding names to tuples. They have the added benefit of making the tuple struct's
// purpose more clear. Tuple structs are useful when you want to give the whole tuple a name and make the
// tuple be a different type from other tuples, and naming each field as a struct would be overkill.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}