use clap::Parser;
use nowasm::{Module, StdVectorFactory, Val};
use orfail::OrFail;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.path).or_fail()?;

    let wasm_bytes = include_bytes!("../wasm/efmt-0.16.0.wasm");
    let module = Module::<StdVectorFactory>::decode(&wasm_bytes[..]).or_fail()?;
    let mut instance = module.instantiate(()).or_fail()?;

    let text_len = text.len() as i32;
    let wasm_text = instance
        .invoke("allocate_vec", &[Val::I32(text_len)])
        .or_fail()?
        .or_fail()?;
    let wasm_text_start = instance
        .invoke("vec_offset", &[wasm_text])
        .or_fail()?
        .or_fail()?
        .as_i32()
        .or_fail()?;
    let wasm_text_end = wasm_text_start + text_len;
    instance.mem_mut()[wasm_text_start as usize..wasm_text_end as usize]
        .copy_from_slice(text.as_bytes());

    let wasm_formatted_text = instance
        .invoke("format", &[Val::I32(wasm_text_start), Val::I32(text_len)])
        .or_fail()?
        .or_fail()?;
    let wasm_formatted_text_start = instance
        .invoke("vec_offset", &[wasm_formatted_text])
        .or_fail()?
        .or_fail()?
        .as_i32()
        .or_fail()? as usize;
    let wasm_formatted_text_len = instance
        .invoke("vec_len", &[wasm_formatted_text])
        .or_fail()?
        .or_fail()?
        .as_i32()
        .or_fail()? as usize;
    let wasm_formatted_text_end = wasm_formatted_text_start + wasm_formatted_text_len;
    let formatted_text =
        std::str::from_utf8(&instance.mem()[wasm_formatted_text_start..wasm_formatted_text_end])
            .or_fail()?;
    print!("{formatted_text}");
    Ok(())
}
