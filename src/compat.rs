//! 호환 자모(U+3130) ↔ 조합용 자모(U+1100) 다리, 그리고 초성↔종성 변환.
//! Compatibility-jamo (U+3130) <-> conjoining-jamo (U+1100) bridge, and
//! cho<->jong consonant conversion. Pure Unicode data, no IME logic.

use crate::syllable::{self, CHO, JONG, JUNG};

/// 현대 초성 인덱스 → 호환 자모.
pub const CHO_COMPAT: [u32; 19] = [
    0x3131, 0x3132, 0x3134, 0x3137, 0x3138, 0x3139, 0x3141, 0x3142, 0x3143, 0x3145, 0x3146, 0x3147,
    0x3148, 0x3149, 0x314A, 0x314B, 0x314C, 0x314D, 0x314E,
];

/// 현대 중성 인덱스 → 호환 자모(U+314F..=U+3163, 연속).
pub const JUNG_COMPAT: [u32; 21] = [
    0x314F, 0x3150, 0x3151, 0x3152, 0x3153, 0x3154, 0x3155, 0x3156, 0x3157, 0x3158, 0x3159, 0x315A,
    0x315B, 0x315C, 0x315D, 0x315E, 0x315F, 0x3160, 0x3161, 0x3162, 0x3163,
];

/// 종성 인덱스(0=없음) → 호환 자모. 0 자리는 0.
pub const JONG_COMPAT: [u32; 28] = [
    0x0000, 0x3131, 0x3132, 0x3133, 0x3134, 0x3135, 0x3136, 0x3137, 0x3139, 0x313A, 0x313B, 0x313C,
    0x313D, 0x313E, 0x313F, 0x3140, 0x3141, 0x3142, 0x3144, 0x3145, 0x3146, 0x3147, 0x3148, 0x314A,
    0x314B, 0x314C, 0x314D, 0x314E,
];

/// 호환 자모 → 조합용 초성 코드포인트(현대 초성에 대응할 때만).
pub fn cho_cp_for_compat(compat: u32) -> Option<u32> {
    CHO_COMPAT.iter().position(|&c| c == compat).map(|i| CHO[i])
}

/// 호환 자모 → 조합용 중성 코드포인트(현대 중성에 대응할 때만).
pub fn jung_cp_for_compat(compat: u32) -> Option<u32> {
    JUNG_COMPAT
        .iter()
        .position(|&c| c == compat)
        .map(|i| JUNG[i])
}

/// 호환 자모 → 조합용 종성 코드포인트(받침에 대응할 때만; 인덱스 0=받침 없음 제외).
pub fn jong_cp_for_compat(compat: u32) -> Option<u32> {
    JONG_COMPAT
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, &c)| c == compat)
        .map(|(i, _)| JONG[i])
}

/// 호환 자모가 모음(U+314F..=U+3163)인가.
pub fn is_vowel_compat(compat: u32) -> bool {
    (0x314F..=0x3163).contains(&compat)
}

/// 초성 코드포인트를 같은 자음의 종성 코드포인트로 바꾼다(대응 없으면 None).
/// 예: ㄱ초성(U+1100) → ㄱ종성(U+11A8).
pub fn cho_to_jong(cho_cp: u32) -> Option<u32> {
    let compat = syllable::cho_index(cho_cp).map(|i| CHO_COMPAT[i as usize])?;
    jong_cp_for_compat(compat)
}

/// 종성 코드포인트를 같은 자음의 초성 코드포인트로 바꾼다(대응 없으면 None).
/// 예: ㅇ종성(U+11BC) → ㅇ초성(U+110B).
pub fn jong_to_cho(jong_cp: u32) -> Option<u32> {
    let compat = syllable::jong_index(jong_cp).map(|i| JONG_COMPAT[i as usize])?;
    cho_cp_for_compat(compat)
}

/// 조합용 자모 코드포인트의 호환 자모(현대 집합일 때만). 위치(초/중/종)별로 본다.
/// `cho_compat`/`jung_compat`/`jong_compat` 의 통합 진입점이 필요할 때 사용.
pub fn cho_compat(cho_cp: u32) -> Option<u32> {
    syllable::cho_index(cho_cp).map(|i| CHO_COMPAT[i as usize])
}
/// 중성 코드포인트 → 호환 자모.
pub fn jung_compat(jung_cp: u32) -> Option<u32> {
    syllable::jung_index(jung_cp).map(|i| JUNG_COMPAT[i as usize])
}
/// 종성 코드포인트 → 호환 자모.
pub fn jong_compat(jong_cp: u32) -> Option<u32> {
    syllable::jong_index(jong_cp).map(|i| JONG_COMPAT[i as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compat_bridge_roundtrip() {
        // ㄱ초성 U+1100 → 호환 ㄱ U+3131 → 다시 초성 U+1100
        assert_eq!(cho_compat(0x1100), Some(0x3131));
        assert_eq!(cho_cp_for_compat(0x3131), Some(0x1100));
        // ㄱ종성 U+11A8 → 호환 ㄱ U+3131
        assert_eq!(jong_compat(0x11A8), Some(0x3131));
        assert_eq!(jong_cp_for_compat(0x3131), Some(0x11A8));
        // ㅏ중성 U+1161 → 호환 ㅏ U+314F
        assert_eq!(jung_compat(0x1161), Some(0x314F));
        assert_eq!(jung_cp_for_compat(0x314F), Some(0x1161));
    }

    #[test]
    fn cho_jong_conversion() {
        // ㄱ초성 ↔ ㄱ종성
        assert_eq!(cho_to_jong(0x1100), Some(0x11A8));
        assert_eq!(jong_to_cho(0x11A8), Some(0x1100));
        // ㅇ초성 U+110B ↔ ㅇ종성 U+11BC
        assert_eq!(cho_to_jong(0x110B), Some(0x11BC));
        assert_eq!(jong_to_cho(0x11BC), Some(0x110B));
    }

    #[test]
    fn is_vowel() {
        assert!(is_vowel_compat(0x314F)); // ㅏ
        assert!(is_vowel_compat(0x3163)); // ㅣ
        assert!(!is_vowel_compat(0x3131)); // ㄱ
    }
}
