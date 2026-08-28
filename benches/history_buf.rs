use std::hint::black_box;

use divan::counter::BytesCount;
use heapless::HistoryBuf;

const NEEDLE: &[u8] = b"needle451";
const WINDOW_SIZE: usize = NEEDLE.len();

fn write_search_window(data: Vec<u8>) {
    let mut search_window: HistoryBuf<u8, WINDOW_SIZE> = HistoryBuf::new();

    for byte in data {
        search_window.write(byte);
    }
}

#[divan::bench]
fn history_buf_write(bencher: divan::Bencher) {
    let total_bytes = 8 * 1024 * 1024;
    let bytes_before_needle = total_bytes - NEEDLE.len();

    bencher
        .counter(BytesCount::new(total_bytes))
        .with_inputs(|| {
            let mut data = Vec::with_capacity(total_bytes);
            data.resize(bytes_before_needle, 0);
            data.extend(NEEDLE);
            data
        })
        .bench_values(|data| write_search_window(black_box(data)));
}

fn main() {
    divan::main();
}
