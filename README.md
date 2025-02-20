# strithms (Structures and Algorithms)

这是一个用 Rust 实现常见数据结构和算法的项目

## 项目结构

## 使用方法

### 构建项目

在项目根目录下运行以下命令来构建项目

```sh
cargo build
```

### 运行项目
在项目根目录下运行以下命令来运行项目

```sh
cargo run
```

### 运行测试
在项目根目录下运行以下命令来执行测试

```sh
cargo test
```

## 数据结构
### ArrayList
ArrayList 是一个顺序表数据结构，采用数组实现。以下是 ArrayList 的一些方法：

- `new()`: 创建一个空的 ArrayList。
- `from(slice: &[T])`: 从切片创建一个 ArrayList。
- `push(item: T)`: 增加元素。
- `pop() -> Option<T>`: 删除并返回最后一个元素。
- `remove(index: usize) -> Option<T>`: 根据索引删除元素。
- `update(index: usize, item: T) -> Option<T>`: 修改元素。
- `get(index: usize) -> Option<&T>`: 根据索引获取元素。
- `len() -> usize`: 获取列表长度。
- `is_empty() -> bool`: 判断列表是否为空。
- `locate_elem(elem: &T) -> Option<usize>`: 查找元素位置。
- `clear()`: 清空列表。
- `print()`: 输出列表内容。
- `margin(list: ArrayList<T>) -> ArrayList<T>`: 合并两个顺序表。

## 贡献
欢迎提交问题和拉去请求来改进此项目

## 许可证
此项目使用 MIT 许可证。有关更多信息，请参见 LICENSE 文件。