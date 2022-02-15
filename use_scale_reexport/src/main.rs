use scale_reexport::codec::{Decode, Encode};

#[derive(Decode, Encode)]
//#[codec(crate = scale_reexport::codec)]
enum TestEnum {
    FirstVariant,
    SecondVariant,
}

fn main() {
    let x = TestEnum::FirstVariant;
    let _y = x.encode();
}
