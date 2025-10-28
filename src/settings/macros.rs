#[macro_export]
macro_rules! settings_enum {
    (
        $(#[$meta:meta])*
        pub enum $Name:ident {
            $(
                $(#[$vmeta:meta])*
                $Variant:ident
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, strum::EnumIter, Debug, Default, serde::Deserialize, serde::Serialize, strum::AsRefStr, strum::VariantNames, strum::EnumString)]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        pub enum $Name {
            $(
                $(#[$vmeta])*
                $Variant,
            )*
        }
    }
}
