use strum::VariantNames;
use strum_macros::EnumVariantNames;

#[derive(EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum SampleEnum {
    One,
    Two,
}

fn main() {
    dbg!(SampleEnum::VARIANTS);
}
