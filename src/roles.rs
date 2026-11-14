//! Generate's role packets: one for every role Curriculum's roles file
//! declares, each the Curriculum Nexus's ComposeRole answer for that role,
//! written where the role's surface reads it.
use std::path::Path;

use curriculum::generated::Roles;
use signal_curriculum_nexus::{RoleSelection, Surface};

use crate::{role_projection, runtime::Error};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolePacket {
    pub path: String,
    pub text: String,
}

/// The roles file's roles, each composed by the Curriculum Nexus at a socket.
pub trait ComposesRolePackets {
    fn packets(&self, curriculum_socket: &Path) -> Result<Vec<RolePacket>, Error>;
}

impl ComposesRolePackets for Roles {
    fn packets(&self, curriculum_socket: &Path) -> Result<Vec<RolePacket>, Error> {
        self.role_definition_vector
            .iter()
            .map(|role| {
                let selection = RoleSelection {
                    role_name: role.role_name.clone(),
                    surface: role.surface.clone(),
                };
                let (_, text) = role_projection::composed(curriculum_socket, &selection)?;
                Ok(RolePacket {
                    path: destination(&role.role_name.0, &role.surface),
                    text,
                })
            })
            .collect()
    }
}

/// Where a role's surface reads its packet, relative to the workspace.
fn destination(name: &str, surface: &Surface) -> String {
    match surface {
        Surface::ClaudeAgent => format!(".claude/agents/{name}.md"),
        Surface::CodexAgent => format!(".codex/agents/{name}.toml"),
        Surface::PiAgent => format!(".pi/agents/{name}.md"),
    }
}
