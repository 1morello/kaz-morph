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
}
