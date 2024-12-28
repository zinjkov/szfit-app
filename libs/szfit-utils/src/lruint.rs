// use std::alloc::{alloc, dealloc, Layout};
// use std::collections::HashMap;
// use std::fmt::{Debug, Formatter};
// use std::hash::Hash;
// use std::mem::{align_of, size_of};
// use std::ptr;
// use std::ptr::NonNull;
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
//         return unsafe { node_ptr.as_ref().value.value };
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
// #[derive(Default)]
// pub struct IntrusiveList<Value> {
//     front: Option<NonNull<Node<Value>>>,
//     back: Option<NonNull<Node<Value>>>,
// }
//
// implementation<Value: Debug> Debug for IntrusiveList<Value> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         {
//             let mut iter = self.front.clone();
//             write!(f, "\nhead ->");
//             while let Some(node_ptr) = iter {
//                 unsafe {
//                     write!(f, " {:?} ->", node_ptr.as_ref().value);
//                     iter = node_ptr.as_ref().next;
//                 }
//             }
//
//             write!(f, " back\n");
//         }
//
//         {
//             let mut iter = self.back.clone();
//             write!(f, "back ->");
//             while let Some(node_ptr) = iter {
//                 unsafe {
//                     write!(f, " {:?} ->", node_ptr.as_ref().value);
//                     iter = node_ptr.as_ref().prev;
//                 }
//             }
//
//             write!(f, " front");
//         }
//         write!(f, "")
//     }
// }
//
// #[derive(Default)]
// pub struct Node<Value> {
//     pub value: Value,
//     prev: Option<NonNull<Node<Value>>>,
//     next: Option<NonNull<Node<Value>>>,
// }
//
// implementation<Value> IntrusiveList<Value> {
//     pub fn push_front(&mut self, value: Value) -> NonNull<Node<Value>> {
//         let node = Node {
//             value,
//             next: None,
//             prev: None,
//         };
//         let mut node_ptr = unsafe {
//             let node_ptr = NonNull::<Node<Value>>::new_unchecked(Self::alloc_node(node));
//             node_ptr
//         };
//         unsafe {
//             if self.is_empty() {
//                 self.front = Some(node_ptr.clone());
//                 self.back = Some(node_ptr.clone());
//             } else {
//                 let mut front_ptr = self.front.unwrap();
//                 unsafe {
//                     node_ptr.as_mut().next = Some(front_ptr.clone());
//                     front_ptr.as_mut().prev = Some(node_ptr.clone());
//                 }
//                 self.front = Some(node_ptr.clone())
//             }
//         }
//         node_ptr
//     }
//
//     pub fn push_front_node(&mut self, mut node_ptr: NonNull<Node<Value>>) -> NonNull<Node<Value>> {
//         unsafe {
//             if self.is_empty() {
//                 self.front = Some(node_ptr.clone());
//                 self.back = Some(node_ptr.clone());
//             } else {
//                 let mut front_ptr = self.front.unwrap();
//                 unsafe {
//                     node_ptr.as_mut().next = Some(front_ptr.clone());
//                     front_ptr.as_mut().prev = Some(node_ptr.clone());
//                 }
//                 self.front = Some(node_ptr.clone())
//             }
//         }
//         node_ptr
//     }
//
//     pub fn remove(&mut self, mut node_ptr: NonNull<Node<Value>>) -> NonNull<Node<Value>> {
//         unsafe {
//             if self.front.unwrap() == node_ptr {
//                 self.front = node_ptr.as_ref().next;
//             }
//             if self.back.unwrap() == node_ptr {
//                 self.back = node_ptr.as_ref().prev;
//             }
//             if let Some(prev_node) = node_ptr.as_mut().prev.as_mut() {
//                 prev_node.as_mut().next = node_ptr.as_ref().next;
//             }
//             if let Some(next_node) = node_ptr.as_mut().next.as_mut() {
//                 next_node.as_mut().prev = node_ptr.as_ref().prev;
//             }
//             node_ptr.as_mut().prev = None;
//             node_ptr.as_mut().next = None;
//             node_ptr
//         }
//     }
//
//     pub fn remove_back(&mut self) -> Option<NonNull<Node<Value>>> {
//         if self.is_empty() {
//             return None;
//         }
//         unsafe {
//             let mut back = self.back.unwrap().clone();
//             if let Some(mut prev_ptr) = back.as_mut().prev {
//                 prev_ptr.as_mut().next = None;
//                 self.back = Some(prev_ptr)
//             } else {
//                 self.front = None;
//                 self.back = None;
//             }
//
//             Some(back)
//         }
//     }
//
//     pub fn replace_to_front(&mut self, node_ptr: NonNull<Node<Value>>) -> NonNull<Node<Value>> {
//         let node_ptr = self.remove(node_ptr);
//         self.push_front_node(node_ptr)
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.front.is_none() && self.back.is_none()
//     }
//
//     unsafe fn alloc_node(node: Node<Value>) -> *mut Node<Value> {
//         let node_ptr = alloc(Self::node_layout()) as *mut Node<Value>;
//         ptr::write(node_ptr, node);
//         node_ptr
//     }
//
//     unsafe fn dealloc_node(node_ptr: NonNull<Node<Value>>) {
//         ptr::drop_in_place(node_ptr.as_ptr());
//         dealloc(node_ptr.as_ptr() as *mut u8, Self::node_layout());
//     }
//
//     unsafe fn node_layout() -> Layout {
//         let size = size_of::<Node<Value>>();
//         let align = align_of::<Node<Value>>();
//         Layout::from_size_align_unchecked(size, align)
//     }
// }
//
// // #[test]
// // fn test() {
// //     let mut lru = LRUCache::new(1);
// //     lru.put(1, 1);
// //     lru.put(3, 2);
// //     lru.get(1);
// //     lru.put(3, 3);
// //     lru.get(2);
// //     lru.put(4, 4);
// //     lru.get(1);
// //     lru.get(3);
// //     lru.get(4);
// //     println!("{lru:?}");
// // }
