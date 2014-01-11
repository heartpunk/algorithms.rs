extern mod extra;
use extra::dlist::DList;
use extra::container::Deque;

#[deriving(Eq, ToStr)]
enum IntTree {
  Branch(~[IntTree])
}

fn main() {
  let mut queue: DList<int> = DList::new();
  queue.push_front(1);
  queue.push_front(2);
  println(queue.len().to_str());

  let mut tree: IntTree = Branch(~[]);
  println(tree.to_str());
}
