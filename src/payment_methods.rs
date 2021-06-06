
use strum::{IntoEnumIterator, EnumIter, ToString};

/*
    EnumIter derive macro lets you iterate over an enum.
    It implements IntoEnumIterator trait and creates a new..
    iterator type called "YourEnumIter".
    Read more:https://docs.rs/strum/0.21.0/strum/derive.EnumIter.html
*/
#[derive(Debug, EnumIter, ToString)]
pub enum PaymentMethod{
    AmericanExpress,
    MasterCard,
    Discovery,
    Visa,
    #[strum(serialize ="JCB")] //We want to serialize Jcb as JCB
    Jcb
}

impl PaymentMethod{
    // Iterate over all variants of PaymentMethod enum
    // Use to_string() in map, to convert each element to String
    // Collect all as a Vec<String>
    pub fn enum_to_string() -> Vec<String>{
        let variants: Vec<String> = PaymentMethod::iter()
                                    .map(|x| x.to_string())
                                    .collect();
        variants
    }
}

