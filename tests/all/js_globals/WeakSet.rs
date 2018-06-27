#![allow(non_snake_case)]

use project;

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_weak_set() -> js::WeakSet {
                js::WeakSet::new()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_weak_set(), "object");
            }
        "#)
        .test()
}

#[test]
fn has() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has_value(this: &js::WeakSet, value: js::Object) -> bool {
                this.has(value)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new WeakSet();
                let value = {some: "value"};
                set.add(value);
                assert.equal(wasm.has_value(set, value), true);

                let nonex = {nonexistent: "value"};
                assert.equal(wasm.has_value(set, nonex), false);
            }
        "#)
        .test()
}

#[test]
fn add() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn add_value(this: &js::WeakSet, value: js::Object) -> js::WeakSet {
                this.add(value)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new WeakSet();
                let value = {some: "value"};
                wasm.add_value(set, value);
                assert.equal(set.has(value), true);

                assert.throws(() => { wasm.add_value(set, 1) }, TypeError);
                assert.throws(() => { wasm.add_value(set, true) }, TypeError);
                assert.throws(() => { wasm.add_value(set, "fail") }, TypeError);
                assert.throws(() => { wasm.add_value(set, null) }, TypeError);
                assert.throws(() => { wasm.add_value(set, undefined) }, TypeError);
            }
        "#)
        .test()
}