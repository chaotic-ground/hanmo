# hanmo (한모)

[![crates.io](https://img.shields.io/crates/v/hanmo.svg)](https://crates.io/crates/hanmo)
[![docs.rs](https://img.shields.io/docsrs/hanmo)](https://docs.rs/hanmo)

한글 음절 조합/분해와 자모 변환을 위한 작은 **순수 유니코드** 라이브러리.

A small, dependency-free, pure-Unicode Rust library for **Hangul (한글) syllable
composition / decomposition** and jamo conversion. It bridges the three layers
of Hangul code points and converts a consonant between initial/final position.
No input-method or configuration logic, just Unicode facts.

- **Conjoining jamo** (첫가끝, `U+1100..`): letters that combine into a block.
- **Precomposed syllables** (`U+AC00..`, e.g. `가`): a finished syllable.
- **Compatibility jamo** (`U+3130..`, e.g. `ㄱ` `ㅏ`): standalone letter forms.

## Usage

```rust
// 조합 / 분해 (compose / decompose)
assert_eq!(hanmo::compose(0x1100, 0x1161, None), Some('가'));         // ㄱ + ㅏ
assert_eq!(hanmo::compose(0x1100, 0x1161, Some(0x11A8)), Some('각')); // + ㄱ받침
assert_eq!(hanmo::decompose('각'), Some((0x1100, 0x1161, Some(0x11A8))));

// 호환 자모 다리 (conjoining <-> compatibility jamo)
assert_eq!(hanmo::cho_cp_for_compat(0x3131), Some(0x1100)); // ㄱ -> 초성 ㄱ
assert_eq!(hanmo::jung_compat(0x1161), Some(0x314F));       // 중성 ㅏ -> ㅏ

// 초성 <-> 종성 (initial <-> final consonant)
assert_eq!(hanmo::cho_to_jong(0x1100), Some(0x11A8)); // ㄱ초성 -> ㄱ종성
assert_eq!(hanmo::jong_to_cho(0x11BC), Some(0x110B)); // ㅇ종성 -> ㅇ초성
```

## What it does

| Area | Functions |
|---|---|
| 음절 조합/분해 | `compose`, `compose_indices`, `decompose` |
| 자모 인덱스 | `cho_index`, `jung_index`, `jong_index`, `is_conjoining_jamo` |
| 호환 자모 다리 | `cho_compat` / `jung_compat` / `jong_compat`, `cho_cp_for_compat` / `jung_cp_for_compat` / `jong_cp_for_compat`, `is_vowel_compat` |
| 초성↔종성 변환 | `cho_to_jong`, `jong_to_cho` |
| 표 (constants) | `CHO`/`JUNG`/`JONG`, `CHO_COMPAT`/`JUNG_COMPAT`/`JONG_COMPAT`, `SBASE`/`LBASE`/`VBASE`/`TBASE`, … |

Only modern Hangul precomposes (U+AC00 block); old-Hangul conjoining jamo are
recognized by `is_conjoining_jamo` but have no precomposed form.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
