use std::hint::black_box;

use divan::counter::BytesCount;
use heapless::HistoryBuf;

const WINDOW_SIZE: usize = 9;

fn write_search_window(data: &[u8]) {
    let mut search_window: HistoryBuf<u8, WINDOW_SIZE> = HistoryBuf::new();

    for byte in data.iter().copied() {
        search_window.write(byte);
    }
    black_box(&search_window);
}

#[divan::bench]
fn history_buf_write(bencher: divan::Bencher) {
    let total_bytes = 8 * 1024 * 1024;

    bencher
        .counter(BytesCount::new(total_bytes))
        .with_inputs(|| vec![0u8; total_bytes])
        .bench_refs(|data| write_search_window(black_box(data)));
}

fn main() {
    divan::main();
}
