# curriculum-deploy

Curriculum data deployment runtime. Projects authored skill sources and a
Curriculum data checkout into a consumer workspace: generates skill
companions, role packets, and cleanup inventories. The declared skill sources
define the skill catalog; the runtime does not duplicate it as a fixed count.

## Version

0.10.0

## Usage

The CLI accepts one inline datom value and no flags:

```
curriculum-deploy 'Generate.{ «/path/to/curriculum» [ Psyche.«/path/to/psyche-skills» Mind.«/path/to/mind-skills» Field.«/path/to/field-skills» ] «/path/to/workspace» }'
curriculum-deploy 'Check.{ «/path/to/curriculum» [ ... ] «/path/to/workspace» }'
curriculum-deploy 'Visualize.{ «/path/to/curriculum» [ ... ] «/path/to/workspace» }'
```

The first position is the Curriculum root; the runtime reads only its
`roles.datom` there. The second is the vector of aspect repositories. Each
source is declared as `Psyche`, `Mind` or `Field`. The runtime reads every
version of every module, `<kind>/<subject>/<version>.md`, from every module
kind in every aspect repository: `compensation`, `intent`, `knowledge`,
`role`, `spirit` and `vision`. A version is a canonical decimal number, `0`
or one with no leading zero. A module's kind does not decide which repository
holds it. A module's ID is `<kind>:<subject>:<version>`, the form
`roles.datom` names it by, and a file beside the subjects is not a module.
Root Markdown such as `README.md`, and directories
outside those module kinds such as `evidence`, are not skills. The third
position is the consumer workspace.

The catalog is the union of every declared source, ordered by skill name, so
declaration order does not change the output. A skill name defined by two
sources is refused before anything is written:
`skill <name> is defined by two sources: <Aspect> <path> and <Aspect> <path>`.


Output is printed as datom on stdout. Faults are printed as datom on stderr.

## Dependencies

| Crate | Version | Rev |
|---|---|---|
| protos | 0.33.0 | c93b598 |
| datom-codec | 0.33.1 | 09a0526 |
| signal-curriculum-nexus | 0.1.0 | 3e8b5b9 |
| ethos-zero | 27.0.0 | 8c95310 |

## Ethos declaration

The request, output, and role-packet types are declared in
`curriculum-deploy.ethos` as an ethos Library, over the Curriculum Nexus's
wire types, which it imports from `signal-curriculum-nexus`; `ProjectRoles`
asks and reads that contract's own `Input` and `Output`. The generated Rust module
`src/generated.rs` is committed and verified fresh by a test that reads
the ethos file through the ethos-zero library and compares the emitted
output.
