#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        // 将新元素直接放入非空队列（默认放入 q1）
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() && self.q2.is_empty() {
            return Err("Stack is empty");
        }

        // 将 q1 中的元素（除最后一个）移动到 q2
        while self.q1.size() > 1 {
            let elem = self.q1.dequeue().unwrap();
            self.q2.enqueue(elem);
        }

        // q1 中的最后一个元素是栈顶元素，弹出它
        let top_elem = self.q1.dequeue().unwrap();

        // 交换 q1 和 q2，使 q1 始终是主队列
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(top_elem)
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_as_stack() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}