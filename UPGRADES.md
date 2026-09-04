# Upgrades

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

1. Bump primary's \`curriculum-deploy\` flake input to the new rev.
2. Ensure primary's \`curriculum\` input is at 143125b1 or later.
3. Regenerate: \`nix run .#generate-skills 'Generate.{ /path/to/Curriculum /home/li/primary }'\`
4. Commit and push the regenerated trees.
