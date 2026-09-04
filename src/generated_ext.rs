//! Trait implementations for generated types that the ethos-zero Library
//! emitter does not produce (it adds derives only in Signal mode).
//! This is an API deviation: see FLOW_DIRECTORY/reports/api-deviations.md.

use crate::generated::*;

macro_rules! unit_enum_traits {
    ($type:ident { $($variant:ident),+ }) => {
        impl Clone for $type {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl Copy for $type {}
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
        impl Eq for $type {}
    };
}

unit_enum_traits!(Provider { Claude, ChatGpt });
unit_enum_traits!(Permission {
    Restricted,
    Unrestricted
});
unit_enum_traits!(Effort {
    Low,
    Medium,
    High,
    Xhigh
});
unit_enum_traits!(Surface {
    ClaudeAgent,
    CodexAgent,
    PiAgent
});
