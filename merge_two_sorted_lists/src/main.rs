fn main() {
    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    pub struct Solution;
    impl Solution {
        pub fn merge_two_lists(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // 再帰法で求める
            if list1.is_none() && list2.is_none() {
                return None;
            }

            if list1.is_none() {
                return list2;
            }

            if list2.is_none() {
                return list1;
            }

            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                if let Some(mut l1_node) = list1 {
                    l1_node.next = Solution::merge_two_lists(l1_node.next, list2);
                    return Some(l1_node);
                }
            }
            if let Some(mut l2_node) = list2 {
                l2_node.next = Solution::merge_two_lists(list1, l2_node.next);
                return Some(l2_node);
            }
            None
        }
    }

    // 値の準備
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));
    let result = Solution::merge_two_lists(list1, list2);
    println!("result:{:?}", result);
}
