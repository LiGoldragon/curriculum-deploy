# curriculum-deploy

Curriculum data deployment runtime. Projects a Curriculum data checkout into
a consumer workspace: generates skill companions, role packets, and cleanup
inventories. The authored checkout defines the skill catalog; the runtime does
not duplicate it as a fixed count.

## Version

0.6.3

## Usage

The CLI accepts one inline datom value and no flags:

```
curriculum-deploy 'Generate.{ «/path/to/curriculum» «/path/to/workspace» }'
curriculum-deploy 'Check.{ «/path/to/curriculum» «/path/to/workspace» }'
curriculum-deploy 'Visualize.{ «/path/to/curriculum» «/path/to/workspace» }'
```


Output is printed as datom on stdout. Faults are printed as datom on stderr.

## Dependencies

| Crate | Version | Rev |
|---|---|---|
| protos | 0.32.2 | 15b41da8f257 |
| datom-codec | 0.32.2 | 4dff16b4f741 |
| ethos-zero | 16.0.0 | c2653dd82adb |

## Ethos declaration

The request, output, and role-packet types are declared in
`curriculum-deploy.ethos` as an ethos Library. The generated Rust module
`src/generated.rs` is committed and verified fresh by a test that reads
the ethos file through the ethos-zero library and compares the emitted
output.
