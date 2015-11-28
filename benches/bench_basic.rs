#![feature(test)]
extern crate test;
extern crate minilzo;
use test::Bencher;

#[bench]
fn bench_basic_minilzo_compression_decompression(b: &mut Bencher) {
    b.iter(|| {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

        let compressed = minilzo::compress(lorem.as_bytes()).unwrap();
        let _ = minilzo::decompress(&compressed, lorem.len()).unwrap();
    })
}

#[bench]
fn bench_basic_minilzo_decompression(b: &mut Bencher) {
    b.iter(|| {
        let lorem = [0, 91, 76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 116, 101, 116, 117, 114, 32, 115, 97, 100, 105, 112, 115, 99, 105, 110, 103, 32, 101, 108, 105, 116, 114, 44, 32, 115, 101, 100, 32, 100, 105, 97, 109, 32, 110, 111, 110, 117, 109, 121, 32, 101, 105, 114, 109, 111, 100, 32, 116, 101, 109, 112, 111, 114, 32, 105, 110, 118, 105, 100, 117, 110, 116, 32, 117, 116, 32, 108, 97, 98, 111, 114, 101, 32, 101, 116, 32, 128, 12, 0, 4, 101, 32, 109, 97, 103, 110, 97, 32, 97, 108, 105, 113, 117, 121, 97, 109, 32, 101, 114, 97, 116, 44, 40, 60, 1, 0, 3, 118, 111, 108, 117, 112, 116, 117, 97, 46, 32, 65, 116, 32, 118, 101, 114, 111, 32, 101, 111, 115, 116, 7, 2, 97, 99, 99, 117, 115, 124, 5, 8, 116, 32, 106, 117, 115, 116, 111, 32, 100, 117, 111, 173, 22, 101, 156, 3, 0, 52, 101, 97, 32, 114, 101, 98, 117, 109, 46, 32, 83, 116, 101, 116, 32, 99, 108, 105, 116, 97, 32, 107, 97, 115, 100, 32, 103, 117, 98, 101, 114, 103, 114, 101, 110, 44, 32, 110, 111, 32, 115, 101, 97, 32, 116, 97, 107, 105, 109, 97, 116, 97, 32, 115, 97, 110, 99, 116, 117, 115, 32, 101, 115, 116, 32, 76, 111, 114, 101, 109, 51, 45, 4, 46, 57, 108, 0, 32, 83, 156, 4, 10, 105, 97, 109, 32, 118, 111, 108, 117, 112, 116, 117, 97, 46, 17, 0, 0];

        let _ = minilzo::decompress(&lorem, 451).unwrap();
    })
}

#[bench]
fn bench_basic_minilzo_compression(b: &mut Bencher) {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";
    b.iter(|| {

        let _ = minilzo::compress(lorem.as_bytes()).unwrap();
    })
}
