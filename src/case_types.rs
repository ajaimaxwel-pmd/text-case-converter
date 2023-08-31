use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Display, EnumString, IntoStaticStr)]  // strum macros.

pub enum CaseType {
    #[strum(serialize = "camel")]
    Camel,

    #[strum(serialize = "snake")]
    Snake,

    #[strum(serialize = "kebab")]
    Kebab,
    
    #[strum(serialize = "pascal")]
    Pascal,

    #[strum(serialize = "macro")]
    Macro,

    #[strum(serialize = "train")]
    Train,
}
