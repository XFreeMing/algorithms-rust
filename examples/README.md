# 📚 Examples 目录说明

这个目录包含了用于学习和演示 Rust 特性的示例代码。

---

## 📁 文件列表

### 1. `asm_demo.rs` - 完整汇编对比示例

**用途：** 全面展示 Rust 的"零成本抽象"

**包含示例：**
- ✅ 数组求和（手写 vs 迭代器 vs 函数式）
- ✅ 数组过滤和映射
- ✅ Vec 初始化对比（push vs 宏 vs 预分配）
- ✅ Two Sum 算法实现
- ✅ 查找最大值

**运行：**
```bash
cargo run --release --example asm_demo
```

**查看汇编：**
```bash
cargo asm --example asm_demo --rust
```

---

### 2. `simple_comparison.rs` - 简化对比示例

**用途：** 更简洁的版本，所有函数都加了 `#[inline(never)]`，方便单独查看汇编

**特点：**
- 每个函数都可以在汇编中独立查看
- 适合初学者理解编译器优化
- 适合复制到 Godbolt.org 查看

**运行：**
```bash
cargo run --release --example simple_comparison
```

**推荐用法：**
1. 复制代码到 https://godbolt.org/
2. 选择 Rust stable
3. 添加编译选项 `-O`
4. 观察右侧汇编代码

---

## 🚀 快速开始

### 运行所有示例

```bash
# 运行 asm_demo
cargo run --release --example asm_demo

# 运行 simple_comparison
cargo run --release --example simple_comparison
```

### 查看汇编代码

```bash
# 方法 1: 使用 cargo-asm（需要先安装）
cargo install cargo-show-asm
cargo asm --example simple_comparison --rust

# 方法 2: 生成汇编文件
cargo rustc --release --example simple_comparison -- --emit asm
cat target/release/examples/simple_comparison-*.s

# 方法 3: 在线查看（推荐）
# 访问 https://godbolt.org/ 并粘贴代码
```

---

## 📖 相关文档

在 `teach/` 目录下：

1. **[查看汇编代码指南.md](../teach/查看汇编代码指南.md)**
   - 详细的使用说明
   - 工具安装指南
   - 汇编指令解读

2. **[汇编对比结果.md](../teach/汇编对比结果.md)**
   - 实际对比结果
   - SIMD 优化分析
   - 性能结论

3. **[0108.md](../teach/0108.md)**
   - 数组、链表、跳表学习教程
   - 完整的学习路线

---

## 🎯 学习目标

通过这些示例，你将学会：

1. ✅ **验证零成本抽象** - 高级语法不影响性能
2. ✅ **理解编译器优化** - SIMD、循环展开等
3. ✅ **掌握查看汇编的方法** - 多种工具和技巧
4. ✅ **建立性能信心** - 放心使用迭代器等抽象

---

## 💡 关键发现

### 发现 1：迭代器 = 手写循环（性能上）

```rust
// 这两个函数生成几乎相同的汇编代码
fn sum_manual(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..nums.len() { sum += nums[i]; }
    sum
}

fn sum_iterator(nums: &[i32]) -> i32 {
    nums.iter().sum()
}
```

### 发现 2：编译器使用 SIMD 优化

即使你写简单的循环，编译器也会自动使用 ARM NEON SIMD 指令：
- 一次处理 4-16 个元素
- 自动循环展开
- 向量化并行计算

### 发现 3：`vec!` 宏最优

```rust
// 汇编代码行数对比
vec_push()          // ~128 行
vec_with_capacity() // ~63 行
vec_macro()         // ~59 行 ✅ 最优
```

---

## 🛠️ 实用命令

```bash
# 快速查看所有可用函数
cargo asm --example simple_comparison --rust | head -20

# 查看特定函数（按序号，比如 11）
cargo asm --example simple_comparison 11 --rust

# 保存汇编到文件
cargo asm --example simple_comparison 11 --rust > output.asm

# 对比两个函数
cargo asm --example simple_comparison 11 --rust > sum_manual.asm
cargo asm --example simple_comparison 10 --rust > sum_iterator.asm
diff sum_manual.asm sum_iterator.asm
```

---

## 🔗 在线资源

- **Godbolt Compiler Explorer**: https://godbolt.org/
- **Rust Playground**: https://play.rust-lang.org/
- **Rust Performance Book**: https://nnethercote.github.io/perf-book/
- **cargo-show-asm**: https://github.com/pacak/cargo-show-asm

---

## ❓ 常见问题

### Q: 为什么有些函数找不到？

**A:** 函数可能被内联了。在 `simple_comparison.rs` 中，所有函数都加了 `#[inline(never)]` 来避免这个问题。

### Q: 汇编代码太复杂了？

**A:** 不用担心！重点是：
- 观察代码长度（优化后更短）
- 查找 SIMD 指令（ldp q, add.4s 等）
- 对比不同实现的相似度

### Q: 性能真的一样吗？

**A:** 是的！你可以：
1. 查看汇编代码（几乎相同）
2. 运行 benchmark（结果相近）
3. 相信编译器（它比我们聪明）

---

## 🎓 下一步

1. ✅ 运行两个示例程序
2. ✅ 使用 Godbolt 查看在线汇编
3. ✅ 阅读 `teach/查看汇编代码指南.md`
4. ✅ 开始做 `teach/0108.md` 中的算法题

**祝学习愉快！** 🚀

---

**创建日期：** 2026-01-08  
**维护者：** AI Assistant  
**项目：** algorithms-rust
