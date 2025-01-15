#![feature(maybe_uninit_uninit_array)]

pub mod ext;
pub mod cpu;
pub mod core;
use crate::ext::queue::Queue;

fn main() {
    let mut queue = ext::queue::StaticQueue::<i32, 3>::new();
    let _ = queue.push(1);
}
