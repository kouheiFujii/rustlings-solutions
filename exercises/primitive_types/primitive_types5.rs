// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    // タプル: 複数の型をまとめて扱うことができる
    let cat = ("Furry McFurson", 3.5);
    // 他にも cat.0, cat.1 というようにアクセスできる
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
