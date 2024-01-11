mod utils;
use std::collections::BTreeSet;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(js_name = logHelloWorld)]
pub fn log_hello_world() {
    log("Hello, World!");
}

#[wasm_bindgen(js_name = getPowers)]
pub fn get_powers(n: u32) -> Vec<u32> {
    alert(&format!("Getting powers of {} ...", n));
    vec![n, n.pow(2), n.pow(3)]
}

#[wasm_bindgen(js_name = addTwoNumbers)]
pub fn add_two_numbers(a: u64, b: u64) -> u64 {
    a + b
}

// TODO can this be smaller? probably.
pub struct Point(u32, u32);

///why bother with the square root
#[wasm_bindgen(js_name = euclidianDistance)]
pub fn euclidian_distance(x_1: u32, y_1 : u32, x_2: u32, y_2: u32) -> u32 {
    (x_1 - x_2)^2 + (y_1 - y_2)^2
}

pub enum InternalError {
    /// number of points is not high enough
    NotEnoughPoints,
    /// length of vector is not a multiple of 2
    ImproperData(u32)

}

// TODO better error handling?! wtf.
// max min euclidian distance
#[wasm_bindgen(js_name = hausdorffDistance)]
pub fn hausdorff_distance(p_1: &[u32], p_2: &[u32]) -> u32 {
    // TODO check for empty drawing.
    let mut cur_max = 0;
    for p_1_window in p_1.windows(2) {
        let [x_1, y_1] = p_1_window else {
            panic!("INVARIANT VIOLATED");
        };

        let mut cur_min = core::u32::MAX;
        for p_2_window in p_2.windows(2) {
            let [x_2, y_2] = p_2_window else {
                panic!("INVARIANT VIOLATED");
            };
            let ed = euclidian_distance(*x_1, *y_1, *x_2, *y_2);
            if ed < cur_min {
                cur_min = ed;
            }
        }

        if cur_max < cur_min {
            cur_max = cur_min;
        }

    }
    todo!()
}

#[wasm_bindgen(js_name = bestDistances)]
pub fn best_distances(num_distances: u32, p_1: &[u32]) {
    // todo fill out
    const CHAR_DISTANCES : Vec<(Vec<u32>, &str)> = vec![];
    let top_n_distances = vec![];
    let num_distances = 0;

    for (p_2, name) in CHAR_DISTANCES {
        let distance = hausdorff_distance(p_1, &p_2);
        let result = (distance, name);
        // inline maintain least index (keep the thing sorted? maybe vec bad choice for this)
        // if num_distances < top_n_distances {
        //
        // }


    }

}

