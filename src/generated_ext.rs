//! Copy impls for unit enums that the Library emitter derives as
//! Clone + Debug + PartialEq + Eq but not Copy.

use crate::generated::*;

impl Copy for Provider {}
impl Copy for Permission {}
impl Copy for Effort {}
impl Copy for Surface {}
