use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use file_rw::{
    preprocess::{
        preprocessor::Search,
        CharIndexMatrix, ContinuousHashmap, WindowsHashmap,
    },
    FileWriter,
};
use rand::Rng;
use tempfile::tempdir;

macro_rules! stress_test_preprocessor_fn {
    ($preprocessor_type:ident, $file_writer:ident, $find_replace_n_triplets:ident, |$find_byte:ident, $replace_byte:ident, $n:ident, $preprocessor_cache:ident| $block:block) => {
        let mut $preprocessor_cache = $file_writer.preprocess_with::<$preprocessor_type>();
        $find_replace_n_triplets.iter().for_each(|(find_vec, replace_vec, n)| {
            let $replace_byte = replace_vec.as_slice();
            let $find_byte = find_vec.as_slice();
            let $n = *n;
            println!("{:?}, {:?}, {:?}", $find_byte, $replace_byte, $n);
            $block
        });
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
        let $preprocessor_type_str = "Default(ContinuousHashmap)";
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

fn gen_dataset(num_bytes: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(num_bytes);
    for _ in 0..num_bytes {
        data.push(rng.gen::<u8>());
    }
    data
}

fn gen_find_replace_n_triplets(num_bytes:usize) -> Vec<(Vec<u8>, Vec<u8>, usize)> {
    let mut rng = rand::thread_rng();
    let triplets_container = (1..num_bytes).into_iter().map(|_| {
        let find_replace_range = rng.gen_range(1u8..10u8);
        let find_vec = (1u8..find_replace_range).into_iter().map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
        let replace_vec = (1u8..find_replace_range).into_iter().map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
    let n = rng.gen_range(1..10);
    (find_vec, replace_vec, n)
    }).collect::<Vec<(Vec<u8>, Vec<u8>, usize)>>();
    triplets_container
}

fn benchmark_find_replace_nth<T: Search>(
    //Allows identification of type passed in macro - type cannot be passed literally in macro as generic
    _preprocessor: &mut T,
    file_writer: &mut FileWriter,
    find_replace_n_triplets: &Vec<(Vec<u8>, Vec<u8>, usize)>,
) {
    stress_test_preprocessor_fn!(T, file_writer, find_replace_n_triplets,|find_byte, replace_byte, n, preprocessor_cache| {
        file_writer.find_replace_nth(
            black_box(find_byte),
            black_box(replace_byte),
            black_box(n),
            &mut preprocessor_cache,
        );
    });
}

fn benchmark_find_replace<T: Search>(
    //Allows identification of type passed in macro - type cannot be passed literally in macro as generic
    _preprocessor: &mut T,
    file_writer: &mut FileWriter,
    find_replace_n_triplets: &Vec<(Vec<u8>, Vec<u8>, usize)>,
) {
    stress_test_preprocessor_fn!(T, file_writer, find_replace_n_triplets,|find_byte, replace_byte, _n, preprocessor_cache| {
        file_writer.find_replace(
            black_box(find_byte),
            black_box(replace_byte),
            &mut preprocessor_cache,
        );
    });
}

fn benchmark_rfind_replace_nth<T: Search>(
    //Allows identification of type passed in macro - type cannot be passed literally in macro as generic
    _preprocessor: &mut T,
    file_writer: &mut FileWriter,
    find_replace_n_triplets: &Vec<(Vec<u8>, Vec<u8>, usize)>,
) {
    stress_test_preprocessor_fn!(T, file_writer, find_replace_n_triplets,|find_byte, replace_byte, n, preprocessor_cache| {
        file_writer.rfind_replace_nth(
            black_box(find_byte),
            black_box(replace_byte),
            black_box(n),
            &mut preprocessor_cache,
        );
    });
}

fn benchmark_rfind_replace<T: Search>(
    //Allows identification of type passed in macro - type cannot be passed literally in macro as generic
    _preprocessor: &mut T,
    file_writer: &mut FileWriter,
    find_replace_n_triplets: &Vec<(Vec<u8>, Vec<u8>, usize)>,
) {
    stress_test_preprocessor_fn!(T, file_writer, find_replace_n_triplets,|find_byte, replace_byte, _n, preprocessor_cache| {
        file_writer.rfind_replace(
            black_box(find_byte),
            black_box(replace_byte),
            &mut preprocessor_cache,
        );
    });
}

fn benchmark_find_replace_all<T: Search>(
    //Allows identification of type passed in macro - type cannot be passed literally in macro as generic
    _preprocessor: &mut T,
    file_writer: &mut FileWriter,
    find_replace_n_triplets: &Vec<(Vec<u8>, Vec<u8>, usize)>,
) {
    stress_test_preprocessor_fn!(T, file_writer, find_replace_n_triplets,|find_byte, replace_byte, _n, preprocessor_cache| {
        file_writer.find_replace_all(
            black_box(find_byte),
            black_box(replace_byte),
            &mut preprocessor_cache,
        );
    });
}

fn benchmark_preprocessors(criterion: &mut Criterion) {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let test_file_path = tempdir_path.join("test_data");

    let mut file_writer = FileWriter::open(&test_file_path);

    const KB: usize = 1024;
    const MB : usize = 1024 * KB;
    const GB : usize = 1024 * MB;
    for size in [KB, MB, GB].iter() {
        let data = gen_dataset(*size);
        file_writer.overwrite(&data);
        let find_replace_n_triplets = gen_find_replace_n_triplets(*size);
        benchmark_with_group!(criterion, "find_replace_nth", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &_s| {
                        b.iter(|| {
                            benchmark_find_replace_nth(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(&find_replace_n_triplets),
                            );
                        });
                    },
                );
                file_writer.overwrite(&data);
            });
            benchmark_group.finish();
        });
        benchmark_with_group!(criterion, "find_replace", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &_s| {
                        b.iter(|| {
                            benchmark_find_replace(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(&find_replace_n_triplets),
                            );
                        });
                    },
                );
                file_writer.overwrite(&data);
            });
            benchmark_group.finish();
        });
        benchmark_with_group!(criterion, "rfind_replace_nth", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &_s| {
                        b.iter(|| {
                            benchmark_rfind_replace_nth(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(&find_replace_n_triplets),
                            );
                        });
                    },
                );
                file_writer.overwrite(&data);
            });
            benchmark_group.finish();
        });
        benchmark_with_group!(criterion, "rfind_replace", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &_s| {
                        b.iter(|| {
                            benchmark_rfind_replace(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(&find_replace_n_triplets),
                            );
                        });
                    },
                );
                file_writer.overwrite(&data);
            });
            benchmark_group.finish();
        });

        benchmark_with_group!(criterion, "find_replace_all", size, |benchmark_group| {
            for_each_preprocessor!(file_writer, |preprocessor, preprocessor_type_str| {
                benchmark_group.bench_with_input(
                    BenchmarkId::new(preprocessor_type_str, size),
                    &size,
                    |b, &_s| {
                        b.iter(|| {
                            benchmark_find_replace_all(
                                black_box(&mut preprocessor),
                                black_box(&mut file_writer),
                                black_box(&find_replace_n_triplets),
                            );
                        });
                    },
                );
                file_writer.overwrite(&data);
            });
            benchmark_group.finish();
        });
    }
}

criterion_group!(benches, benchmark_preprocessors);
criterion_main!(benches);
