use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasmi::{errors::LinkerError, AsContext, AsContextMut};
#[allow(unused)]
use wasmi_component::{
    CallResult, Component, ComponentValue, HostResult, Instance, Linker, ListAccessor, Lower,
    StoreData, TypedFunc,
};

#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl wasmi_component::ComponentValue for Position {
    type Borrowed<'a> = PositionBorrowed<'a>;
    fn value_type() -> wasmi_component::ValueType {
        wasmi_component::ValueType::Record {
            name: std::rc::Rc::from("Position"),
            fields: std::rc::Rc::from([
                (std::rc::Rc::from("x"), <f32>::value_type()),
                (std::rc::Rc::from("y"), <f32>::value_type()),
            ]),
        }
    }
    fn arg_count() -> usize {
        0 + <f32>::arg_count() + <f32>::arg_count()
    }
    fn byte_align() -> usize {
        let mut result = 0;
        result = std::cmp::max(result, <f32>::byte_align());
        result = std::cmp::max(result, <f32>::byte_align());
        result
    }
    fn byte_size() -> usize {
        let align = Self::byte_align();
        let mut result = 0;
        result += wasmi_component::helpers::round_up(<f32>::byte_size(), align);
        result += wasmi_component::helpers::round_up(<f32>::byte_size(), align);
        result
    }
    fn lift<'mem>(
        reader: &mut impl wasmi_component::lib_structs::LiftReader<'mem>,
    ) -> wasmi_component::ConvertResult<Self::Borrowed<'mem>> {
        let align = Self::byte_align();
        let x = reader.read_record_field::<f32>(align)?;
        let y = reader.read_record_field::<f32>(align)?;
        Ok(PositionBorrowed { x, y })
    }
}
impl wasmi_component::Lower<Self> for Position {
    fn lower(
        &self,
        writer: &mut impl wasmi_component::lib_structs::LowerWriter,
    ) -> wasmi_component::ConvertResult<()> {
        let align = Self::byte_align();
        writer.write_record_field(&self.x, align)?;
        writer.write_record_field(&self.y, align)?;
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct PositionBorrowed<'a> {
    pub x: <f32 as wasmi_component::ComponentValue>::Borrowed<'a>,
    pub y: <f32 as wasmi_component::ComponentValue>::Borrowed<'a>,
}
impl wasmi_component::Lift<Position> for PositionBorrowed<'_> {
    fn lift_owned(&self) -> wasmi_component::ConvertResult<Position> {
        Ok(Position {
            x: self.x.lift_owned()?,
            y: self.y.lift_owned()?,
        })
    }
    fn lift_to(&self, target: &mut Position) -> wasmi_component::ConvertResult<()> {
        self.x.lift_to(&mut target.x)?;
        self.y.lift_to(&mut target.y)?;
        Ok(())
    }
}

#[allow(unused)]
pub trait AdaptorImports {
    fn get_mouse_world_position(&mut self) -> HostResult<Position>;
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct AdaptorExports {
    pub instance: Instance,
    pub update: TypedFunc<(f64,), ()>,
}

#[allow(unused)]
impl AdaptorExports {
    pub fn call_update<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        time_elapsed: f64,
    ) -> CallResult<()> {
        self.update.call(ctx, (time_elapsed,))
    }

    pub fn call_update_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        time_elapsed: f64,
        callback: impl FnOnce(&mut T, ()) -> R,
    ) -> CallResult<R> {
        self.update
            .call_with_results(ctx, (time_elapsed,), callback)
    }
}

#[allow(unused)]
pub fn add_adaptor_to_linker<T: AdaptorImports>(linker: &mut Linker<T>) -> Result<(), LinkerError> {
    linker.func_typed::<(), Position>(
        "liquislime:api/host-functions",
        "get-mouse-world-position",
        |host_data, params| host_data.get_mouse_world_position(),
    )?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_adaptor_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<AdaptorExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    let update = instance.get_typed_func(ctx.as_context(), "", "update")?;

    Ok(AdaptorExports { instance, update })
}
