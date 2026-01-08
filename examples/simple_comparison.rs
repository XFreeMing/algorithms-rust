//! 简单对比示例 - 适合在 Godbolt.org 查看
//!
//! 复制这个文件到 https://godbolt.org/
//! 选择 Rust stable，优化级别 -O
//! 观察右侧的汇编代码

// ========== 示例 1: 数组求和 ==========

/// 方法 1: 手写循环
#[inline(never)] // 阻止内联，便于查看单独的汇编
pub fn sum_manual(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in nums {
        sum += num;
    }
    sum
}

/// 方法 2: 使用迭代器
#[inline(never)]
pub fn sum_iterator(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// ========== 示例 2: Vec 初始化 ==========

/// 方法 1: 逐个 push（低效）
#[inline(never)]
pub fn vec_push() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec
}

/// 方法 2: 使用 vec! 宏（高效）
#[inline(never)]
pub fn vec_macro() -> Vec<i32> {
    vec![1, 2, 3]
}

/// 方法 3: 预分配容量
#[inline(never)]
pub fn vec_with_capacity() -> Vec<i32> {
    let mut vec = Vec::with_capacity(3);
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec
}

// ========== 示例 3: 过滤和映射 ==========

/// 方法 1: 手写循环
#[inline(never)]
pub fn filter_map_manual(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &num in nums {
        if num > 10 {
            result.push(num * 2);
        }
    }
    result
}

/// 方法 2: 迭代器链
#[inline(never)]
pub fn filter_map_iterator(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x > 10).map(|&x| x * 2).collect()
}

// ========== 主函数 ==========

pub fn main() {
    // 使用所有函数，防止被优化掉
    let data = vec![1, 5, 15, 20, 25];

    println!("Sum manual: {}", sum_manual(&data));
    println!("Sum iterator: {}", sum_iterator(&data));

    println!("Vec push: {:?}", vec_push());
    println!("Vec macro: {:?}", vec_macro());
    println!("Vec capacity: {:?}", vec_with_capacity());

    println!("Filter manual: {:?}", filter_map_manual(&data));
    println!("Filter iterator: {:?}", filter_map_iterator(&data));
}
