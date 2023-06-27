use std::collections::VecDeque;

trait Queue<T> {
    fn deque(&mut self) -> Option<T>;
    fn enqueue(&mut self, value: T);
    fn peek(&self) -> Option<&T>;
    fn size(&self) -> usize;
}

//use std::cmp;

//struct Node<'a, T> {
//    value: Option<T>,
//    next: Option<&'a Node<T>>,
//}

//struct Queue<'a, T> {
//    pub length: usize,
//    head: &Node<'a T>,
//    tail: &Node<'a T>,
//}

//impl<T> Queue<T> {
//    fn new() -> Self {
//        let node = Node {
//            value: None,
//            next: None,
//        };
//        let tail = &node;
//        let queue = Queue {
//            head: &tail,
//            length: 0,
//            tail,
//        };

//        queue
//    }

//    fn enqueue(&self, value: T) {
//        let node = Node {
//            value: Some(value),
//            next: None,
//        };

//        self.tail.next = Some(&node);
//        self.tail = &node;
//        self.head = if self.length == 0 { &node } else { self.head };

//        self.length += 1;
//    }

//    fn deque(&self) -> Option<T> {
//        let node = self.head;

//        self.length = cmp::max(0, self.length - 1);
//        self.tail = if self.length == 0 {
//            &Node {
//                value: None,
//                next: None,
//            }
//        } else {
//            self.tail
//        };
//        self.head = if self.length == 0 {
//            &self.tail
//        } else {
//            self.head.next
//        };

//        node.value
//    }
//}

struct QueueNaive<T> {
    queue: Vec<T>,
}

impl<T> QueueNaive<T> {
    fn new() -> Self {
        let queue: Vec<T> = Vec::new();

        Self { queue }
    }
}

impl<T> Queue<T> for QueueNaive<T> {
    fn enqueue(&mut self, value: T) {
        self.queue.push(value);
    }

    fn deque(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            // Vec::remove shifts every element after it, resulting in a
            // worst-case performance of O(n) - we've lost the O(1) benefit
            // by using this method
            Some(self.queue.remove(0))
        }
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}

struct QueueVecDeque<T> {
    queue: VecDeque<T>,
}

impl<T> QueueVecDeque<T> {
    fn new() -> Self {
        let queue: VecDeque<T> = VecDeque::new();

        Self { queue }
    }
}

impl<T> Queue<T> for QueueVecDeque<T> {
    fn enqueue(&mut self, value: T) {
        self.queue.push_back(value);
    }

    fn deque(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        if self.queue.is_empty() {
            None
        } else {
            self.queue.get(0)
        }
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}

fn evaluate_queue<T, Q>(mut queue: Q, xs: &Vec<T>)
where
    T: std::fmt::Debug + Copy,
    Q: Queue<T>,
{
    for &x in xs.iter() {
        queue.enqueue(x);
    }

    for (i, _) in xs.iter().enumerate() {
        assert_eq!(queue.size(), xs.len() - i);

        println!("Iteration: {i:?}");
        println!("Next value: {:?}", queue.peek());

        let value = queue.deque();

        println!("Popped value: {value:?}\n");
    }
}

fn main() {
    let xs = (5..10).step_by(2).collect::<Vec<i32>>();
    let queue_using_vec: QueueNaive<i32> = QueueNaive::new();
    let queue_using_vecdeque: QueueVecDeque<i32> = QueueVecDeque::new();

    println!("**************");
    println!("queue naive / invalid");
    println!("**************");

    evaluate_queue(queue_using_vec, &xs);

    println!("**************");
    println!("queue using VecDeque");
    println!("**************");

    evaluate_queue(queue_using_vecdeque, &xs);
}
