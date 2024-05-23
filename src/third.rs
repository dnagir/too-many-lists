use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn clear(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            let a = Rc::try_unwrap(node);
            match a {
                Ok(mut node) => cur = node.next.take(),
                Err(_) => break, // No need to release others if we still have a pointer.
            }
        }
    }

    pub fn prepend(&self, data: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|x| x.next.clone()),
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
    fn basics() {
        let list = List::default();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works.
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let l = List::new().prepend(1).prepend(2).prepend(3);

        assert_eq!(
            l.iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .join(", "),
            "3, 2, 1",
        );
    }
}
