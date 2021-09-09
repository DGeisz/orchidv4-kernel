pub enum DeclarationEnum {
    Constant,
    Axiom,
    Definition,
    Theorem,
    Lemma,
}

pub const CONSTANT: &str = "constant";
pub const AXIOM: &str = "axiom";
pub const DEFINITION: &str = "definition";
pub const THEOREM: &str = "theorem";
pub const LEMMA: &str = "lemma";

pub fn name_to_dec_autocomplete(dec_name: String) -> Option<DeclarationEnum> {
    let cleaned_name = dec_name.to_lowercase();

    if CONSTANT.starts_with(&cleaned_name) {
        Some(DeclarationEnum::Constant)
    } else if AXIOM.starts_with(&cleaned_name) {
        Some(DeclarationEnum::Axiom)
    } else if DEFINITION.starts_with(&cleaned_name) {
        Some(DeclarationEnum::Definition)
    } else if THEOREM.starts_with(&cleaned_name) {
        Some(DeclarationEnum::Theorem)
    } else if LEMMA.starts_with(&cleaned_name) {
        Some(DeclarationEnum::Lemma)
    } else {
        None
    }
}
