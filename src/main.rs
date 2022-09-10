use std::{sync::atomic::{AtomicUsize, Ordering}, cell::UnsafeCell};

struct Simple {
    count: AtomicUsize
}

impl Simple {
    fn update(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
        println!("cur data: {}", self.count.load(Ordering::Relaxed));
    }
}

struct Data {
    d: UnsafeCell<usize>
}

impl Data {
    fn new(t: usize) -> Self {
        Data {
            d: UnsafeCell::new(t)
        }
    }

    fn update(&self) {
        let x = self.d.get();
        unsafe {
            (*x) += 1;
        }
    }
}

fn main() {
    let s = Simple {
        count: AtomicUsize::new(0)
    };

    s.update();
    s.update();

    let mut data = Data::new(0);
    println!("Init Data = {}", data.d.get_mut());

    // For UnsafeCell, borrowing check is during runtime, rather than at compile stage.
    // If it checks at compile stage, rd1 and rd2 cannot exist at the same time.
    let rd1 = &mut data;
    rd1.update();
    let rd2 = &mut data;
    rd2.update();
    println!("After Data = {}", data.d.get_mut());
}
