/// Vowel harmony group.
///
/// Жуан (back) vowels take suffixes with а/ы.
/// Жіңішке (front) vowels take suffixes with е/і.
///
/// This is the primary axis for Kazakh vowel harmony
/// and determines suffix selection throughout the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelGroup {
    /// Жуан дауыстылар: а, о, ұ, ы, у
    Back,
    /// Жіңішке дауыстылар: ә, ө, ү, е, і, и, э
    Front,
}

/// Classification of consonants by voicing.
///
/// Based on Kessikbayeva Table 3. Determines which
/// allomorph of a suffix is selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsonantClass {
    /// Үнді (sonorous): л, р, й, у, м, н, ң
    Sonorous,
    /// Ұяң (voiced): з, ж, б, в, г, д
    Voiced,
    /// Қатаң (voiceless): п, ф, қ, к, т, с, ш, ч, х, ц, щ
    Voiceless,
}

/// What the stem ends with — drives suffix allomorph selection.
///
/// Kessikbayeva Table 4: suffix variant depends on whether
/// the stem ends in a vowel, sonorous/voiced consonant,
/// or voiceless consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StemEnd {
    Vowel,
    SonorousOrVoiced,
    Voiceless,
}

const BACK_VOWELS: &[char] = &['а', 'о', 'ұ', 'ы', 'у'];
const FRONT_VOWELS: &[char] = &['ә', 'ө', 'ү', 'е', 'і', 'и', 'э'];

const SONOROUS: &[char] = &['л', 'р', 'й', 'у', 'м', 'н', 'ң'];
const VOICED: &[char] = &['з', 'ж', 'б', 'в', 'г', 'д'];
const VOICELESS: &[char] = &['п', 'ф', 'қ', 'к', 'т', 'с', 'ш', 'ч', 'х', 'ц', 'щ'];

/// Determine the vowel harmony group from the last vowel in a stem.
///
/// Kazakh vowel harmony is determined by the last vowel
/// in the word, not the first. Loan words from Persian
/// may mix groups (e.g., мұғалім — back then front),
/// but the last syllable still wins.
pub fn vowel_group(stem: &str) -> Option<VowelGroup> {
    stem.chars().rev().find_map(|c| classify_vowel(c))
}

/// Classify a single character as Back or Front vowel.
pub fn classify_vowel(c: char) -> Option<VowelGroup> {
    if BACK_VOWELS.contains(&c) {
        Some(VowelGroup::Back)
    } else if FRONT_VOWELS.contains(&c) {
        Some(VowelGroup::Front)
    } else {
        None
    }
}

/// Classify a consonant by voicing.
pub fn classify_consonant(c: char) -> Option<ConsonantClass> {
    if SONOROUS.contains(&c) {
        Some(ConsonantClass::Sonorous)
    } else if VOICED.contains(&c) {
        Some(ConsonantClass::Voiced)
    } else if VOICELESS.contains(&c) {
        Some(ConsonantClass::Voiceless)
    } else {
        None
    }
}

/// Classify how a stem ends — vowel, sonorous/voiced, or voiceless.
pub fn stem_end(stem: &str) -> StemEnd {
    let Some(last) = stem.chars().last() else {
        return StemEnd::Vowel;
    };

    if classify_vowel(last).is_some() {
        StemEnd::Vowel
    } else {
        match classify_consonant(last) {
            Some(ConsonantClass::Voiceless) => StemEnd::Voiceless,
            Some(_) => StemEnd::SonorousOrVoiced,
            // Unknown chars (e.g. Russian letters in loans) — default to voiced
            None => StemEnd::SonorousOrVoiced,
        }
    }
}

/// Consonant voicing mutation: voiceless → voiced.
///
/// Happens when a vowel-initial suffix follows a stem
/// ending in a voiceless stop:
///   п → б  (мектеп → мектебі)
///   к → г  (мектепке but: балық → балығы)
///   қ → ғ  (тауық → тауығы)
pub fn devoiced_to_voiced(c: char) -> Option<char> {
    match c {
        'п' => Some('б'),
        'к' => Some('г'),
        'қ' => Some('ғ'),
        _ => None,
    }
}

/// Reverse mutation: recover original stem consonant.
///
/// Used during analysis to reconstruct the lemma
/// from a mutated surface form.
pub fn voiced_to_devoiced(c: char) -> Option<char> {
    match c {
        'б' => Some('п'),
        'г' => Some('к'),
        'ғ' => Some('қ'),
        _ => None,
    }
}

/// Check if a character is a vowel (either group).
pub fn is_vowel(c: char) -> bool {
    classify_vowel(c).is_some()
}

/// Check if a character is a consonant.
pub fn is_consonant(c: char) -> bool {
    classify_consonant(c).is_some()
}

