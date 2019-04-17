struct Point {
    x: i32,
    y: i32,
}

// struct が持つ値はイミュータブルがデフォルト
// mut を用いるとミュータブルに変更できる
// ただし、ミュータビリティは束縛にのみ付与できる属性であり、
// 構造体の定義に付与できる属性ではない

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = &v1;
    println!("{}, {}", v1[0], v2[1]);

    let origin = Point { x: 0, y: 0 };
    println!("The origin is at ({}, {})", origin.x, origin.y);
}
