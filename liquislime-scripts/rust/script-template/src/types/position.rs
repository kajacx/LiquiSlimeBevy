use super::TilePosition;
use crate::api::{pack_f32s, unpack_f32s, FromWasmAbi, ToWasmAbi};
use anyhow::Result;
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, Add, Sub, AddAssign, SubAssign, PartialEq, PartialOrd)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add_x(self, x_add: f32) -> Self {
        Self {
            x: self.x + x_add,
            ..self
        }
    }

    pub fn add_y(self, y_add: f32) -> Self {
        Self {
            y: self.y + y_add,
            ..self
        }
    }

    pub fn from_tile_center(tile_position: TilePosition) -> Self {
        Self::new(tile_position.x as f32 + 0.5, tile_position.y as f32 + 0.5)
    }

    pub const fn from_tile_bottom_left(tile_position: TilePosition) -> Self {
        Self::new(tile_position.x as f32, tile_position.y as f32)
    }

    pub fn to_tile_position(self) -> TilePosition {
        TilePosition::from_position(self)
    }

    pub fn is_in_bounds(self) -> bool {
        self.to_tile_position().is_in_bounds()
    }

    pub fn is_in_tile(self, tile_position: TilePosition) -> bool {
        tile_position.contains(self)
    }

    pub fn serialize(self, writer: &mut impl std::io::Write) -> Result<()> {
        rmp::encode::write_f32(writer, self.x)?;
        rmp::encode::write_f32(writer, self.y)?;
        Ok(())
    }

    pub fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        let x = rmp::decode::read_f32(reader)?;
        let y = rmp::decode::read_f32(reader)?;
        Ok(Self::new(x, y))
    }
}

impl ToWasmAbi for Position {
    type Abi = u64;

    fn to_wasm_abi(&self) -> Self::Abi {
        pack_f32s(self.x, self.y)
    }
}

impl FromWasmAbi for Position {
    type Abi = u64;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        let unpacked = unpack_f32s(abi);

        Ok(Self {
            x: unpacked.0,
            y: unpacked.1,
        })
    }
}

impl ToWasmAbi for Option<Position> {
    type Abi = u64;

    fn to_wasm_abi(&self) -> Self::Abi {
        match &self {
            Some(position) => position.to_wasm_abi(),
            None => pack_f32s(f32::NAN, f32::NAN),
        }
    }
}

impl FromWasmAbi for Option<Position> {
    type Abi = u64;

    fn from_wasm_abi(abi: Self::Abi) -> Result<Self> {
        let unpacked = unpack_f32s(abi);
        Ok(if unpacked.0.is_nan() {
            None
        } else {
            Some(Position {
                x: unpacked.0,
                y: unpacked.1,
            })
        })
    }
}
