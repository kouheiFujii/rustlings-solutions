// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;
    // `&mut x` はxの値を「可変な参照」していることを示す。
    // 可変な参照先は、一度に一つしか存在できない。
    // y が変更されて、x の値が反映されることを担保するため。
    let y = &mut x;
    *y += 100;
    // y が変更されたことが確定したので、再度 `&mut x` が可能になる。
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
