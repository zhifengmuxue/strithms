/**
 * `DoublyLinkedList` 双向链表
 * 
 * 支持的操作：
 * 1. 创建空链表
 * 2. 从切片创建链表
 * 3. 在链表头部插入元素
 * 4. 在链表尾部插入元素
 * 5. 删除链表头部的元素
 * 6. 删除链表尾部的元素
 * 7. 获取链表长度
 * 8. 判断链表是否为空
 * 9. 格式化输出
 * 10. 合并两个链表
 */

#[derive(Clone)]
#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub prev: Option<Box<Node<T>>>,
    pub next: Option<Box<Node<T>>>,
}

pub struct DoublyLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub tail: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T> DoublyLinkedList<T> {
    /// 创建一个空的双向链表
    /// @return DoublyLinkedList<T>
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// 从切片创建链表
    /// @param slice: 切片
    /// @return DoublyLinkedList<T>
    /// @where T: Clone 表示T类型必须实现Clone trait
    pub fn from(slice: &[T]) -> Self where T: Clone {
        let mut list = DoublyLinkedList::new();
        for item in slice.iter().rev() {
            list.push_front(item.clone());
        }

        list
    }

    /// 在链表头部插入元素
    /// @param data: 插入的数据
    /// @where T: Clone 表示T类型必须实现Clone trait
    pub fn push_front(&mut self, data: T) where T: Clone {
        let new_node = Box::new(Node {
            data,
            prev: None,
            next: self.head.take(),
        });

        if let Some(ref mut head) = self.head {
            head.prev = Some(new_node.clone());
        } else {
            self.tail = Some(new_node.clone());
        }

        self.head = Some(new_node);
        self.length += 1;
    }

    /// 在链表尾部插入元素
    /// @param data: 插入的数据
    /// @where T: Clone 表示T类型必须实现Clone trait
    pub fn push_back(&mut self, data: T) where T: Clone {
        let new_node = Box::new(Node {
            data,
            prev: self.tail.take(),
            next: None,
        });
    
        if let Some(ref mut tail) = self.tail {
            tail.next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
    
        self.tail = Some(new_node);
        self.length += 1;
    }
    
    

    /// 删除链表头部的元素
    /// @return Option<T>
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if let Some(ref mut head) = self.head {
                head.prev = None;
            } else {
                self.tail = None;
            }
            self.length -= 1;
            node.data
        })
    }

    /// 删除链表尾部的元素
    /// @return Option<T>
    pub fn pop_back(&mut self) -> Option<T>{
        self.tail.take().map(|mut node|{
            self.tail = node.prev.take();
            if let Some(ref mut tail) = self.tail{
                tail.next = None;
            }else {
                self.head = None;
            }
            self.length -= 1;
            node.data
        })
    }
    
    /// 获取链表长度
    /// @return usize
    pub fn len(&self) -> usize{
        self.length
    }

    /// 判断是否为空
    /// @return bool
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 格式化输出
    /// @where T: std::fmt::Debug 表示T类型必须实现Debug trait
    pub fn print(&self) where T: std::fmt::Debug {
        let mut current = &self.head;
        print!("head -> ");
        while let Some(node) = current {
            print!("{:?} <-> ", node.data);
            current = &node.next;
        }
        println!("tail");
    }
    

    /// 合并两个链表
    /// @param list: 链表
    /// @return DoublyLinkedList<T>
    /// @where T: Clone
    pub fn margin(&self, list: &DoublyLinkedList<T>) -> DoublyLinkedList<T>
    where T: Clone + std::fmt::Debug {
        let mut new_list = DoublyLinkedList::new();
        let mut current = &list.head;
        while let Some(node) = current {
            new_list.push_front(node.data.clone());
            current = &node.next;
        }
        current = &self.head;
        while let Some(node) = current {
            new_list.push_front(node.data.clone());
            current = &node.next;
        }
        new_list
    }



    /// 反转链表
    /// @return LinkedList<T>
    /// @where T: Clone
    pub fn reverse(&mut self){
        let mut current = self.head.take();
        std::mem::swap(&mut self.head, &mut self.tail);
        while let Some(mut node) = current{
            std::mem::swap(&mut node.next, &mut node.prev);
            current = node.next.take();
        }
    }
}

