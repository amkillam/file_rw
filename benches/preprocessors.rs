use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use file_rw::{
    preprocess::{
        preprocessor::{Preprocessor, Search},
        CharIndexMatrix, ContinuousHashmap, WindowsHashmap,
    },
    FileReader, FileWriter,
};
use rand::Rng;
use tempfile::tempdir;

macro_rules! stress_test_preprocessor_fn {
    ($preprocessor_type:ident, $file_writer:ident, $data_len:ident, |$byte:ident, $replace_byte:ident, $preprocessor_cache:ident, $digit:ident| $block:block) => {
        for $digit in 0..$data_len {
            let mut $preprocessor_cache = $file_writer.preprocess_with::<$preprocessor_type>();
            for int_byte in 0u8..0xFFu8 {
                let int_replace_byte = 0xFF - int_byte;
                let $replace_byte = int_replace_byte.to_be_bytes();
                let $byte = int_byte.to_be_bytes();
                $block
            }
        }
    };
}

macro_rules! for_each_preprocessor{
    ($file_writer:ident, |$preprocessor:ident, $preprocessor_type_str:ident| $benchmark_block:block) => {
        let mut $preprocessor = $file_writer.preprocess_with::<CharIndexMatrix>();
        let $preprocessor_type_str = "CharIndexMatrix";

        $benchmark_block
        let mut $preprocessor = $file_writer.preprocess_with::<ContinuousHashmap>();
        let $preprocessor_type_str = "ContinuousHashmap";

       $benchmark_block
        let mut $preprocessor = $file_writer.preprocess_with::<WindowsHashmap>();
        let $preprocessor_type_str = "WindowsHashmap";

       $benchmark_block
        let mut $preprocessor = $file_writer.preprocess();
        let $preprocessor_type_str = "ContinuousHashmap";

       $benchmark_block
    };
}

macro_rules! benchmark_with_group {
    ($criterion:ident, $benchmark_fn_str: expr, $size:ident, |$benchmark_group:ident| $benchmark_block: block) => {
        let mut $benchmark_group = $criterion.benchmark_group($benchmark_fn_str);
        $benchmark_group.throughput(Throughput::Bytes(*$size as u64));
        $benchmark_block
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
    preprocessor: &mut T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocessor_cache, digit| {
            file_writer.find_replace_nth(
                black_box(byte),
                black_box(replace_byte),
                black_box(digit),
                &mut preprocessor_cache,
            );
        }
    );
}

fn benchmark_find_replace<T: Search>(
    preprocessor: &mut T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocessor_cache, digit| {
            file_writer.find_replace(
                black_box(byte),
                black_box(replace_byte),
                &mut preprocessor_cache,
            );
        }
    );
}

fn benchmark_rfind_replace_nth<T: Search>(
    preprocessor: &mut T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocessor_cache, digit| {
            file_writer.rfind_replace_nth(
                black_box(byte),
                black_box(replace_byte),
                black_box(digit),
                &mut preprocessor_cache,
            );
        }
    );
}

fn benchmark_rfind_replace<T: Search>(
    preprocessor: &mut T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocessor_cache, digit| {
            file_writer.rfind_replace(
                black_box(byte),
                black_box(replace_byte),
                &mut preprocessor_cache,
            );
        }
    );
}

fn benchmark_find_replace_all<T: Search>(
    preprocessor: &mut T,
    file_writer: &mut FileWriter,
    data_len: usize,
) {
    stress_test_preprocessor_fn!(
        T,
        file_writer,
        data_len,
        |byte, replace_byte, preprocessor_cache, digit| {
            file_writer.find_replace_all(
                black_box(byte),
                black_box(replace_byte),
                &mut preprocessor_cache,
            );
        }
    );
}

fn benchmark_preprocessors(criterion: &mut Criterion) {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let test_file_path = tempdir_path.join("test_data");

    let mut file_writer = FileWriter::open(&test_file_path);

    const KB: usize = 1024;
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        let data = gen__dataset(*size);
        file_writer.overwrite(&data);
        benchmark_with_group!(criterion, "find_replace_nth", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &s| {
                        b.iter(|| {
                            benchmark_find_replace_nth(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(*s),
                            );
                        });
                        file_writer.overwrite(&data);
                    },
                );
            });
            benchmark_group.finish();
        });
        benchmark_with_group!(criterion, "find_replace", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &s| {
                        b.iter(|| {
                            benchmark_find_replace(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(*s),
                            );
                        });
                        file_writer.overwrite(&data);
                    },
                );
            });
            benchmark_group.finish();
        });
        benchmark_with_group!(criterion, "rfind_replace_nth", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &s| {
                        b.iter(|| {
                            benchmark_rfind_replace_nth(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(*s),
                            );
                        });
                        file_writer.overwrite(&data);
                    },
                );
            });
            benchmark_group.finish();
        });
        benchmark_with_group!(criterion, "rfind_replace", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &s| {
                        b.iter(|| {
                            benchmark_rfind_replace(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(*s),
                            );
                        });
                        file_writer.overwrite(&data);
                    },
                );
            });
            benchmark_group.finish();
        });

        benchmark_with_group!(criterion, "find_replace_all", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &s| {
                        b.iter(|| {
                            benchmark_find_replace_all(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(*s),
                            );
                        });
                        file_writer.overwrite(&data);
                    },
                );
            });
            benchmark_group.finish();
        });
    }
}

criterion_group!(benches, benchmark_preprocessors);
criterion_main!(benches);
