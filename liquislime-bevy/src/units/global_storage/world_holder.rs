use bevy::prelude::*;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{Mutex, MutexGuard},
};

static GLOBAL_WORLD: Mutex<InnerWorldRef> = Mutex::new(InnerWorldRef(std::ptr::null_mut()));
static THREAD_LOCK: Mutex<()> = Mutex::new(());

fn set_world(world: *mut World) {
    GLOBAL_WORLD.try_lock().expect("Set world mutex lock").0 = world;
}

pub fn get_world() -> impl DerefMut<Target = World> {
    let locked = GLOBAL_WORLD.try_lock().expect("Get world mutex lock");
    WorldRef(locked)
}

struct InnerWorldRef(*mut World);
unsafe impl Send for InnerWorldRef {}

struct WorldRef<'a>(MutexGuard<'a, InnerWorldRef>);

impl Deref for WorldRef<'_> {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        let inner_pointer = self.0 .0;

        if inner_pointer.is_null() {
            panic!("Global world reference is null.");
        }

        // SAFETY: shared reference to self ensures that no mutable reference exists.
        // See the DerefMut impl for the rest of the safety arguments.
        unsafe { &*inner_pointer }
    }
}

impl DerefMut for WorldRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let inner_pointer = self.0 .0;

        if inner_pointer.is_null() {
            panic!("Global world reference is null.");
        }

        /* SAFETY:
         * Setting the world referene is not public. The only way to set it is via the `use_world_reference_in` function.
         * That takes the mutable reference to the world, so no one else can use it while the function runs.
         * And it is only while the function runs that this code will be reached, because of the null check.
         * You also cannot call the `use_world_reference_in` twice at the same time because of the THREAD_LOCK.
         * Finally, you cannot call get_world() twice to get 2 references, because of the mutex lock.
         */
        unsafe { &mut *inner_pointer }
    }
}

pub fn use_world_reference_in(reference: &mut World, callback: impl FnOnce()) {
    let lock = THREAD_LOCK
        .try_lock()
        .expect("You should only call this method once at a time");

    assert!(GLOBAL_WORLD.try_lock().unwrap().0.is_null());

    set_world(reference as *mut _);

    callback();

    set_world(std::ptr::null_mut());

    drop(lock);
}
