# GST Invoice Engine

A fast, accurate Rust library for GST (Goods and Services Tax) calculations, built for Indian invoicing systems.

## Features

- **GST Calculation** — Calculate GST amount and total from base price and GST percentage
- **CGST/SGST/IGST Split** — Automatically split GST based on whether the transaction is intra-state or inter-state
- **Duplicate Item Detection** — Case-insensitive detection of duplicate line items in an invoice
- **Invoice Validation** — Validates GSTIN format, amount, and buyer details before invoice generation

## Why Rust?

This engine is written in Rust for speed and reliability:
- Runs with minimal memory footprint
- Zero runtime errors from type mismatches (caught at compile time)
- Can be compiled to WebAssembly (WASM) for offline, in-browser calculations — no server round-trip needed

## Usage

```rust
use gst_engine::{calculate_gst, calculate_total, calculate_gst_split};

let gst = calculate_gst(1000.0, 18.0); // 180.0
let total = calculate_total(1000.0, 18.0); // 1180.0
let split = calculate_gst_split(1000.0, 18.0, true); // CGST: 90.0, SGST: 90.0
```

## Running Tests

```bash
cargo test
```

All 13 tests cover GST calculation, splitting, duplicate detection, and validation logic.

## Status

🚧 **MVP / Prototype** — Core calculation logic is implemented and fully tested (13 passing tests), but this is not yet production-hardened. 

**What's solid:** GST calculation, CGST/SGST/IGST split, duplicate-item detection, basic invoice validation.

**What's not yet handled:** HSN-code-based automatic GST rate lookup, composition scheme, reverse charge, exempted items, and precision-safe decimal arithmetic (currently uses `f64`, which is not ideal for financial calculations at scale).

Inspired by real-world GST invoicing needs from [VoltaBill](https://voltabill.netlify.app), an invoicing tool for electrical contractors — built as an independent, reusable engine for any GST-registered business.
## Author

Built by [Mayur Nagrare](https://github.com/9create) — May Software Solutions

## Author

Built by [Mayur Nagrare](https://github.com/9create) — May Software Solutions