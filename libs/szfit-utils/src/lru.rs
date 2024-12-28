// use std::collections::HashMap;
// use std::fmt::Debug;
// use std::ptr::NonNull;
//
// use crate::intrusive_list::{IntrusiveList, Node};
//
// #[derive(Default, Debug)]
// struct LRUCache {
//     capacity: i32,
//     map: HashMap<i32, NonNull<Node<NodeValue>>>,
//     list: IntrusiveList<NodeValue>,
// }
//
// #[derive(Debug, Default)]
// struct NodeValue {
//     key: i32,
//     value: i32,
// }
//
// implementation LRUCache {
//     fn new(capacity: i32) -> Self {
//         Self {
//             capacity,
//             ..Default::default()
//         }
//     }
//
//     fn get(&mut self, key: i32) -> i32 {
//         let Some(mut node_ptr) = self.map.get(&key).cloned() else {
//             return -1;
//         };
//         let node_ptr = self.list.replace_to_front(node_ptr);
//         unsafe { node_ptr.as_ref().value.value }
//     }
//
//     fn put(&mut self, key: i32, value: i32) {
//         if let Some(mut node) = self.map.get(&key).cloned() {
//             unsafe { node.as_mut().value.value = value; }
//             self.list.replace_to_front(node);
//         } else {
//             if self.capacity == self.map.len() as i32 {
//                 if let Some(node) = self.list.remove_back() {
//                     unsafe {
//                         self.map.remove(&node.as_ref().value.key);
//                         IntrusiveList::dealloc_node(node);
//                     }
//                 }
//             }
//             let node = self.list.push_front(NodeValue { key, value });
//             self.map.insert(key, node);
//         }
//     }
// }
//
// #[test]
// fn test() {
//     let mut lru = LRUCache::new(105);
//     lru.put(1, 1);
//     lru.put(2, 2);
//     lru.put(3, 3);
//     lru.put(4, 4);
//     lru.put(5, 5);
//     lru.put(6, 6);
//     println!("=\n{lru:?}");
//     println!("22 {}", lru.get(1));
// }
