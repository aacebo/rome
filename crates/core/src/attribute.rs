use serde::ser::SerializeStruct;

use super::{Context, Entity};

pub trait Attribute: Send + Sync + std::fmt::Debug + std::any::Any {
    fn name(&self) -> &'static str;
    fn value(&self) -> ayr_reflect::Value;

    fn on_spawn(&mut self, _ctx: &Context, _entity: &dyn Entity) {}
    fn on_change(&mut self, _ctx: &Context, _entity: &dyn Entity) {}
    fn on_destroy(&mut self, _ctx: &Context, _entity: &dyn Entity) {}
}

impl serde::Serialize for dyn Attribute {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut v = s.serialize_struct("Entity", 2)?;
        v.serialize_field("name", self.name())?;
        v.serialize_field("value", &self.value())?;
        v.end()
    }
}
