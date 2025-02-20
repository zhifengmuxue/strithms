/// `ArrayList` 顺序表结构，采用数组实现

pub struct ArrayList<T>{
    data: Vec<T>,
}

impl<T> ArrayList<T> {
    /// 初始化
    pub fn new() -> Self{
        ArrayList {data: Vec::new()}
    }

    /// 从切片创建
    pub fn from(slice: &[T]) -> Self where T: Clone{
        ArrayList {
            data: slice.to_vec(),
        }
    }

    /// 增加元素
    pub fn push(&mut self, item: T){
        self.data.push(item);
    }
    /// 删除元素
    pub fn pop(&mut self) -> Option<T>{
        self.data.pop()
    }
    /// 根据index删除元素
    pub fn remove(&mut self, index: usize) -> Option<T>{
        if index < self.data.len(){
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    /// 修改元素
    pub fn update(&mut self, index: usize, item: T) -> Option<T>{
        if index < self.data.len(){
            Some(std::mem::replace(&mut self.data[index], item))
        } else {
            None
        }
    }
    /// 根据index获取元素
    pub fn get(&self, index: usize) -> Option<&T>{
        self.data.get(index)
    }
    /// 获取列表长度
    pub fn len(&self) -> usize{
        self.data.len()
    }
    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// 查找元素位置
    pub fn locate_elem(&self, elem: &T) -> Option<usize> where T: PartialEq{
        self.data.iter().position(|x| x == elem)
    }
    /// 清空列表
    pub fn clear(&mut self){
        self.data.clear();
    }
    /// 输出操作
    pub fn print(&self)
    where T: std::fmt::Debug {
        println!("{:?}", self.data);
    }

    /// 合并两个顺序表
    pub fn margin(&self, list: ArrayList<T>) -> ArrayList<T>
    where T: Clone {
        let mut new_list = ArrayList::new();
        for i in 0..self.len(){
            new_list.push((*self.get(i).unwrap()).clone());
        }
        for i in 0..list.len(){
            new_list.push((*list.get(i).unwrap()).clone());
        }
        new_list
    }
}