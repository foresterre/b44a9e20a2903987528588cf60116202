use strum_macros::EnumVariantNames;

#[derive(EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum SampleEnum {
    One,
    Two,
}
