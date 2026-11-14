//! A stand-in Curriculum Nexus: it listens on a socket bound before the
//! runtime starts, so the runtime connects at once, and answers each
//! ComposeRole with the composition its test gives for the selection.
#![allow(dead_code)]

use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
    thread,
};

use signal_curriculum_nexus::{
    ContextPayloadSha256, ContextText, DescriptionPayloadSha256, DescriptionText, Effort, Input,
    ModelId, Output, Permission, RoleCallers, RoleComposition, RoleSelection, RoleTools,
};

pub struct StandInCurriculum {
    pub socket: PathBuf,
}

impl StandInCurriculum {
    /// Binds `socket` and answers every connection, one ComposeRole each,
    /// for as long as the test runs.
    pub fn answering(
        socket: &Path,
        compose: impl Fn(&RoleSelection) -> Output + Send + 'static,
    ) -> Self {
        let listener = UnixListener::bind(socket).expect("stand-in Curriculum socket");
        thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.expect("runtime connects");
                let request = read_frame(&mut stream);
                let output = match rkyv::from_bytes::<Input, rkyv::rancor::Error>(&request) {
                    Ok(Input::ComposeRole(selection)) => compose(&selection),
                    _ => Output::Rejected("the stand-in answers ComposeRole alone".into()),
                };
                let response = rkyv::to_bytes::<rkyv::rancor::Error>(&output).expect("output");
                write_frame(&mut stream, &response);
            }
        });
        Self {
            socket: socket.to_path_buf(),
        }
    }

    /// Composes every role it is asked for with `composition`.
    pub fn composing_every_role(socket: &Path) -> Self {
        Self::answering(socket, |selection| {
            Output::RoleComposed(composition(selection, "Role.", "Context.\n"))
        })
    }
}

/// A composition of the selected role with a description and context text.
pub fn composition(selection: &RoleSelection, description: &str, context: &str) -> RoleComposition {
    RoleComposition {
        role_name: selection.role_name.clone(),
        surface: selection.surface.clone(),
        model_id: ModelId("sonnet".into()),
        effort: Effort::High,
        permission: Permission::Unrestricted,
        description_text: DescriptionText(description.into()),
        context_text: ContextText(context.into()),
        role_tools: RoleTools::Explicit(vec!["Read".into(), "Bash".into()]),
        role_callers: RoleCallers::Any,
        context_provenance_vector: vec![],
        description_payload_sha256: DescriptionPayloadSha256("description".into()),
        context_payload_sha256: ContextPayloadSha256("context".into()),
    }
}

fn read_frame(stream: &mut UnixStream) -> Vec<u8> {
    let mut length = [0; 4];
    stream.read_exact(&mut length).expect("frame length");
    let mut frame = vec![0; u32::from_be_bytes(length) as usize];
    stream.read_exact(&mut frame).expect("frame");
    frame
}

fn write_frame(stream: &mut UnixStream, frame: &[u8]) {
    let length = u32::try_from(frame.len()).expect("frame length");
    stream
        .write_all(&length.to_be_bytes())
        .expect("frame length");
    stream.write_all(frame).expect("frame");
}
