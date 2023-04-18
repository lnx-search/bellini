mod core;
mod serde_compat;

use std::collections::BTreeMap;
use std::hint::black_box;
use std::mem::size_of;
use std::time::Instant;

use crate::core::Document;

macro_rules! pretty {
    ($val:expr) => {{
        humansize::format_size($val, humansize::DECIMAL)
    }};
}

fn main() -> anyhow::Result<()> {
    dbg!(size_of::<(u16, u16)>());
    dbg!(size_of::<tantivy::Document>());

    let data = std::fs::read("./movies.json")?;

    let start = Instant::now();
    let compressed = black_box(lz4_flex::compress(&data));
    println!(
        "Serde compress took {:?} total size: {}",
        start.elapsed(),
        pretty!(compressed.len())
    );
    let start = Instant::now();
    let decompressed = black_box(lz4_flex::decompress(&compressed, data.len()))?;
    println!(
        "Serde decompress took {:?} total size: {}",
        start.elapsed(),
        pretty!(decompressed.len())
    );

    let start = Instant::now();
    let values = serde_json::from_slice::<Vec<BTreeMap<String, serde_json::Value>>>(&data)?;
    println!(
        "Serde took {:?} total size: {}",
        start.elapsed(),
        pretty!(data.len())
    );

    let start = Instant::now();
    let rkyv_docs = serde_json::from_slice::<Vec<Document>>(&data)?;

    println!(
        "Serde took {:?} total size: {}",
        start.elapsed(),
        pretty!(data.len())
    );

    let values2 = serde_json::from_slice::<Vec<BTreeMap<String, serde_json::Value>>>(&data)?;
    let start = Instant::now();
    let mut total = 0;
    let mut output_docs = Vec::with_capacity(values.len());
    for doc in values2.iter() {
        let data = serde_json::to_vec(black_box(doc))?;
        total += data.len();
        output_docs.push(black_box(data));
    }
    println!("Serde Serialize took {:?} {}", start.elapsed(), pretty!(total));

    let start = Instant::now();
    for doc in output_docs.iter() {
        let data = serde_json::from_slice::<BTreeMap<String, serde_json::Value>>(doc)?;
        black_box(data);
    }
    println!("Serde Deserialize took {:?} {:?}/iter", start.elapsed(), start.elapsed() / output_docs.len() as u32);

    let mut total = 0;
    let mut output_docs = Vec::with_capacity(rkyv_docs.len());
    let mut output_buffer = Vec::new();
    let start = Instant::now();
    for doc in rkyv_docs.iter() {
        let data = rkyv::to_bytes::<_, 1024>(black_box(doc))?;
        total += data.len();
        output_buffer.extend_from_slice(&data);
        output_docs.push(black_box(data));
    }
    println!("Rkyv Serialize took {:?} {}", start.elapsed(), pretty!(total));

    let start = Instant::now();
    let compressed = black_box(lz4_flex::compress(&output_buffer));
    println!(
        "Zstd Rkyv compress took {:?} total size: {}",
        start.elapsed(),
        pretty!(compressed.len())
    );
    let start = Instant::now();
    let decompressed = black_box(lz4_flex::decompress(&compressed, output_buffer.len()))?;
    println!(
        "Zstd Rkyv decompress took {:?} total size: {}",
        start.elapsed(),
        pretty!(decompressed.len())
    );

    let start = Instant::now();
    for doc in output_docs.iter() {
        let data = unsafe { rkyv::archived_root::<Document>(doc) };
        black_box(data);
    }
    println!("Rkyv Deserialize took {:?} {:?}/iter", start.elapsed(), start.elapsed() / output_docs.len() as u32);

    Ok(())
}
