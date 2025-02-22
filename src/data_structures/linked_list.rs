/**
 * `LinkList` 单链表
 * 
 * 支持的操作：
 * 1. 创建空链表
 * 2. 从切片创建链表
 * 3. 在链表头部插入元素
 * 4. 删除链表头部的元素
 * 5. 获取链表长度
 * 6. 判断链表是否为空
 * 7. 格式化输出
 * 8. 合并两个链表
 * 9. 反转链表
 */


struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T>{
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T>{
    /// 创建一个空的链表
    /// @return LinkedList<T>
    pub fn new() -> Self{
        LinkedList { head: None, length: 0}
    }

    /// 切片创建
    /// @param slice: 切片
    /// @return LinkedList<T>
    pub fn from(slice: &[T]) -> Self where T: Clone{
        let mut list = LinkedList::new();
        for item in slice.iter().rev(){
            list.push_front(item.clone());
        }
        list
    }

    /// 在链表头部插入元素
    /// @param data: 插入的数据
    pub fn push_front(&mut self, data: T){
        let new_node = Box::new(Node{
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    /// 删除链表头部的元素
    /// @return Option<T>
    pub fn pop_front(&mut self) -> Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }

    /// 获取链表长度
    /// @return usize
    pub fn len(&self) -> usize{
        self.length
    }
    
    /// 判断链表是否为空
    /// @return bool
    pub fn is_empty(&self) -> bool{
        self.length == 0
    }

    /// 格式化输出
    /// @param T: 实现Debug的类型
    /// @where T: std::fmt::Debug
    pub fn print(&self) where T: std::fmt::Debug{
        let mut current = &self.head;
        while let Some(node) = current{
            print!("{:?} -> ", node.data);
            current = &node.next;
        }
        println!("Null");
    }

    /// 合并两个链表
    /// @param list: 链表
    /// @return LinkedList<T>
    /// @where T: Clone
    pub fn merge(&self, list: LinkedList<T>) -> LinkedList<T>
    where T: Clone{
        let mut new_list = LinkedList::new();
        let mut current = &self.head;
        while let Some(node) = current {
            new_list.push_front(node.data.clone());
            current = &node.next;
        }
        let mut current = &list.head;
        while let Some(node) = current {
            new_list.push_front(node.data.clone());
            current = &node.next;
        }
        new_list.reverse();
        new_list
    }

    /// 反转链表
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }
}