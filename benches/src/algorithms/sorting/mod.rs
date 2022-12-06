use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;
use wc::algorithms::sorting::*;

static CAPACITY: usize = 10_000;

fn values() -> Vec<u32> {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    (0..CAPACITY).map(|_| rng.next_u32()).collect()
}

macro_rules! sorting_benchmark (
    ($fn_name: ident, $T: ty) => {
        fn $fn_name(bh: &mut criterion::Criterion) {
            bh.bench_function(stringify!($fn_name), move |bh| bh.iter(|| {
                let mut values = values();
                <$T>::sort(&mut values);
            }));
        }
    }
);

sorting_benchmark!(bubble_sort, BubbleSort);
sorting_benchmark!(quick_sort, QuickSort);
sorting_benchmark!(insertion_sort, InsertionSort);

criterion_group!(
    name = bench;
    config = crate::default_config();
    targets = bubble_sort, insertion_sort, quick_sort
);
