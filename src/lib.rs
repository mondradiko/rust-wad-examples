/*use wasm_bindgen::prelude::*;*/

extern crate gltf;
extern crate lz_fear;

use std::io::{Cursor, Read};
use std::process;

mod mondradiko {
    extern "C" {
        fn mondradiko_load_asset(data: *const u8, size: i32);
    }

    #[inline]
    pub fn load_asset(data: Vec<u8>) {
        unsafe {
            mondradiko_load_asset(data.as_ptr(), data.len() as i32);
        }
    }
}

#[inline]
fn abort_none<T>(o: Option<T>) -> T {
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

#[inline]
fn abort_err<S, E>(r: Result<S, E>) -> S {
    match r {
        Ok(s) => s,
        Err(_) => process::abort(),
    }
}

#[no_mangle]
pub extern "C" fn wad_main() {
    println!("Loading glTF...");

    let model_data = get_model_data();
    let gltf = gltf::Gltf::from_slice(model_data.as_slice());
    process_gltf(abort_err(gltf))
}

#[inline]
fn get_model_data() -> Vec<u8> {
    let compressed_bytes = Cursor::new(include_bytes!("DamagedHelmet.glb.lz4"));
    let frame_reader = lz_fear::LZ4FrameReader::new(compressed_bytes);
    let mut reader = abort_err(frame_reader).into_read();

    let mut model_data = Vec::<u8>::new();
    abort_err(reader.read_to_end(&mut model_data));
    model_data
}

#[inline]
fn process_gltf(model: gltf::Gltf) {
    println!("nodes (#{}):", model.nodes().count());
    for node in model.nodes() {
        match node.name() {
            Some(name) => println!("  {}", name),
            None => println!(" <unnamed node>"),
        }
    }
    println!("End nodes.");

    if let Some(data) = model.blob {
        /*mondradiko::load_asset(data);*/
    } else {
        println!("Couldn't find binary data :'(");
    }
}
