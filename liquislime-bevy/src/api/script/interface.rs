use crate::api::{ApiFaction, ApiInstance, ApiPosition, ApiSlimeAmount, ApiTilePosition, ApiUnit};

pub trait LiquislimeImports: Send + Sync + 'static {
    fn level_width(&self) -> i32;
    fn level_height(&self) -> i32;

    fn get_current_unit(&self) -> ApiUnit;
    fn get_current_instance(&self) -> ApiInstance;

    fn get_own_faction(&self) -> ApiFaction;
    fn get_own_position(&self) -> ApiTilePosition;

    fn get_slime_amount(&self, faction: ApiFaction, position: ApiTilePosition) -> ApiSlimeAmount;
    fn set_slime_amount(
        &self,
        faction: ApiFaction,
        position: ApiTilePosition,
        amount: ApiSlimeAmount,
    );

    fn get_mouse_position(&self) -> Option<ApiPosition>;
    fn is_mouse_pressed(&self) -> bool;

    fn log(&self, message: &str);
}
