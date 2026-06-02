use crate::types::*;

/// Kazakh personal pronouns have stem changes in oblique cases
/// These can't be handled by suffix stripping
/// and require a dedicated lookup table.
pub struct IrregularForm {
    pub surface: &'static str,
    pub lemma: &'static str,
    pub pos: Pos,
    pub features: Features,
}

/// Table of all known irregular forms.
pub fn irregular_forms() -> Vec<IrregularForm> {
    vec![
        // мен (я) 
        irregular("маған", "мен", Pos::Pronoun, Some(Case::Dative)),
        irregular("менің", "мен", Pos::Pronoun, Some(Case::Genitive)),
        irregular("мені", "мен", Pos::Pronoun, Some(Case::Accusative)),
        irregular("менде", "мен", Pos::Pronoun, Some(Case::Locative)),
        irregular("менен", "мен", Pos::Pronoun, Some(Case::Ablative)),
        irregular("менімен", "мен", Pos::Pronoun, Some(Case::Instrumental)),

        //  сен (ты) 
        irregular("саған", "сен", Pos::Pronoun, Some(Case::Dative)),
        irregular("сенің", "сен", Pos::Pronoun, Some(Case::Genitive)),
        irregular("сені", "сен", Pos::Pronoun, Some(Case::Accusative)),
        irregular("сенде", "сен", Pos::Pronoun, Some(Case::Locative)),
        irregular("сенен", "сен", Pos::Pronoun, Some(Case::Ablative)),
        irregular("сенімен", "сен", Pos::Pronoun, Some(Case::Instrumental)),

        //  ол (он/она)
        irregular("оған", "ол", Pos::Pronoun, Some(Case::Dative)),
        irregular("оның", "ол", Pos::Pronoun, Some(Case::Genitive)),
        irregular("оны", "ол", Pos::Pronoun, Some(Case::Accusative)),
        irregular("онда", "ол", Pos::Pronoun, Some(Case::Locative)),
        irregular("одан", "ол", Pos::Pronoun, Some(Case::Ablative)),
        irregular("онымен", "ол", Pos::Pronoun, Some(Case::Instrumental)),

        //  біз (мы) 
        irregular("бізге", "біз", Pos::Pronoun, Some(Case::Dative)),
        irregular("біздің", "біз", Pos::Pronoun, Some(Case::Genitive)),
        irregular("бізді", "біз", Pos::Pronoun, Some(Case::Accusative)),
        irregular("бізде", "біз", Pos::Pronoun, Some(Case::Locative)),
        irregular("бізден", "біз", Pos::Pronoun, Some(Case::Ablative)),
        irregular("бізбен", "біз", Pos::Pronoun, Some(Case::Instrumental)),

        //  сіз (Вы) 
        irregular("сізге", "сіз", Pos::Pronoun, Some(Case::Dative)),
        irregular("сіздің", "сіз", Pos::Pronoun, Some(Case::Genitive)),
        irregular("сізді", "сіз", Pos::Pronoun, Some(Case::Accusative)),
        irregular("сізде", "сіз", Pos::Pronoun, Some(Case::Locative)),
        irregular("сізден", "сіз", Pos::Pronoun, Some(Case::Ablative)),
        irregular("сізбен", "сіз", Pos::Pronoun, Some(Case::Instrumental)),

        //  олар (они) 
        irregular("оларға", "олар", Pos::Pronoun, Some(Case::Dative)),
        irregular("олардың", "олар", Pos::Pronoun, Some(Case::Genitive)),
        irregular("оларды", "олар", Pos::Pronoun, Some(Case::Accusative)),
        irregular("оларда", "олар", Pos::Pronoun, Some(Case::Locative)),
        irregular("олардан", "олар", Pos::Pronoun, Some(Case::Ablative)),
        irregular("олармен", "олар", Pos::Pronoun, Some(Case::Instrumental)),
    ]
}

fn irregular(surface: &'static str, lemma: &'static str, pos: Pos, case: Option<Case>) -> IrregularForm {
    IrregularForm {
        surface,
        lemma,
        pos,
        features: Features {
            case,
            ..Default::default()
        },
    }
}
