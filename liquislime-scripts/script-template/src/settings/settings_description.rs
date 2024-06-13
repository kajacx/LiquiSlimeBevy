use super::{SdNone, SdSlimeAmount, SdText};
use crate::api::ToWasmAbi;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SettingsDescription {
    None(SdNone),
    Text(SdText),
    SlimeAmount(SdSlimeAmount),
}

impl SettingsDescription {
    pub fn serialize(&self, writer: &mut impl std::io::Write) -> Result<()> {
        match self {
            Self::None(none) => {
                rmp::encode::write_str(writer, "None")?;
                none.serialize(writer)
            }
            Self::Text(text) => {
                rmp::encode::write_str(writer, "Text")?;
                text.serialize(writer)
            }
            Self::SlimeAmount(amount) => {
                rmp::encode::write_str(writer, "SlimeAmount")?;
                amount.serialize(writer)
            }
        }
    }
}

impl ToWasmAbi for SettingsDescription {
    type Abi = <&'static [u8] as ToWasmAbi>::Abi;

    fn to_wasm_abi(&self) -> Self::Abi {
        let mut bytes = vec![];
        self.serialize(&mut bytes)
            .expect("writing to expandable vec");
        bytes.as_slice().to_wasm_abi()
    }
}
