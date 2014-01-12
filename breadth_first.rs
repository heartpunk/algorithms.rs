extern mod extra;
use extra::dlist::DList;
use extra::container::Deque;

#[deriving(Eq, ToStr, Clone)]
enum IntTree {
  Branch(int, ~[IntTree]),
  Leaf(int)
}

impl IntTree {
  fn children<'a> (&'a mut self) -> Option<&'a mut ~[IntTree]> {
    match self {
      &Branch(_, ref mut children) => Some(children),
      &Leaf(_) => None
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
  println((*tree.children().unwrap()).to_str());
  tree.add_child(10);
  println((*tree.children().unwrap()).to_str());
  (*tree.children().unwrap())[0].add_child(100);
  (*tree.children().unwrap())[0].add_child(100);
  println((*tree.children().unwrap()).to_str());
  println(tree.to_str());
}

// shape of test tree for BFS vs DFS
//          1
//     2          3
//  4    5     6     7
// 8 9 10 11 12 13 14 15
