// 要让此文件独立运行，必须确保包含 main 函数，并保证其为可执行的 main 文件。
// 直接用 `cargo run --bin demo` 或 `cargo run` (如果本文件为主入口) 即可运行。

#[cfg(test)]
mod tests {

    #[test]
    fn test_demo() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];

        // 动态数组 Vec（类似 Java ArrayList）
        let vec = vec![1, 2];

        // 访问元素
        let first = vec[0]; // 可能 panic
        let first_safe = vec.first(); // 返回 Option

        println!("arr: {:?}", arr);
        println!("vec: {:?}", vec);
        println!("first: {}", first);
        println!("first_safe: {:?}", first_safe);
    }
}
