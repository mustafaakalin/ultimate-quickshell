use std::sync::Arc;

use crate::{EnvironmentEvent, EnvironmentState};

/// Pure state transition boundary.
///
/// External actors produce events; only the reducer is allowed to turn those
/// events into a new authoritative snapshot. This keeps I/O and policy out of
/// the state transition itself.
#[derive(Default)]
pub struct StateReducer;

impl StateReducer {
    pub fn reduce(
        &self,
        current: &EnvironmentState,
        event: &EnvironmentEvent,
    ) -> Option<EnvironmentState> {
        match event {
            EnvironmentEvent::Snapshot(snapshot) => Some((**snapshot).clone()),
            EnvironmentEvent::ComponentChanged { .. }
            | EnvironmentEvent::Incident(_)
            | EnvironmentEvent::AgentIntent(_)
            | EnvironmentEvent::TransactionPlanned(_) => {
                let mut next = current.clone();
                next.generation = next.generation.saturating_add(1);
                Some(next)
            }
            EnvironmentEvent::Command(_) => None,
            EnvironmentEvent::Compositor(event) => {
                let mut next = current.clone();
                next.compositor = event.compositor.clone();
                next.generation = next.generation.saturating_add(1);
                Some(next)
            }
        }
    }

    pub fn apply(
        &self,
        current: Arc<EnvironmentState>,
        event: &EnvironmentEvent,
    ) -> Option<Arc<EnvironmentState>> {
        self.reduce(&current, event).map(Arc::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wem0x01_protocol::CompositorEvent;

    #[test]
    fn compositor_event_advances_generation() {
        let reducer = StateReducer;
        let current = EnvironmentState {
            generation: 7,
            compositor: "hyprland".into(),
            ..Default::default()
        };
        let event = EnvironmentEvent::Compositor(CompositorEvent {
            compositor: "niri".into(),
            name: "workspace".into(),
            data: "1".into(),
        });

        let next = reducer.reduce(&current, &event).expect("state transition");
        assert_eq!(next.generation, 8);
        assert_eq!(next.compositor, "niri");
    }

    #[test]
    fn snapshot_is_authoritative() {
        let reducer = StateReducer;
        let current = EnvironmentState::default();
        let snapshot = EnvironmentState {
            generation: 42,
            compositor: "sway".into(),
            ..Default::default()
        };

        let next = reducer
            .reduce(&current, &EnvironmentEvent::Snapshot(Arc::new(snapshot)))
            .expect("snapshot transition");
        assert_eq!(next.generation, 42);
        assert_eq!(next.compositor, "sway");
    }
}
