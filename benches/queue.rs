#[macro_use]
extern crate bencher;

use bencher::Bencher;

extern crate pfds;

use pfds::queue::*;

fn bench_pf_queue_snoc<Queue: PfQueue<i32>>(b: &mut Bencher) {
    b.iter(|| {
        let mut q = Queue::new();
        for i in 0..1000 {
            q = q.snoc(i);
        }
    })
}

fn bench_pf_queue_tail<Queue: PfQueue<i32>>(b: &mut Bencher) {
    let mut q_full = Queue::new();
    for i in 0..1000 {
        q_full = q_full.snoc(i);
    }
    b.iter(|| {
        let mut q = q_full.clone();
        loop {
            match q.tail() {
                Ok(nq) => q = nq,
                Err(_) => break
            }
        }
    })
}

fn bench_pf_queue_kill_batched<Queue: PfQueue<i32>>(b: &mut Bencher) {
    let mut q = Queue::new();
    for i in 0..1000 {
        q = q.snoc(i);
    }
    b.iter(|| {
        q.tail().unwrap();
    })
}

fn bench_pf_batched_queue_snoc(b: &mut Bencher) {
    bench_pf_queue_snoc::<PfBatchedQueue<i32>>(b);
}

fn bench_pf_banker_queue_snoc(b: &mut Bencher) {
    bench_pf_queue_snoc::<PfBankerQueue<i32>>(b);
}

fn bench_pf_batched_queue_tail(b: &mut Bencher) {
    bench_pf_queue_tail::<PfBatchedQueue<i32>>(b);
}

fn bench_pf_banker_queue_tail(b: &mut Bencher) {
    bench_pf_queue_tail::<PfBankerQueue<i32>>(b);
}

fn bench_pf_batched_queue_kill_batched(b: &mut Bencher) {
    bench_pf_queue_kill_batched::<PfBatchedQueue<i32>>(b);
}

fn bench_pf_banker_queue_kill_batched(b: &mut Bencher) {
    bench_pf_queue_kill_batched::<PfBankerQueue<i32>>(b);
}

benchmark_group!(benches,
                 bench_pf_batched_queue_snoc,
                 bench_pf_banker_queue_snoc,
                 bench_pf_batched_queue_tail,
                 bench_pf_banker_queue_tail,
                 bench_pf_batched_queue_kill_batched,
                 bench_pf_banker_queue_kill_batched
);
benchmark_main!(benches);
