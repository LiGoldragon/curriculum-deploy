# Architecture

## Layers

Text -> Protoform -> Datom -> Corporal.

Realization (inbound): `delineate` -> `conceive` -> `incorporate`.
Textualization (outbound): `datomize` -> `protosize` -> `print`.

## Modules

- `curriculum-deploy.ethos` -- the type declarations.
- `src/generated.rs` -- committed output of ethos-zero; freshness-tested.
- `src/generated_ext.rs` -- Clone/Copy/PartialEq/Eq for unit enums
  (ethos-zero Library mode omits derives).
- `src/runtime.rs` -- CLI dispatch, root-head convention, Deployment logic,
  skill template rendering, Meaning-to-Text normalization.
- `src/roles.rs` -- role packet assembly from the Roles data.
- `src/main.rs` -- entry point.

## Root-head convention

Standalone datom files (roles.datom, generated-role-outputs.datom) carry
a named variant head: `Roles.{ ... }`, `GeneratedRoleOutputs.{ ... }`.
The `RootReading` and `RootWriting` traits wrap/unwrap this head in
application code; the generated Datomic impls handle the inner struct.

