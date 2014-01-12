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
  let mut queue: DList<IntTree> = DList::new();

  let mut tree: IntTree = Branch(1, ~[Leaf(1)]);
  println(tree.to_str());
  println((*tree.children().unwrap()).to_str());
  tree.add_child(10);
  println((*tree.children().unwrap()).to_str());
  (*tree.children().unwrap())[0].add_child(100);
  (*tree.children().unwrap())[0].add_child(100);
  println((*tree.children().unwrap()).to_str());
  println(tree.to_str());

  // shape of test tree for BFS vs DFS
  //          1
  //     2          3
  //  4    5     6     7
  // 8 9 10 11 12 13 14 15

  // FIXME: it shouldn't be so awkward to build this tree, should it?
  // first layer has to be built first...
  let four_tree: IntTree = Branch(4, ~[Leaf(8), Leaf(9)]);
  let five_tree: IntTree = Branch(5, ~[Leaf(10), Leaf(11)]);
  let six_tree: IntTree = Branch(6, ~[Leaf(12), Leaf(13)]);
  let seven_tree: IntTree = Branch(7, ~[Leaf(14), Leaf(15)]);
  // then we can build the second...
  let two_tree: IntTree = Branch(2, ~[four_tree, five_tree]);
  let three_tree: IntTree = Branch(3, ~[six_tree, seven_tree]);
  // the tree topper!
  let one_tree: IntTree = Branch(1, ~[two_tree, three_tree]);
  println(one_tree.to_str());

  let mut cur_node: IntTree;
  let goal_int: int = 8;
  queue.push_front(one_tree);

  println("beginning BFS");
  while true {
    match queue.pop_back() {
      Some(tree) => cur_node = tree,
      None() => {
        println("nope, it's not in here.");
        break;
      }
    }
    println(cur_node.to_str());
    match cur_node {
      Branch(8, _) => {
          println("found it!");
          break;
        },
      Leaf(8) => {
          println("found it!");
          break;
        },
      Branch(val, children) => {
          for child in children.iter() {
            queue.push_front(child.clone()); // I admitted defeat here. I wanted a pointer, but a clone is sufficient. D=
            println("pushing child: " + child.to_str());
          }
        },
      Leaf(_) => {
        println("nope, it's not in here.");
        break;
      }
    }
  }
}
