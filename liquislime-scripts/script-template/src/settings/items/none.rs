use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SdNone;

impl SdNone {
    pub fn serialize(&self, _writer: &mut impl std::io::Write) -> Result<()> {
        Ok(())
    }
}
