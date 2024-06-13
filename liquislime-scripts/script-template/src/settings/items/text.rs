use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SdText;

impl SdText {
    pub fn serialize(&self, _writer: &mut impl std::io::Write) -> Result<()> {
        Ok(())
    }
}
