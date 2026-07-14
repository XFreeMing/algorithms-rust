1.  布隆过滤器的实现及应用

参考链接

- [布隆过滤器的原理和实现](https://www.cnblogs.com/cpselvis/p/6265825.html)

- [使用布隆过滤器解决缓存击穿、垃圾邮件识别、集合判重](https://blog.csdn.net/tianyaleixiaowu/article/details/74721877)

- [布隆过滤器 Python 代码示例](https://shimo.im/docs/UITYMj1eK88JCJTH)

- [布隆过滤器 Python
  实现示例](https://www.geeksforgeeks.org/bloom-filters-introduction-and-python-implementation/)

- [高性能布隆过滤器 Python 实现示例](https://github.com/jhgg/pybloof)

- [布隆过滤器 Java 实现示例
  1](https://github.com/lovasoa/bloomfilter/blob/master/src/main/java/BloomFilter.java)

- [布隆过滤器 Java 实现示例
  2](https://github.com/Baqend/Orestes-Bloomfilter)

2.  LRU Cache的实现、应用和题解

参考链接

- [ Understanding the Meltdown
  exploit](https://www.sqlpassion.at/archive/2018/01/06/understanding-the-meltdown-exploit-in-my-own-simple-words/)

- [替换算法总揽](https://en.wikipedia.org/wiki/Cache_replacement_policies)

- [ LRU Cache Python 代码示例](https://shimo.im/docs/CoyPAyXooGcDuLQo)

实战题目 / 课后作业

- [ LRU
  缓存机制](https://leetcode-cn.com/problems/lru-cache/#/)（亚马逊、字节跳动、Facebook、微软在半年内面试中常考）

## 三、课程内容安排与学习目标

### 1. 布隆过滤器的实现及应用

**什么是布隆过滤器？**

- **核心思想**：
  - 概率型数据结构，用于快速判断元素是否存在
  - 位数组 + 多个哈希函数
  - 可能误判"存在"，但绝不会误判"不存在"

- **特性**：
  - 空间效率和查询时间远超一般数据结构
  - 有误判率（可调节）
  - 不支持删除（Counting Bloom Filter 可解决）

- **应用场景**：
  - 缓存穿透防护
  - 垃圾邮件过滤
  - 大规模数据判重（爬虫 URL 去重）

### 2. LRU Cache 的实现、应用和题解

**LRU Cache：**
- **核心需求**：O(1) 的 get 和 put
- **数据结构**：HashMap + 双向链表
- **淘汰策略**：最近最少使用

- **工程应用**：
  - 数据库缓存、DNS 缓存
  - 页面置换算法
  - 各种框架的缓存组件

**核心学习点：**
- 理解布隆过滤器的原理和误判率
- 掌握 LRU Cache 的 HashMap + 双向链表实现
- 学会结合两种结构解决实际工程问题

## 四、学习路径总结

```
布隆过滤器原理（位数组 + 哈希）
    ↓
布隆过滤器应用（缓存穿透、判重）
    ↓
LRU Cache 设计（HashMap + 双向链表）
    ↓
工程综合应用
```
