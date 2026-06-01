//! # hanmo (한모)
//!
//! 한글 음절 조합/분해와 자모 변환을 위한 작은 순수-유니코드 라이브러리.
//! 자모를 **모아** 음절을 만든다(모아쓰기)는 뜻에서 이름을 따왔다.
//!
//! A small, pure-Unicode library for **Hangul (한글) syllable composition and
//! decomposition**, plus jamo conversions. It bridges the three layers of
//! Hangul code points:
//!
//! - **Conjoining jamo** (첫가끝, U+1100..): the letters that combine into a block.
//! - **Precomposed syllables** (U+AC00.., e.g. `가`): a finished syllable.
//! - **Compatibility jamo** (U+3130.., e.g. `ㄱ` `ㅏ`): standalone letter forms.
//!
//! It also converts a consonant between its initial (초성) and final (종성)
//! positions. No input-method or configuration logic, just Unicode facts.
//!
//! ## Examples
//!
//! ```
//! // 조합 / 분해 (compose / decompose)
//! assert_eq!(hanmo::compose(0x1100, 0x1161, None), Some('가'));        // ㄱ + ㅏ
//! assert_eq!(hanmo::compose(0x1100, 0x1161, Some(0x11A8)), Some('각')); // + ㄱ받침
//! assert_eq!(hanmo::decompose('각'), Some((0x1100, 0x1161, Some(0x11A8))));
//!
//! // 호환 자모 다리 (conjoining <-> compatibility)
//! assert_eq!(hanmo::cho_cp_for_compat(0x3131), Some(0x1100)); // ㄱ -> 초성 ㄱ
//! assert_eq!(hanmo::jung_compat(0x1161), Some(0x314F));       // 중성 ㅏ -> ㅏ
//!
//! // 초성 <-> 종성 (initial <-> final consonant)
//! assert_eq!(hanmo::cho_to_jong(0x1100), Some(0x11A8)); // ㄱ초성 -> ㄱ종성
//! assert_eq!(hanmo::jong_to_cho(0x11BC), Some(0x110B)); // ㅇ종성 -> ㅇ초성
//! ```

#![forbid(unsafe_code)]

pub mod compat;
pub mod syllable;

// ── 음절 조합/분해 (syllable) ────────────────────────────────────────────────
pub use syllable::{
    cho_index, compose, compose_indices, decompose, is_conjoining_jamo, jong_index, jung_index,
    CHO, JONG, JUNG, LBASE, LCOUNT, SBASE, SCOUNT, TBASE, TCOUNT, VBASE, VCOUNT,
};

// ── 호환 자모 다리 + 초성/종성 변환 (compat) ─────────────────────────────────
pub use compat::{
    cho_compat, cho_cp_for_compat, cho_to_jong, is_vowel_compat, jong_compat, jong_cp_for_compat,
    jong_to_cho, jung_compat, jung_cp_for_compat, CHO_COMPAT, JONG_COMPAT, JUNG_COMPAT,
};
