#[derive(Debug, Default)]
pub struct List {
    head: Option<Box<Node>>,
}

type Data = i32;

#[derive(Debug)]
struct Node {
    data: Data,
    next: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn length(&self) -> usize {
        let mut count = 0;
        let mut cur = &self.head;
        while let Some(node) = cur {
            count += 1;
            cur = &node.next;
        }
        count
    }

    pub fn push(&mut self, data: Data) {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<Data> {
        match self.head.take() {
            None => None,
            Some(head) => {
                self.head = head.next;
                Some(head.data)
            }
        }
    }

    fn clear(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let l = List::default();
        assert_eq!(format!("{:?}", l), "List { head: None }");
        assert_eq!(l.length(), 0);
    }

    #[test]
    fn push_pop() {
        let mut l = List::default();
        l.push(3);
        l.push(2);
        assert_eq!(
            format!("{:?}", &l),
            "List { head: Some(Node { data: 2, next: Some(Node { data: 3, next: None }) }) }",
        );
        assert_eq!(2, l.length());

        assert_eq!(Some(2), l.pop());
        assert_eq!(Some(3), l.pop());
        assert_eq!(None, l.pop());
        assert_eq!(0, l.length());
    }

    #[test]
    fn clear() {
        let mut l = List::default();
        l.push(1);
        l.push(2);
        l.push(3);
        l.clear();
        assert_eq!(format!("{:?}", l), "List { head: None }");
    }
}
