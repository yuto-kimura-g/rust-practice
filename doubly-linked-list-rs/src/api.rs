/// url: https://gist.github.com/d0iasm/896f6af69cc09456a063443698aef7e7

struct Node<T> {}

struct DoublyLinkedList<T> {}

impl<T> DoublyLinkedList<T> {
    /// Removes the last element from a list and returns it, or None if it is empty.
    /// This operation should compute in O(1) time.
    fn pop_back(&mut self) -> Option<T> {}

    /// Removes the first element and returns it, or None if the list is empty.
    /// This operation should compute in O(1) time.
    fn pop_front(&mut self) -> Option<T> {}

    /// Appends an element to the back of a list.
    /// This operation should compute in O(1) time.
    fn push_back(&mut self, elt: T) {}

    /// Adds an element first in the list.
    /// This operation should compute in O(1) time.
    fn push_front(&mut self, elt: T) {}
}

impl<T> fmt::Display for DoublyLinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("{}", list); // 1<--->2<--->3

    list.push_front(4);
    list.push_front(5);

    println!("{}", list); // 5<--->4<--->1<--->2<--->3
}
