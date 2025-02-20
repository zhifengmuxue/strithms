# strithms (Structures and Algorithms)

这是一个用 Rust 实现常见数据结构和算法的项目

## 项目结构

```
structures_algorithms/ 
├── .gitignore 
├── Cargo.lock 
├── Cargo.toml 
├── LICENSE 
├── README.md 
├── src/ 
│ ├── data_structures/ 
│ │ ├── array_list.rs 
│ │ ├── array_list_test.rs 
│ │ ├── linked_list.rs 
│ │ ├── linked_list_test.rs 
│ │ └── mod.rs 
│ ├── lib.rs 
│ └── main.rs 
└── target/
```

- **.gitignore**: 指定 Git 应忽略的文件和目录。
- **Cargo.lock**: 锁定依赖项的确切版本。
- **Cargo.toml**: 项目的配置文件，包含项目的元数据和依赖项。
- **LICENSE**: 项目的许可证文件。
- **README.m**: 项目的自述文件。
- **src/**: 源代码目录。
  - **data_structures/**: 包含各种数据结构的实现和测试。
    - **array_list.rs**: 顺序表（ArrayList）的实现。
    - **array_list_test.rs**: 顺序表（ArrayList）的测试代码。
    - **linked_list.rs**: 单链表（LinkedList）的实现。
    - **linked_list_test.rs**: 单链表（LinkedList）的测试代码。
    - **mod.rs**: 声明数据结构模块。
  - **lib.rs**: 库文件，用于声明模块。
  - **main.rs**: 项目的入口文件，包含 `main` 函数。
- **target/**: 编译输出目录。


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

- new(): 创建一个空的 ArrayList。
- from(slice: &[T]): 从切片创建一个 ArrayList。
- push(item: T): 增加元素。
- pop() -> Option<T>: 删除并返回最后一个元素。
- remove(index: usize) -> Option<T>: 根据索引删除元素。
- update(index: usize, item: T) -> Option<T>: 修改元素。
- get(index: usize) -> Option<&T>: 根据索引获取元素。
- len() -> usize: 获取列表长度。
- is_empty() -> bool: 判断列表是否为空。
- locate_elem(elem: &T) -> Option<usize>: 查找元素位置。
- clear(): 清空列表。
- print(): 输出列表内容。
- margin(list: ArrayList<T>) -> ArrayList<T>: 合并两个顺序表。

## LinkedList
LinkedList 是一个单链表数据结构。以下是 LinkedList 的一些方法：

- new(): 创建一个空的 LinkedList。
- from(slice: &[T]): 从切片创建一个 LinkedList。
- push_front(data: T): 在链表头部插入元素。
- pop_front() -> Option<T>: 删除并返回链表头部的元素。
- len() -> usize: 获取链表长度。
- is_empty() -> bool: 判断链表是否为空。
- print(): 格式化输出链表内容。
- merge(list: LinkedList<T>) -> LinkedList<T>: 合并两个链表。
- reverse(): 反转链表。


## 贡献
欢迎提交问题和拉去请求来改进此项目

## 许可证
此项目使用 MIT 许可证。有关更多信息，请参见 LICENSE 文件。