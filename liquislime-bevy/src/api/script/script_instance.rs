use super::{LoadedScript, Script};
use crate::api::{
    ApiTimeInterval, ScriptImpl, SettingsDescription, SettingsTempValue, SettingsUiDisplay,
    SettingsValue,
};
use anyhow::bail;
use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use slab::Slab;

static INSTANCES: AtomicRefCell<Slab<ScriptInstanceInner>> = AtomicRefCell::new(Slab::new());

#[derive(Debug, Clone, Copy)]
pub struct ScriptInstance(usize);

#[derive(Debug)]
struct ScriptInstanceInner {
    id: u32,
    script: Script,
    loaded_script: LoadedScript,
    settings: SettingsValue,
    temp_settings: SettingsTempValue,
    instance_state: InstanceState,
}

#[derive(Debug)]
enum InstanceState {
    Initialized,
    NotInitialized,
}

impl ScriptInstance {
    pub fn new(script: Script, loaded_script: LoadedScript, settings: SettingsValue) -> Self {
        let mut lock = INSTANCES.try_borrow_mut().unwrap();

        let id = lock.vacant_key();

        let mut temp_settings = SettingsTempValue(().into());

        loaded_script
            .settings_description()
            .reset_settings(&settings, &mut temp_settings);

        let inner = ScriptInstanceInner {
            id: id as u32,
            script,
            loaded_script,
            settings,
            temp_settings,
            instance_state: InstanceState::NotInitialized,
        };

        let index = lock.insert(inner);

        assert_eq!(id, index, "Index must match id!");

        Self(index)
    }

    pub fn change_settings(self) -> Result<()> {
        self.with_inner(|inner| {
            inner
                .loaded_script
                .script_impl()
                .change_settings(inner.id, &inner.settings)
        })
    }

    pub fn update(self, time_elapsed: ApiTimeInterval) -> Result<()> {
        self.with_inner(|inner| {
            inner
                .loaded_script
                .script_impl()
                .update(inner.id, time_elapsed)
        })
    }

    pub fn id(self) -> u32 {
        self.0 as u32
    }

    pub fn with_name<T>(self, callback: impl FnOnce(&str) -> T) -> T {
        self.with_inner(move |inner| callback(inner.script.name()))
    }

    pub fn with_settings<T>(
        self,
        callback: impl FnOnce(&SettingsDescription, &mut SettingsValue, &mut SettingsTempValue) -> T,
    ) -> T {
        self.with_inner_mut(|inner| {
            callback(
                inner.loaded_script.settings_description(),
                &mut inner.settings,
                &mut inner.temp_settings,
            )
        })
    }

    fn with_inner<T>(self, callback: impl FnOnce(&ScriptInstanceInner) -> T) -> T {
        callback(&INSTANCES.try_borrow().unwrap()[self.0])
    }

    fn with_inner_mut<T>(self, callback: impl FnOnce(&mut ScriptInstanceInner) -> T) -> T {
        callback(&mut INSTANCES.try_borrow_mut().unwrap()[self.0])
    }
}
