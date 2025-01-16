#![feature(test)]

extern crate test;
use test::Bencher;

use imagequant::*;

#[bench]
fn histogram(b: &mut Bencher) {
    let img = lodepng::decode32_file("/Users/kornel/Desktop/canvas.png").unwrap();
    let liq = Attributes::new();
    b.iter(move || {
        let mut img = liq.new_image(&img.buffer, img.width, img.height, 0.).unwrap();
        let mut hist = Histogram::new(&liq);
        hist.add_image(&mut img).unwrap();
    })
}

#[bench]
fn remap_ord(b: &mut Bencher) {
    let img = lodepng::decode32_file("/Users/kornel/Desktop/canvas.png").unwrap();
    let mut buf = vec![std::mem::MaybeUninit::uninit(); img.width * img.height];
    let mut liq = Attributes::new();
    liq.set_speed(10);
    let mut img = liq.new_image(&img.buffer, img.width, img.height, 0.).unwrap();
    let mut res = liq.quantize(&mut img).unwrap();
    res.set_dithering_level(0.);
    b.iter(move || {
        res.remap_into(&mut img, &mut buf).unwrap();
        res.remap_into(&mut img, &mut buf).unwrap();
    })
}

#[bench]
fn remap_floyd(b: &mut Bencher) {
    let img = lodepng::decode32_file("/Users/kornel/Desktop/canvas.png").unwrap();
    let mut buf = vec![std::mem::MaybeUninit::uninit(); img.width * img.height];
    let mut liq = Attributes::new();
    liq.set_speed(10);
    let mut img = liq.new_image(&img.buffer, img.width, img.height, 0.).unwrap();
    let mut res = liq.quantize(&mut img).unwrap();
    res.set_dithering_level(1.);
    b.iter(move || {
        res.remap_into(&mut img, &mut buf).unwrap();
        res.remap_into(&mut img, &mut buf).unwrap();
    })
}

#[bench]
fn quantize_s8(b: &mut Bencher) {
    let img = lodepng::decode32_file("/Users/kornel/Desktop/canvas.png").unwrap();
    let mut liq = Attributes::new();
    liq.set_speed(8);
    b.iter(move || {
        let mut img = liq.new_image(&img.buffer, img.width, img.height, 0.).unwrap();
        liq.quantize(&mut img).unwrap();
    })
}

#[bench]
fn quantize_s1(b: &mut Bencher) {
    let img = lodepng::decode32_file("/Users/kornel/Desktop/canvas.png").unwrap();
    let mut liq = Attributes::new();
    liq.set_speed(1);
    b.iter(move || {
        let mut img = liq.new_image(&img.buffer, img.width, img.height, 0.).unwrap();
        liq.quantize(&mut img).unwrap();
    })
}
