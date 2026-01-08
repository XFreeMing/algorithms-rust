//! 汇编代码对比示例
//!
//! 用于演示 Rust 的"零成本抽象"
//!
//! 运行方式：
//! 1. 生成汇编：cargo rustc --release --example asm_demo -- --emit asm
//! 2. 查看汇编：cat target/release/examples/asm_demo-*.s | less
//! 3. 或使用：cargo asm --example asm_demo

use std::collections::HashMap;

// ============= 示例 1：数组求和对比 =============

/// 手写循环 - 低级写法
pub fn sum_manual(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..nums.len() {
        sum += nums[i];
    }
    sum
}

/// 迭代器 - 高级抽象
pub fn sum_iterator(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

/// 函数式风格 - 链式调用
pub fn sum_functional(nums: &[i32]) -> i32 {
    nums.iter().copied().sum()
}

// ============= 示例 2：数组过滤和映射 =============

/// 手写循环版本
pub fn filter_map_manual(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &num in nums {
        if num > 10 {
            result.push(num * 2);
        }
    }
    result
}

/// 迭代器版本（零成本抽象）
pub fn filter_map_iterator(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x > 10).map(|&x| x * 2).collect()
}

// ============= 示例 3：Vec 初始化对比 =============

/// 低效方式：逐个 push
pub fn vec_push_style() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec
}

/// 高效方式：使用宏
pub fn vec_macro_style() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

/// 预分配容量
pub fn vec_with_capacity() -> Vec<i32> {
    let mut vec = Vec::with_capacity(5);
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec
}

// ============= 示例 4：Two Sum 算法 =============

/// 经典的 Two Sum 问题
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&j) = map.get(&diff) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

// ============= 示例 5：查找最大值 =============

/// 手写循环
pub fn find_max_manual(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let mut max = nums[0];
    for &num in &nums[1..] {
        if num > max {
            max = num;
        }
    }
    Some(max)
}

/// 迭代器方式
pub fn find_max_iterator(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().max()
}

// ============= 主函数：测试所有实现 =============

fn main() {
    println!("=== 零成本抽象示例 ===\n");

    // 测试数据
    let data = vec![1, 5, 15, 20, 25, 8, 30];

    // 示例 1：求和
    println!("示例 1: 数组求和");
    println!("手写循环:   {}", sum_manual(&data));
    println!("迭代器:     {}", sum_iterator(&data));
    println!("函数式:     {}\n", sum_functional(&data));

    // 示例 2：过滤和映射
    println!("示例 2: 过滤大于10的数并乘2");
    println!("手写循环:   {:?}", filter_map_manual(&data));
    println!("迭代器:     {:?}\n", filter_map_iterator(&data));

    // 示例 3：Vec 初始化
    println!("示例 3: Vec 初始化");
    println!("Push 方式:  {:?}", vec_push_style());
    println!("宏方式:     {:?}", vec_macro_style());
    println!("预分配:     {:?}\n", vec_with_capacity());

    // 示例 4：Two Sum
    println!("示例 4: Two Sum");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("输入: {:?}, 目标: {}", nums, target);
    println!("结果: {:?}\n", two_sum(nums, target));

    // 示例 5：查找最大值
    println!("示例 5: 查找最大值");
    println!("手写循环:   {:?}", find_max_manual(&data));
    println!("迭代器:     {:?}", find_max_iterator(&data));

    println!("\n✅ 所有示例运行完成！");
    println!("💡 提示: 查看汇编代码，你会发现高级抽象和低级代码生成的机器码几乎相同！");
}
