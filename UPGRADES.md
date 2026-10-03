# Upgrades

## 0.8.0

Breaking: skills come from declared skill sources, not from the Curriculum
checkout.

### What changed

- The request is `Generate.{ «/curriculum» [ Psyche.«/dir» Mind.«/dir» Field.«/dir» ] «/workspace» }`.
  Each `SkillSource` names its aspect and the directory whose `*.md` files are
  its skills. Curriculum contributes only `roles.datom`; a `skills/` directory
  left in it is read only when declared as a source.
- The catalog is the union of the sources ordered by name. A skill defined by
  two sources is refused with `skill <name> is defined by two sources: ...`,
  and nothing is written.
- Generated output for an unchanged set of skills is byte-identical to 0.7.0;
  the `three-source-parity` Nix check proves it against the 0.7.0 runtime.

To deploy: until the skills move, a consumer keeps today's catalog by
declaring Curriculum's own `skills/` directory as one source; after the move it
declares the three skill repositories instead.

## 0.7.0

Breaking: repinned to ethos-zero 16.0.0, protos 0.32.2 and datom-codec 0.32.2.

### What changed

- `src/generated.rs` is regenerated: every type derives rkyv Archive,
  Serialize, Deserialize plus Clone, Debug, PartialEq, Eq, Hash; Datomizable
  and Composing sit behind a `datom` feature.
- The crate depends on rkyv 0.8 and declares `datom`, a default feature. The
  runtime textualizes always, so datom-codec stays unconditional (with its
  `rkyv` feature); building with `--no-default-features` is unsupported.
- `Datomizable` no longer has an `Output` associated type.
- Reply text from the vertical canonical print follows protos 0.32.

To deploy: land this source and let Primary's curriculum-deploy projection
repin to it; no data migration is needed.

## 0.6.0

Breaking: the runtime now uses the final Protos, Datom, and Ethos stack.

### What changed

- Requests, outputs, role data, and cleanup inventories use generated named
  fields and the trait-borne `Potential::actualize` and
  `datomize` -> `protosize` -> `textualize` chains.
- Datom strings use guillemet delimiters. The Curriculum `roles.datom`
  migration preserves every role value while changing the delimiters required
  by the final parser, including delimiters around dotted model names.
- The old Datomic/Corporal, Protoform, Text, and Fault APIs have no runtime
  compatibility path.

## 0.4.0

Breaking: the request root is now a plain data enum.

### What changed

- The \`CurriculumRequest.{ ... }\` wrapper is removed. The request is
  \`Generate.{ /curriculum /primary }\` and nothing else.
- No Meaning-to-Text normalization: a Text position accepts only Text
  (curly-quoted or bare) and never Meaning (parenthesized). The
  Curriculum's roles.datom was migrated to canonical datom.

## 0.3.0

Port from the retired \`datom\` crate and old protos API to the
ProtoformStack train: datomic 0.8.0 and protos 0.15.0.

### What changed

- Dependency \`datom\` (github:LiGoldragon/datom) replaced by \`datomic\`
  (github:LiGoldragon/datomic).
- Dependency \`protos\` updated from bfea114c to 56c683ec.
- All hand-written DatomRealizing/DatomTextualizing impls replaced by
  generated Datomic (Corporal + datomize) impls from an ethos Library.
- Canonical print uses spaced delimiters: \`{ a b }\` not \`{a b}\`.
- generated-role-outputs.datom paths are curly-quoted when they contain
  separator characters.
- Curriculum input bumped to 143125b1 (canonical datom migration).

### How to deploy

1. Bump primary's `curriculum-deploy` flake input to the new rev.
2. Ensure primary's `curriculum` input contains the matching `roles.datom`
   delimiter migration.
3. Regenerate with
   `nix run .#generate-skills 'Generate.{ «/path/to/Curriculum» «/home/li/primary» }'`.
4. Commit and push the regenerated trees.
