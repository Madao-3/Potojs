#![allow(unused)]
#![allow(illegal_floating_point_literal_pattern)]

extern crate js_sys;
extern crate wasm_bindgen;

use js_sys::Math;
// use rand::Rng;
use std::f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_name = Math)]
    static math: Math;
}

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

fn random_range(min: f32, max: f32) -> f32 {
    // let mut rng = StdRng::from_entropy();
    let rand = Math::random() as f32;
    (min + (rand * (max - min))).round()
}

fn trauncate(number: f32) -> f32 {
    if number > 255f32 {
        return 255f32;
    } else if number < 0f32 {
        return 0f32;
    }
    number.floor()
}

//  0 < α < 1
//  f(x)= α(x−128)+128+b
fn contrast_adjust(value: f32, contrast: f32) -> f32 {
    let factor = (255f32 * (contrast + 255f32)) / (255f32 * (255f32 - contrast));
    let result = factor * (value - 128f32) + 128f32;
    trauncate(result)
}

fn f32_max(arr: &Vec<f32>) -> f32 {
    let mut max: &f32 = &f32::MIN;
    for number in arr {
        if number > &max {
            max = number;
        }
    }
    max.to_owned()
}

fn f32_min(arr: &Vec<f32>) -> f32 {
    let mut min: &f32 = &f32::MAX;
    for number in arr {
        if number < &min {
            min = number;
        }
    }
    min.to_owned()
}

fn rgb_to_hsv((mut r, mut g, mut b): (f32, f32, f32)) -> (f32, f32, f32) {
    r /= 255f32;
    g /= 255f32;
    b /= 255f32;
    let max = f32_max(&vec![r, g, b]);
    let min = f32_min(&vec![r, g, b]);
    let v = max;
    let d = max - min;
    let s = if max == 0f32 { 0f32 } else { d / max };
    let h = if max == min {
        0f32
    } else {
        (match max {
            r => ((g - b) / d) + (if g < b { 6f32 } else { 0f32 }),
            g => ((b - r) / d) + 2f32,
            b => ((r - g) / d) + 4f32,
        }) / 6f32
    };
    (h, s, v)
}

fn hsv_to_rgb((h, s, v): (f32, f32, f32)) -> (f32, f32, f32) {
    let i = (h * 6f32).floor();
    let f = (h * 6f32) - i;
    let p = v * (1f32 - s);
    let q = v * (1f32 - f * s);
    let t = v * (1f32 - (1f32 - f) * s);
    let mut r: f32;
    let mut g: f32;
    let mut b: f32;
    match i % 6f32 {
        0f32 => {
            r = v;
            g = t;
            b = p
        }
        1f32 => {
            r = q;
            g = v;
            b = p
        }
        2f32 => {
            r = p;
            g = v;
            b = t
        }
        3f32 => {
            r = p;
            g = q;
            b = v
        }
        4f32 => {
            r = t;
            g = p;
            b = v
        }
        5f32 => {
            r = v;
            g = p;
            b = q
        }
        _ => {
            r = v;
            g = t;
            b = p
        }
    };

    (
        (r * 255f32).floor(),
        (g * 255f32).floor(),
        (b * 255f32).floor(),
    )
}

// fn bezier(start: f32, c1: f32, c2: f32, end: f32) {
//     let control_points = vec![start, c1, c2, end];
//     let lerp = |a: f32, b: f32, t: f32| a * (1f32 - t) + b * t;
//     let clamp = |a: f32, min: f32, max: f32| min_f32(max_f32(a, min), max);
// }

#[wasm_bindgen]
pub fn brightness(mut arr: Vec<f32>, adjustment: f32) -> Vec<f32> {
    let mut index = 0usize;
    let count = arr.len();
    if adjustment == 0f32 {
        return arr;
    }
    loop {
        if index >= count {
            break;
        }
        arr[index] = trauncate(arr[index] + adjustment);
        arr[index + 1] = trauncate(arr[index + 1] + adjustment);
        arr[index + 2] = trauncate(arr[index + 2] + adjustment);
        index += 4;
    }
    arr
}

#[wasm_bindgen]
pub fn grayscale(mut arr: Vec<f32>, is_open: bool) -> Vec<f32> {
    let mut i = 0usize;
    let count = arr.len();
    if !is_open {
        return arr;
    }
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];
        let v = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        arr[i] = v;
        arr[i + 1] = v;
        arr[i + 2] = v;
        i += 4;
    }
    arr
}

// threshold
#[wasm_bindgen]
pub fn threshold(mut arr: Vec<f32>, adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }
    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];
        let v = if 0.2126 * r + 0.7152 * g + 0.0722 * b >= adjustment {
            255f32
        } else {
            0f32
        };
        arr[i] = v;
        arr[i + 1] = v;
        arr[i + 2] = v;
        i += 4;
    }
    arr
}
// contrast
#[wasm_bindgen]
pub fn contrast(mut arr: Vec<f32>, adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }

    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        arr[i] = contrast_adjust(arr[i], adjustment);
        arr[i + 1] = contrast_adjust(arr[i + 1], adjustment);
        arr[i + 2] = contrast_adjust(arr[i + 2], adjustment);
        i += 4;
    }
    arr
}

