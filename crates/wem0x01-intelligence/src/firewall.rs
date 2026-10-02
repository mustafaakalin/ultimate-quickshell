use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLabel {
    System,
    Trusted,
    Observed,
    Untrusted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    pub source: String,
    pub label: TrustLabel,
    pub content: String,
    pub sensitivity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizedContext {
    pub items: Vec<ContextItem>,
    pub redactions: usize,
}

#[derive(Debug, Default)]
pub struct ContextFirewall;

impl ContextFirewall {
    pub fn sanitize(&self, input: impl IntoIterator<Item = ContextItem>) -> SanitizedContext {
        let mut redactions = 0;
        let items = input
            .into_iter()
            .map(|mut item| {
                // Context is data, never executable authority. High-sensitivity
                // untrusted content is withheld from remote/cloud runtimes.
                if item.label == TrustLabel::Untrusted && item.sensitivity >= 8 {
                    item.content = "[REDACTED: untrusted sensitive context]".into();
                    redactions += 1;
                }
                item
            })
            .collect();
        SanitizedContext { items, redactions }
    }
}
