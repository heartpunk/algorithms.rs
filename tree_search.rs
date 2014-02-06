extern mod extra;
use extra::dlist::DList;
use extra::container::Deque;

#[deriving(Eq, ToStr, Clone)]
enum IntTree {
  Branch(int, ~[IntTree]),
  Leaf(int)
}

enum SearchStrategy {
  BFS,
  DFS
}

fn print_queue(queue: &DList<IntTree>) {
  println("[");
  for (elt, pos) in queue.iter().zip(range(0, queue.len())) {
    print("  ");
    print(elt.to_str());
    if (pos != queue.len()) {
      println(",");
    }
  }
  println("]");
}

fn search_int_tree(tree: &IntTree, goal_int: int, strategy: SearchStrategy) {
  let mut cur_node: IntTree;
  let mut queue: DList<IntTree> = DList::new();
  queue.push_front(tree.clone());

  match strategy {
    BFS => println("\nbeginning BFS"),
    DFS => println("\nbeginning DFS")
  }
  loop {
    print_queue(&queue);
    let next_node: Option<IntTree> = match strategy {
      BFS => queue.pop_back(),
      DFS => queue.pop_front()
    };
    match next_node {
      Some(tree) => cur_node = tree,
      None() => {
        println("nope, it's not in here.");
        break;
      }
    }
    println(cur_node.to_str());
    match cur_node {
      Branch(val, children) => {
          if (val == goal_int) {
            println("found it!");
            break;
          } else {
            for child in children.iter() {
              queue.push_front(child.clone()); // I admitted defeat here. I wanted a pointer, but a clone is sufficient. D=
              println("pushing child: " + child.to_str());
            }
          }
        },
      Leaf(val) => {
        if (val == goal_int) {
          println("found it!");
          break;
        }
      }
    }
  }
}

fn main() {
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

  let goal_int: int = 8;

  search_int_tree(&one_tree.clone(), goal_int, BFS);
  search_int_tree(&one_tree.clone(), goal_int, DFS);
}
