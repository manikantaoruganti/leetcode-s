/*impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        
    }
}*/
  use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node {
    val: i64,
    id: usize,
    alive: bool,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i64, id: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val,
            id,
            alive: true,
            prev: None,
            next: None,
        }))
    }
}

struct Pair {
    sum: i64,
    leftid: usize,
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
}

impl Pair {
    fn new(s: i64, id: usize, l: Rc<RefCell<Node>>, r: Rc<RefCell<Node>>) -> Self {
        Pair { sum: s, leftid: id, left: l, right: r }
    }
}

impl Eq for Pair {}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum && self.leftid == other.leftid
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sum == other.sum {
            other.leftid.cmp(&self.leftid)
        } else {
            other.sum.cmp(&self.sum)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new();
        for i in 0..n {
            nodes.push(Node::new(nums[i] as i64, i));
        }

        for i in 0..n - 1 {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
            nodes[i + 1].borrow_mut().prev = Some(Rc::downgrade(&nodes[i]));
        }

        let mut head = nodes[0].clone();
        let mut violation = 0;

        {
            let mut cur = Some(head.clone());
            while let Some(node) = cur {
                let next = node.borrow().next.clone();
                if let Some(ref nxt) = next {
                    if node.borrow().val > nxt.borrow().val {
                        violation += 1;
                    }
                }
                cur = next;
            }
        }

        if violation == 0 {
            return 0;
        }

        let mut pq = BinaryHeap::new();
        {
            let mut cur = Some(head.clone());
            while let Some(node) = cur {
                let next = node.borrow().next.clone();
                if let Some(ref nxt) = next {
                    pq.push(Pair::new(node.borrow().val + nxt.borrow().val, node.borrow().id, node.clone(), nxt.clone()));
                }
                cur = next;
            }
        }

        let mut ops = 0;
        while violation > 0 {
            while let Some(cur) = pq.pop() {
                if !cur.left.borrow().alive || !cur.right.borrow().alive {
                    continue;
                }

                let next_ptr_match = cur.left.borrow().next.as_ref().map(|n| Rc::ptr_eq(n, &cur.right)).unwrap_or(false);
                if !next_ptr_match {
                    continue;
                }

                let left = cur.left.clone();
                let right = cur.right.clone();
                let new_val = left.borrow().val + right.borrow().val;

                let ln = left.borrow().prev.as_ref().and_then(|w| w.upgrade());
                let rn = right.borrow().next.clone();

                let mut old_count = 0;
                if let Some(ref l) = ln {
                    if l.borrow().val > left.borrow().val {
                        old_count += 1;
                    }
                }
                if left.borrow().val > right.borrow().val {
                    old_count += 1;
                }
                if let Some(ref r) = rn {
                    if right.borrow().val > r.borrow().val {
                        old_count += 1;
                    }
                }

                let mut new_count = 0;
                if let Some(ref l) = ln {
                    if l.borrow().val > new_val {
                        new_count += 1;
                    }
                }
                if let Some(ref r) = rn {
                    if new_val > r.borrow().val {
                        new_count += 1;
                    }
                }

                violation = violation - old_count + new_count;

                let new_node = Node::new(new_val, left.borrow().id);

                if let Some(ref l) = ln {
                    l.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(Rc::downgrade(l));
                } else {
                    head = new_node.clone();
                }

                if let Some(ref r) = rn {
                    r.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                    new_node.borrow_mut().next = Some(r.clone());
                }

                left.borrow_mut().alive = false;
                right.borrow_mut().alive = false;

                if let Some(ref l) = ln {
                    pq.push(Pair::new(l.borrow().val + new_node.borrow().val, l.borrow().id, l.clone(), new_node.clone()));
                }
                if let Some(ref r) = rn {
                    pq.push(Pair::new(new_node.borrow().val + r.borrow().val, new_node.borrow().id, new_node.clone(), r.clone()));
                }

                ops += 1;
                break;
            }
        }

        ops
    }
}
