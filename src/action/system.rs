use crate::{action::Action, diagnostic::DiagnosticBuffer, world::World};

#[derive(Debug, serde::Serialize)]
#[serde(tag = "name")]
pub enum SystemAction {
    Stop,
}

impl Action for SystemAction {
    fn name(&self) -> &str {
        match self {
            Self::Stop => "system.stop",
        }
    }

    fn apply(self: Box<Self>, _world: &mut World, _diagnostics: &mut DiagnosticBuffer) {
        todo!()
    }
}