// saturation
//   adjust *= -0.01
//   @process "saturation", (rgba) ->
//     max = Math.max r, g, b
//     r += (max - r) * adjust if r isnt max
//     g += (max - g) * adjust if g isnt max
//     b += (max - b) * adjust if b isnt max
//     rgba
#[wasm_bindgen]
pub fn saturation(mut arr: Vec<f32>, mut adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }
    adjustment *= -0.01;
    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];
        let max = f32_max(&vec![r, g, b]);
        if r != max {
            arr[i] += (max - r) * adjustment;
        }
        if g != max {
            arr[i + 1] += (max - g) * adjustment;
        }
        if b != max {
            arr[i + 2] += (max - b) * adjustment;
        }
        i += 4;
    }
    arr
}
// Hue
// adjustment cannot less than zero
// BUGGY
#[wasm_bindgen]
pub fn hue(mut arr: Vec<f32>, adjustment: f32) -> Vec<f32> {
    if adjustment <= 0f32 {
        return arr;
    }
    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];

        let (mut h, s, v) = rgb_to_hsv((r, g, b));

        h = h * 100f32;
        h += adjustment;
        h = h % 100f32;
        h /= 100f32;
        h = h;

        let result = hsv_to_rgb((h, s, v));
        arr[i] = result.0;
        arr[i + 1] = result.1;
        arr[i + 2] = result.2;
        i += 4;
    }
    arr
}

// seipa
#[wasm_bindgen]
pub fn sepia(mut arr: Vec<f32>, mut adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }
    adjustment /= 100f32;
    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];
        arr[i] = f32_min(&vec![
            255f32,
            (r * (1f32 - (0.607 * adjustment)))
                + (g * (0.769 * adjustment))
                + (b * (0.189 * adjustment)),
        ]);
        arr[i + 1] = f32_min(&vec![
            255f32,
            (r * (0.349 * adjustment))
                + (g * (1f32 - (0.314 * adjustment)))
                + (b * (0.168 * adjustment)),
        ]);
        arr[i + 2] = f32_min(&vec![
            255f32,
            (r * (0.272 * adjustment))
                + (g * (0.534 * adjustment))
                + (b * (1f32 - (0.869 * adjustment))),
        ]);

        i += 4;
    }
    arr
}

// gamma
#[wasm_bindgen]
pub fn gamma(mut arr: Vec<f32>, mut adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }
    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];
        arr[i] = (r / 255f32).powf(adjustment) * 255f32;
        arr[i + 1] = (g / 255f32).powf(adjustment) * 255f32;
        arr[i + 2] = (b / 255f32).powf(adjustment) * 255f32;
        i += 4;
    }
    arr
}
// noise
#[wasm_bindgen]
pub fn noise(mut arr: Vec<f32>, mut adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }
    let mut i = 0usize;
    let count = arr.len();
    adjustment = adjustment.abs() * 2.55;
    loop {
        if i >= count {
            break;
        }
        let rand = random_range(-1f32 * adjustment, adjustment);
        arr[i] += rand;
        arr[i + 1] += rand;
        arr[i + 2] += rand;
        i += 4;
    }
    arr
}

// vibrance
#[wasm_bindgen]
pub fn vibrance(mut arr: Vec<f32>, mut adjustment: f32) -> Vec<f32> {
    if adjustment == 0f32 {
        return arr;
    }
    let mut i = 0usize;
    let count = arr.len();
    adjustment *= -1f32;
    loop {
        if i >= count {
            break;
        }
        let r = arr[i];
        let g = arr[i + 1];
        let b = arr[i + 2];
        let max = f32_max(&vec![r, g, b]);
        let avg = (r + g + b) / 3f32;
        let amt = (((max - avg).abs() * 2f32 / 255f32) * adjustment) / 100f32;

        if r != max {
            arr[i] += (max - r) * amt
        };
        if g != max {
            arr[i + 1] += (max - g) * amt
        };
        if b != max {
            arr[i + 2] += (max - b) * amt
        };
        i += 4;
    }
    arr
}

#[wasm_bindgen]
pub fn invert(mut arr: Vec<f32>, is_invert: bool) -> Vec<f32> {
    if !is_invert {
        return arr;
    }
    let mut i = 0usize;
    let count = arr.len();
    loop {
        if i >= count {
            break;
        }
        arr[i] = 255f32 - arr[i];
        arr[i + 1] = 255f32 - arr[i + 1];
        arr[i + 2] = 255f32 - arr[i + 2];
        i += 4;
    }
    arr
}

// stack blur
// sharpen
