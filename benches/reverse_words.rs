use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn reverse_words(mut s: String) -> String {
    unsafe {
        fn rev(b: &mut [u8]) {
            let mut start = 0;
            let mut end = b.len() - 1;
            while start < end {
                unsafe {
                    let tmp = *b.get_unchecked(start);
                    *b.get_unchecked_mut(start) = *b.get_unchecked(end);
                    *b.get_unchecked_mut(end) = tmp;
                }
                start += 1;
                end -= 1;
            }
        }

        let n = s.len();
        let p: &mut [u8] = s.as_bytes_mut();

        let mut start = 0;
        for i in 1..n {
            if *p.get_unchecked(i) == b' ' {
                rev(p.get_unchecked_mut(start..i));
                start = i + 1;
            }
        }
        rev(&mut p.get_unchecked_mut(start..n));

        s
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let text = std::fs::read_to_string("lorem.txt").unwrap();
    c.bench_with_input(BenchmarkId::new("reverse", ""), &text, |b, i| {
        b.iter_batched(
            || i.clone(),
            |i| reverse_words(i),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
