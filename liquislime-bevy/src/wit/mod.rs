wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "liquislime-unit"
});

struct LiquislimeHost;

impl LiquislimeUnitImports for LiquislimeHost {
    fn get_own_position(&mut self) -> wasmtime::Result<TilePosition> {
        todo!("import get own")
    }

    fn add_slime_amount(
        &mut self,
        position: TilePosition,
        amount: SlimeAmount,
    ) -> wasmtime::Result<()> {
        todo!("import add slime")
    }
}

impl std::ops::Add for SlimeAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SlimeAmount {
            amount: self.amount + rhs.amount,
        }
    }
}

impl std::ops::AddAssign for SlimeAmount {
    fn add_assign(&mut self, rhs: Self) {
        self.amount += rhs.amount;
    }
}

impl std::ops::Sub for SlimeAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        SlimeAmount {
            amount: self.amount - rhs.amount,
        }
    }
}

impl std::ops::SubAssign for SlimeAmount {
    fn sub_assign(&mut self, rhs: Self) {
        self.amount -= rhs.amount;
    }
}

impl std::ops::Div<i64> for SlimeAmount {
    type Output = Self;

    fn div(self, rhs: i64) -> Self {
        SlimeAmount {
            amount: self.amount / rhs,
        }
    }
}

pub trait NonNegative {
    fn non_negative(self) -> Self;
}

impl NonNegative for SlimeAmount {
    fn non_negative(self) -> Self {
        SlimeAmount {
            amount: self.amount.max(0),
        }
    }
}
