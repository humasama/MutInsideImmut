use std::sync::atomic::{AtomicUsize, Ordering};

struct Simple {
    count: AtomicUsize
}

impl Simple {
    fn update(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
        println!("cur data: {}", self.count.load(Ordering::Relaxed));
    }
}

fn main() {
    let s = Simple {
        count: AtomicUsize::new(0)
    };

    s.update();
    s.update();
}
