#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn length(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn clear(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Default for List<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(node) => {
                self.current = node.next.as_deref();
                Some(&node.data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let l = List::<i32>::new();
        assert_eq!(format!("{:?}", l), "List { head: None }");
        assert_eq!(l.length(), 0);
    }

    #[test]
    fn push_pop() {
        let mut l = List::<i32>::default();
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
    fn peek() {
        let mut l = List::<i32>::default();
        assert_eq!(l.peek(), None);
        l.push(3);
        assert_eq!(l.peek(), Some(&3));
        assert_eq!(l.length(), 1);

        if let Some(v) = l.peek_mut() {
            *v = 99
        }
        assert_eq!(l.peek(), Some(&99));
    }

    #[test]
    fn clear() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        l.clear();
        assert_eq!(format!("{:?}", l), "List { head: None }");
    }

    #[test]
    fn iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);

        let mut s = String::new();
        for v in l.iter() {
            s.push_str(&format!("{v}"));
        }
        assert_eq!(s, "321");

        assert_eq!(
            l.iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .join(", "),
            "3, 2, 1",
        );
    }
}
