use super::*;

pub struct LiquislimeUnit;

impl LiquislimePlugin for LiquislimeUnit {
    fn update(time_elapsed: TimeInterval) {
        println!("Time elapsed: {:?}", time_elapsed);
    }
}
