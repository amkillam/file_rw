use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use file_rw::{
    preprocess::{
        preprocessor::{Preprocessor, Search},
        CharIndexMatrix, ContinuousHashmap, WindowsHashmap,
    },
    FileReader, FileWriter,
};
use rand;
use tempfile::tempdir;

macro_rules! stress_test_preprocessor_fn {
    ($preprocessor:ident, $preprocessor_type:ident, $file_writer:ident, $data_len:ident, |$byte:ident, $replace_byte:ident, $preprocessor_cache:ident| $block:block) => {
        for $digit in 0..$data_len {
            let mut $preprocess_cache = $file_writer.preprocess_with::<$preprocessor_type>();
            for $byte in 0..0xFF {
                let $replace_byte = 0xFF - $byte;
                $block
            }
        }
    };
}

macro_rules! for_each_preprocessor{
    ($file_writer:ident, $data_len:ident, |$preprocessor:ident, $preprocessor_type:ident| $benchmark_block:block) => {
        let mut $preprocessor = $file_writer.preprocess_with::<CharIndexMatrix>();
        let $preprocessor_type = "CharIndexMatrix";
        $benchmark_block
        let mut $preprocessor = $file_writer.preprocess_with::<ContinuousHashmap>();
        let $preprocessor_type = "ContinuousHashmap";
       $benchmark_block
        let mut $preprocessor = $file_writer.preprocess_with::<WindowsHashmap>();
        let $preprocessor_type = "WindowsHashmap";
       $benchmark_block
        let mut $preprocessor = $file_writer.preprocess();
        let $preprocessor_type = "None";
       $benchmark_block
    };
}

macro_rules! benchmark_with_group{
    ($criterion:ident, $benchmark_block: ident, $benchmark_fn: ident, $size:ident) => {
        let mut benchmark_group = criterion.benchmark_group($benchmark_fn);
        benchmark_group.throughput(Throughput::Bytes(*$size as u64));
        $benchmark_block
        benchmark_group.finish();
    };
    }

macro_rules! for_each_benchmark_fn {
    ($criterion:ident, $size:ident, |$benchmark_fn:ident | $benchmark_block:block) => {
        let $benchmark_fn = "benchmark_find_replace";
        benchmark_with_group!($criterion, $benchmark_block, $benchmark_fn, $size);

        let $benchmark_fn = "benchmark_find_replace_nth";
        benchmark_with_group!($criterion, $benchmark_block, $benchmark_fn, $size);

        let $benchmark_fn = "benchmark_rfind_replace";
        benchmark_with_group!($criterion, $benchmark_block, $benchmark_fn, $size);

        let $benchmark_fn = "benchmark_rfind_replace_nth";
        benchmark_with_group!($criterion, $benchmark_block, $benchmark_fn, $size);

        let $benchmark_fn = "benchmark_find_replace_all";
        benchmark_with_group!($criterion, $benchmark_block, $benchmark_fn, $size);
    };
}

fn gen__dataset(bytes: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(bytes);
    for _ in 0..1024 * 1024 * 10 {
        data.push(rng.gen::<u8>());
    }
    data
}

fn benchmark_find_replace_nth<T: Search>(
    preprocessor: T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        preprocessor,
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocess_cache| {
            file_writer.find_replace_nth(
                black_box(byte),
                black_box(replace_byte),
                black_box(digit),
                &mut preprocess_cache,
            );
        }
    );
}

fn benchmark_find_replace<T: Search>(
    preprocessor: T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        preprocessor,
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocess_cache| {
            file_writer.find_replace(
                black_box(byte),
                black_box(replace_byte),
                &mut preprocess_cache,
            );
        }
    );
}

fn benchmark_rfind_replace_nth<T: Search>(
    preprocessor: T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        preprocessor,
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocess_cache| {
            file_writer.rfind_replace_nth(
                black_box(byte),
                black_box(replace_byte),
                black_box(digit),
                &mut preprocess_cache,
            );
        }
    );
}

fn benchmark_rfind_replace<T: Search>(
    preprocessor: T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        preprocessor,
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocess_cache| {
            file_writer.rfind_replace(
                black_box(byte),
                black_box(replace_byte),
                &mut preprocess_cache,
            );
        }
    );
}

fn benchmark_find_replace_all<T: Search>(
    preprocessor: T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        preprocessor,
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocess_cache| {
            file_writer.find_replace_all(
                black_box(byte),
                black_box(replace_byte),
                &mut preprocess_cache,
            );
        }
    );
}

fn benchmark_preprocessors() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let test_file_path = tempdir_path.join("test_data");

    let mut file_writer = FileWriter::open(&test_file_path);

    const KB: usize = 1024;
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        let data = gen__dataset(size);
        file_writer.overwrite(&data);
        for_each_benchmark_fn!(criterion, size, |benchmark_fn| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type| {
                criterion.bench_with_input(
                    BenchmarkId::new(preprocessor_type, size),
                    &size,
                    |b, &s| {
                        b.iter(|| {
                            benchmark_fn::<preprocessor_type>(
                                black_box(preprocessor),
                                black_box(&mut file_writer),
                                black_box(s),
                            )
                        });
                    },
                );
                file_writer.overwrite(&data);
            });
        });
    }
}

criterion_group!(benches, benchmark_preprocessors);
criterion_main!(benches);
