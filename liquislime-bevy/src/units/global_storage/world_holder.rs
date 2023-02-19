use bevy::prelude::*;
use std::{marker::PhantomData, ops::DerefMut, sync::Mutex};

static GLOBAL_WORLD: Mutex<UnsafeWorldRef> = Mutex::new(UnsafeWorldRef(std::ptr::null_mut()));

fn set_world(world: *mut World) {
    (*GLOBAL_WORLD.lock().expect("Set world mutex lock")).0 = world;
}

pub fn get_world() -> impl DerefMut<Target = World> {
    let locked = GLOBAL_WORLD.lock().expect("Get world mutex lock");
    let inner = locked.0;

    if inner.is_null() {
        panic!("Global world reference is null.");
    }

    /* SAFETY:
     * Setting the world referene is not public. The only way to set it is via the `use_world_reference_in` function.
     * That takes the mutable reference to the world, so no one else can use it while the function runs.
     * And it is only while the function run that this code will be reached, because of the null check.
     * Finally, you cannot call get_world() twice to get 2 reference, because of the mutex lock.
     */
    unsafe { &mut *inner }

    // TODO: this is still unsafe!
}

struct UnsafeWorldRef(*mut World);
unsafe impl Send for UnsafeWorldRef {}

#[derive(Debug, Clone, Copy)]
pub struct WorldRefToken {
    _private: PhantomData<()>, // Disable public constructor
}

pub fn use_world_reference_in(reference: &mut World, use_in: impl FnOnce(&WorldRefToken)) {
    set_world(reference as *mut _);

    let ref_token = WorldRefToken {
        _private: PhantomData,
    };

    use_in(&ref_token);

    set_world(std::ptr::null_mut());
}
