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
curriculum-deploy 'Generate.{ «/path/to/curriculum» [ Psyche.«/path/to/psyche-skills» Mind.«/path/to/mind-skills» Field.«/path/to/field-skills» ] «/path/to/workspace» «/run/user/1001/curriculum/curriculum.sock» }'
curriculum-deploy 'Check.{ «/path/to/curriculum» [ ... ] «/path/to/workspace» «/path/to/curriculum.sock» }'
curriculum-deploy 'Visualize.{ «/path/to/curriculum» [ ... ] «/path/to/workspace» «/path/to/curriculum.sock» }'
```

The first position is the Curriculum root; the runtime reads only its
`roles.datom` there, in Curriculum's own format (`RolesDocument`, from the
`curriculum` crate). The second is the vector of aspect repositories. Each
source is declared as `Psyche`, `Mind` or `Field`. The runtime reads every
version of every module, `<kind>/<subject>/<version>.md`, from every module
kind in every aspect repository: `compensation`, `intent`, `knowledge`,
`role`, `spirit` and `vision`. A version is a canonical decimal number, `0`
or one with no leading zero. A module's kind does not decide which repository
holds it. A module's ID is `<kind>:<subject>:<version>`, the form
`roles.datom` names it by, and a file beside the subjects is not a module.
Root Markdown such as `README.md`, and directories
outside those module kinds such as `evidence`, are not skills. The third
position is the consumer workspace. The fourth is the Curriculum Nexus's
socket: every role packet is that Nexus's `ComposeRole` answer for the role,
rendered for its surface as `ProjectRoles` renders it, so the role packets
come from the Nexus's belief checkouts and the skills from the sources named
here. `Visualize` counts the roles without asking the Nexus.

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
| signal-curriculum-nexus | 0.1.0 | 4358aa1 |
| curriculum | 0.1.0 | 1b6c454 |
| ethos-zero | 27.0.0 | 8c95310 |

## Ethos declaration

The request, output, and role-packet types are declared in
`curriculum-deploy.ethos` as an ethos Library, over the Curriculum Nexus's
wire types, which it imports from `signal-curriculum-nexus`. The roles file
format is Curriculum's, declared in Curriculum's `curriculum.ethos`, and this
crate takes it from the `curriculum` crate; `ProjectRoles`
asks and reads that contract's own `Input` and `Output`. The generated Rust module
`src/generated.rs` is committed and verified fresh by a test that reads
the ethos file through the ethos-zero library and compares the emitted
output.