/// Validate that a suffix variant is compatible with a stem.
///
/// Checks two things:
/// 1. Vowel harmony - suffix vowels must match stem's last vowel group
/// 2. Consonant harmony - suffix-initial consonant must match stem-final class
///
/// Returns true if the combination is phonologically valid.
pub fn is_valid_suffix(stem: &str, suffix: &str) -> bool {
    if stem.is_empty() || suffix.is_empty() {
        return true;
    }

    // Check vowel harmony: suffix vowels must agree with stem's vowel group
    if let Some(stem_group) = vowel_group(stem) {
        if let Some(suffix_group) = vowel_group(suffix) {
            // Instrumental -мен/-бен/-пен is always front, skip harmony check
            if suffix != "мен" && suffix != "бен" && suffix != "пен"
                && suffix != "менен" && suffix != "бенен" && suffix != "пенен"
            {
                if stem_group != suffix_group {
                    return false;
                }
            }
        }
    }

    // Check consonant harmony: suffix-initial consonant must match stem-final class
    let s_end = stem_end(stem);
    let suffix_first = match suffix.chars().next() {
        Some(c) => c,
        None => return true,
    };

    match suffix_first {
        // д/т alternation: д after vowels and voiced, т after voiceless
        'д' => s_end != StemEnd::Voiceless,
        'т' => s_end == StemEnd::Voiceless,

        // н after vowels (for some suffixes), д/т otherwise
        'н' => s_end == StemEnd::Vowel || s_end == StemEnd::SonorousOrVoiced,

        // л after vowels, д after voiced, т after voiceless
        'л' => s_end == StemEnd::Vowel,

        // ғ/г after vowels and voiced, қ/к after voiceless
        'ғ' | 'г' => s_end != StemEnd::Voiceless,
        'қ' | 'к' => s_end == StemEnd::Voiceless,

        // м after vowels and voiced, б after some voiced, п after voiceless
        'м' => s_end == StemEnd::Vowel || s_end == StemEnd::SonorousOrVoiced,
        'б' => s_end == StemEnd::SonorousOrVoiced,
        'п' => s_end == StemEnd::Voiceless,

        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_vowel_harmony() {
        assert_eq!(vowel_group("қала"), Some(VowelGroup::Back));
        assert_eq!(vowel_group("балық"), Some(VowelGroup::Back));
        assert_eq!(vowel_group("тау"), Some(VowelGroup::Back));
    }

    #[test]
    fn front_vowel_harmony() {
        assert_eq!(vowel_group("мектеп"), Some(VowelGroup::Front));
        assert_eq!(vowel_group("үстел"), Some(VowelGroup::Front));
        assert_eq!(vowel_group("жүрек"), Some(VowelGroup::Front));
    }

    #[test]
    fn loan_word_last_syllable_wins() {
        // мұғалім: ұ(back), а(back), і(front) → front wins
        assert_eq!(vowel_group("мұғалім"), Some(VowelGroup::Front));
    }

    #[test]
    fn stem_end_classification() {
        assert_eq!(stem_end("мектеп"), StemEnd::Voiceless);
        assert_eq!(stem_end("қала"), StemEnd::Vowel);
        assert_eq!(stem_end("үстел"), StemEnd::SonorousOrVoiced);
        assert_eq!(stem_end("балық"), StemEnd::Voiceless);
    }

    #[test]
    fn consonant_mutations() {
        assert_eq!(devoiced_to_voiced('п'), Some('б'));
        assert_eq!(devoiced_to_voiced('қ'), Some('ғ'));
        assert_eq!(devoiced_to_voiced('к'), Some('г'));
        assert_eq!(devoiced_to_voiced('л'), None);
    }

    #[test]
    fn reverse_mutations() {
        assert_eq!(voiced_to_devoiced('б'), Some('п'));
        assert_eq!(voiced_to_devoiced('ғ'), Some('қ'));
        assert_eq!(voiced_to_devoiced('г'), Some('к'));
        assert_eq!(voiced_to_devoiced('д'), None);
    }

    #[test]
    fn vowel_consonant_checks() {
        assert!(is_vowel('а'));
        assert!(is_vowel('ө'));
        assert!(!is_vowel('т'));

        assert!(is_consonant('т'));
        assert!(is_consonant('л'));
        assert!(!is_consonant('а'));
    }

    #[test]
    fn valid_suffix_vowel_harmony() {
        // back stem + back suffix
        assert!(is_valid_suffix("мектеп", "те"));
        assert!(is_valid_suffix("қала", "да"));

        // back stem + front suffix = invalid
        assert!(!is_valid_suffix("қала", "де"));
        assert!(!is_valid_suffix("қала", "те"));

        // front stem + front suffix
        assert!(is_valid_suffix("мектеп", "тер"));
        assert!(!is_valid_suffix("мектеп", "тар"));
    }

    #[test]
    fn valid_suffix_consonant_harmony() {
        // voiceless stem + т variant
        assert!(is_valid_suffix("мектеп", "те"));
        assert!(!is_valid_suffix("мектеп", "де"));

        // vowel stem + д variant
        assert!(is_valid_suffix("қала", "да"));
        assert!(!is_valid_suffix("қала", "та"));

        // voiced stem + д variant
        assert!(is_valid_suffix("көз", "де"));
        assert!(!is_valid_suffix("көз", "те"));
    }

    #[test]
    fn instrumental_skips_harmony() {
        // -мен is always front, should pass with any stem
        assert!(is_valid_suffix("қала", "мен"));
        assert!(is_valid_suffix("мектеп", "пен"));
    }
}
