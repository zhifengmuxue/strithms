/**
 * `ArrayList` 顺序表结构，采用数组实现
 * 
 * 支持的操作：
 * 1. 初始化
 * 2. 从切片创建
 * 3. 增加元素
 * 4. 删除元素
 * 5. 根据index删除元素
 * 6. 修改元素
 * 7. 根据index获取元素
 * 8. 获取列表长度
 * 9. 判断是否为空
 * 10. 查找元素位置
 * 11. 清空列表
 * 12. 输出操作
 * 13. 合并两个顺序表
 */ 

pub struct ArrayList<T>{
    data: Vec<T>,
}

impl<T> ArrayList<T> {
    /// 初始化
    /// @return ArrayList<T>
    pub fn new() -> Self{
        ArrayList {data: Vec::new()}
    }

    /// 从切片创建
    /// @param slice: 切片
    /// @return ArrayList<T>
    /// @where T: Clone 表示T类型必须实现Clone trait
    pub fn from(slice: &[T]) -> Self where T: Clone{
        ArrayList {
            data: slice.to_vec(),
        }
    }

    /// 增加元素
    /// @param item: 元素
    pub fn push(&mut self, item: T){
        self.data.push(item);
    }
    /// 删除元素
    /// @return Option<T>
    pub fn pop(&mut self) -> Option<T>{
        self.data.pop()
    }
    /// 根据index删除元素
    /// @param index: 索引
    /// @return Option<T>
    pub fn remove(&mut self, index: usize) -> Option<T>{
        if index < self.data.len(){
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    /// 修改元素
    /// @param index: 索引
    /// @param item: 元素
    /// @return Option<T>
    pub fn update(&mut self, index: usize, item: T) -> Option<T>{
        if index < self.data.len(){
            Some(std::mem::replace(&mut self.data[index], item))
        } else {
            None
        }
    }
    /// 根据index获取元素
    /// @param index: 索引
    /// @return Option<&T>
    pub fn get(&self, index: usize) -> Option<&T>{
        self.data.get(index)
    }
    /// 获取列表长度
    /// @return usize
    pub fn len(&self) -> usize{
        self.data.len()
    }
    /// 判断是否为空
    /// @return bool
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// 查找元素位置
    /// @param elem: 元素
    /// @return Option<usize>
    /// @where T: PartialEq 表示T类型必须实现PartialEq trait
    pub fn locate_elem(&self, elem: &T) -> Option<usize> where T: PartialEq{
        self.data.iter().position(|x| x == elem)
    }
    /// 清空列表
    pub fn clear(&mut self){
        self.data.clear();
    }
    /// 输出操作
    /// @where T: std::fmt::Debug 表示T类型必须实现Debug trait
    pub fn print(&self)
    where T: std::fmt::Debug {
        println!("{:?}", self.data);
    }

    /// 合并两个顺序表
    /// @param list: ArrayList<T>
    /// @return ArrayList<T>
    /// @where T: Clone 表示T类型必须实现Clone trait
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