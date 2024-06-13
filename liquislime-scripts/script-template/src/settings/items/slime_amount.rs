use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SdSlimeAmount;

impl SdSlimeAmount {
    pub fn serialize(&self, _writer: &mut impl std::io::Write) -> Result<()> {
        Ok(())
    }
}
