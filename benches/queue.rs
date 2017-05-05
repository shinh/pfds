#[macro_use]
extern crate bencher;

use bencher::Bencher;

extern crate pfds;

use pfds::queue::*;

fn bench_pf_queue<Queue: PfQueue<i32>>(b: &mut Bencher) {
    let mut q = Queue::new();
    for i in 0..1000 {
        q = q.snoc(i);
    }
    b.iter(|| {
        q.tail().unwrap();
    })
}

fn bench_pf_batched_queue(b: &mut Bencher) {
    bench_pf_queue::<PfBatchedQueue<i32>>(b);
}

benchmark_group!(benches, bench_pf_batched_queue);
benchmark_main!(benches);