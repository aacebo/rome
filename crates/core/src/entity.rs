use super::{Context, Zone};

pub trait Entity: Send + Sync + std::any::Any + std::fmt::Debug {
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context, _zone: &dyn Zone) {}
    fn on_change(&mut self, _ctx: &Context, _zone: &dyn Zone) {}
    fn on_destroy(&mut self, _ctx: &Context, _zone: &dyn Zone) {}
}

impl serde::Serialize for dyn Entity {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name().serialize(s)
    }
}
