use std::time::SystemTime;
use lockfree::queue::Queue;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU64, Ordering};

lazy_static! {
    static ref MERKLE_TREE_INTERVALS: Queue<u64> = Queue::new();
}

// Use AtomicU64 to store the timestamp
static START_TIMESTAMP: AtomicU64 = AtomicU64::new(0);

pub fn mt_start_timer() {
    let now = SystemTime::now();
    // Store duration since UNIX_EPOCH
    START_TIMESTAMP.store(
        now.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64,
        Ordering::SeqCst
    );
}

pub fn mt_stop_timer() {
    let start = START_TIMESTAMP.load(Ordering::SeqCst);
    if start > 0 {
        let elapsed = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64 - start;
        MERKLE_TREE_INTERVALS.push(elapsed);
    }
}

pub fn mt_get_time() -> Option<u64> {
    let mut sum = 0;
    while let Some(val) = MERKLE_TREE_INTERVALS.pop() {
        sum += val;
    }
    if sum > 0 { Some(sum) } else { None }
}