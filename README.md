# curriculum-deploy

`curriculum-deploy` is the runtime that projects a Curriculum data checkout into a consumer workspace.

It owns the generator, checker, visualizer, templates, and engine-only Nix packaging. It does not contain Curriculum skills, role data, manifests, request files, or generated consumer output.

The executable accepts exactly one inline Datom configuration. Its root operation names the mode, the external Curriculum data root, and the consumer workspace root. It reads no environment variables, current working directory, flags, or request files for configuration.

`CurriculumRequest.{Generate.{data-root workspace-root}}` writes 38 discovered
skill companions, 27 role packets, and the typed
`skills/generated-role-outputs.datom` cleanup inventory. `Check` verifies the
same projection and `Visualize` reports its discovered counts without writing.

The parent-child flow contract is Curriculum data: a parent brief supplies
`FLOW_ID`, `FLOW_DIRECTORY`, and `THREAD_ID` to each `$child-flow`; nested
children preserve the first two values. The runtime projects that contract but
does not inject identity into a vendor harness.

The external Curriculum repository is an independently pinned data input. Updating its data does not change this runtime's Rust or Nix source.

The `external-data` Nix check exercises the pinned public data input on a
remote builder. The runtime package itself remains independent of that input.
