use super::{Context, Entity};

pub trait Attribute: Send + Sync + std::fmt::Debug + std::any::Any + ayr_reflect::ToValue {
    fn name(&self) -> &'static str;

    fn on_spawn(&mut self, _ctx: &Context, _entity: &dyn Entity) {}
    fn on_change(&mut self, _ctx: &Context, _entity: &dyn Entity) {}
    fn on_destroy(&mut self, _ctx: &Context, _entity: &dyn Entity) {}
}

impl serde::Serialize for dyn Attribute {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_value().serialize(s)
    }
}
