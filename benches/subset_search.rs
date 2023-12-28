use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use file_rw::{FileReader, FileWriter};
use rand::Rng;
use tempfile::tempdir;

macro_rules! benchmark_with_group {
    ($criterion:ident, $benchmark_fn_str: expr, $total_throughput_size:ident, |$benchmark_group:ident| $benchmark_block: block) => {
        let mut $benchmark_group = $criterion.benchmark_group($benchmark_fn_str);
        $benchmark_group.throughput(Throughput::Bytes($total_throughput_size as u64));
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

fn gen_find_bytes_replace_n_triplets(num_searches: usize) -> Vec<(Vec<u8>, Vec<u8>, usize)> {
    let mut rng = rand::thread_rng();
    let vals_container = (1..num_searches)
        .into_iter()
        .map(|_| {
            let find_bytes_range = rng.gen_range(1u8..10u8);
            let find_bytes_vec = (1u8..find_bytes_range)
                .into_iter()
                .map(|_| rng.gen::<u8>())
                .collect::<Vec<u8>>();
            let replace_vec = (1u8..find_bytes_range)
                .into_iter()
                .map(|_| rng.gen::<u8>())
                .collect::<Vec<u8>>();

            let n = rng.gen_range(0..find_bytes_range as usize);
            (find_bytes_vec, replace_vec, n)
        })
        .collect::<Vec<(Vec<u8>, Vec<u8>, usize)>>();
    vals_container
}

const KB: usize = 1024;
const MB: usize = 1024 * KB;
const HALF_GB: usize = 512 * MB;
const GB: usize = 2 * HALF_GB;
fn benchmark_subset_search(c: &mut Criterion) {
    let num_searches_arr = [KB, MB, HALF_GB, GB];
    let num_bytes_arr = [100, 1000, 10000, 100000];
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let test_file_path = tempdir_path.join("test_file");
    let mut file_writer = FileWriter::open(&test_file_path);

    num_searches_arr.iter().for_each(|num_searches| {
        num_bytes_arr.iter().for_each(|num_bytes| {
            let dataset = gen_dataset(*num_bytes);
            let find_bytes_replace_n_triplets = gen_find_bytes_replace_n_triplets(*num_searches);
            let total_throughput_size = num_searches * num_bytes;
            file_writer.overwrite(&dataset);
            let file_reader = FileReader::open(&test_file_path);

            benchmark_with_group!(
                c,
                "benchmark_find_bytes_all",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, _replace_val, _n)| {
                                        file_reader.find_bytes_all(find_bytes_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_rfind_bytes_all",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, _replace_val, _n)| {
                                        file_reader.rfind_bytes_all(find_bytes_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_find",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, _replace_val, _n)| {
                                        file_reader.find_bytes(find_bytes_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_rfind",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, _replace_val, _n)| {
                                        file_reader.rfind_bytes(find_bytes_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );
            benchmark_with_group!(
                c,
                "benchmark_find_bytes_nth",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, _replace_val, n)| {
                                        file_reader.find_bytes_nth(find_bytes_val, *n);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_rfind_bytes_nth",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, _replace_val, n)| {
                                        file_reader.rfind_bytes_nth(find_bytes_val, *n);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_find_bytes_replace",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, replace_val, _n)| {
                                        file_writer.find_replace(find_bytes_val, replace_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_rfind_bytes_replace",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, replace_val, _n)| {
                                        file_writer.rfind_replace(find_bytes_val, replace_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_find_bytes_replace_nth",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, replace_val, n)| {
                                        file_writer.find_replace_nth(
                                            find_bytes_val,
                                            replace_val,
                                            *n,
                                        );
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_rfind_bytes_replace_nth",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, replace_val, n)| {
                                        file_writer.rfind_replace_nth(
                                            find_bytes_val,
                                            replace_val,
                                            *n,
                                        );
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );

            benchmark_with_group!(
                c,
                "benchmark_find_bytes_replace_all",
                total_throughput_size,
                |benchmark_group| {
                    benchmark_group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{}-{}", num_searches, num_bytes)),
                        &dataset,
                        |b, _dataset| {
                            b.iter(|| {
                                find_bytes_replace_n_triplets.iter().for_each(
                                    |(find_bytes_val, replace_val, _n)| {
                                        file_writer.find_replace_all(find_bytes_val, replace_val);
                                    },
                                );
                            });
                        },
                    );
                    benchmark_group.finish();
                }
            );
        });
    });
}

criterion_group!(benches, benchmark_subset_search);
criterion_main!(benches);
