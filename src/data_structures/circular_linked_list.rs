/**
 * `CircularLinkedList` 循环链表
 * 
 * 一个循环链表是一个链表，其中最后一个节点指向第一个节点。
 * 支持的操作有：
 * 1. 创建一个空的循环链表
 * 2. 从切片创建链表
 * 3. 在链表头部插入元素
 * 4. 删除链表头部的元素
 * 5. 获取链表长度
 * 6. 判断是否为空
 * 7. 格式化输出
 */

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct CircularLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub tail: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T> CircularLinkedList<T> {
    /// 创建一个空的循环链表
    pub fn new() -> Self {
        CircularLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// 从切片创建链表
    pub fn from(slice: &[T]) -> Self
    where
        T: Clone,
    {
        let mut list = CircularLinkedList::new();
        for item in slice.iter().rev() {
            list.push_front(item.clone());
        }
        list
    }

    /// 在链表头部插入元素
    pub fn push_front(&mut self, data: T) where T: Clone{
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        if let Some(ref mut tail) = self.tail {
            tail.next = Some(new_node.clone());
        } else {
            self.tail = Some(new_node.clone());
        }

        self.head = Some(new_node);
        self.length += 1;

        if self.length == 1 {
            self.tail.as_mut().unwrap().next = self.head.clone();
        }
    }

    /// 删除链表头部的元素
    pub fn pop_front(&mut self) -> Option<T> where T: Clone{
        if self.length == 0 {
            return None;
        }

        let head = self.head.take().unwrap();
        self.head = head.next;

        if self.length == 1 {
            self.tail = None;
        } else {
            self.tail.as_mut().unwrap().next = self.head.clone();
        }

        self.length -= 1;
        Some(head.data)
    }

    /// 获取链表长度
    pub fn len(&self) -> usize {
        self.length
    }

    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 格式化输出
    pub fn print(&self)
    where
        T: std::fmt::Debug,
    {
        let mut current = &self.head;
        print!("head -> ");
        for _ in 0..self.length {
            if let Some(node) = current {
                print!("{:?} -> ", node.data);
                current = &node.next;
            }
        }
        println!("head");
    }
}