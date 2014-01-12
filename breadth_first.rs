extern mod extra;
use extra::dlist::DList;
use extra::container::Deque;

#[deriving(Eq, ToStr, Clone)]
enum IntTree {
  Branch(int, ~[IntTree]),
  Leaf(int)
}

impl IntTree {
  fn children (&self) -> Option<~[IntTree]> {
    match self.clone() {
      Branch(_, children) => Some(children.clone()),
      Leaf(_) => None
    }
  }

  fn add_child(&mut self, num: int) {
    let new_self = match self.clone() {
      Branch(val, children) => Branch(val, std::vec::append_one(children, Leaf(num))),
      Leaf(val) => Branch(val, ~[Leaf(num)])
    };
    *self = new_self;
  }
}

fn main() {
  let mut queue: DList<int> = DList::new();
  queue.push_front(1);
  queue.push_front(2);
  println(queue.len().to_str());

  let mut tree: IntTree = Branch(1, ~[Leaf(1)]);
  println(tree.to_str());
  println(tree.children().to_str());
  tree.add_child(10);
  println(tree.children().to_str());
  match tree.children() {
    Some(children) => children[0].add_child(100), // this doensn't work because apparently the children are returned as immutable.
    None => ()
  };
  println(tree.children().to_str());
}

// shape of test tree for BFS vs DFS
//          1
//     2          3
//  4    5     6     7
// 8 9 10 11 12 13 14 15
