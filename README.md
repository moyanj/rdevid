# rdevid

A project providing device identification capabilities with multiple language bindings.

## Workspace Structure

```
.
├── rdevid/                # Core Rust library
├── bindings/              # Language bindings
│   ├── wasm/              # WebAssembly bindings
│   ├── c/                 # C bindings (FFI)
│   ├── python/            # Python bindings
│   └── node/              # Node.js bindings
├── Cargo.toml             # Workspace configuration
└── Cargo.lock             # Dependency lock file
```

## Features

- Cross-platform device identification
- Multiple language bindings (WASM, C, Python, Node.js)
- Workspace organization for modular development

## Building

```bash
# Build rust library
cd rdevid
cargo build --release 

# Build c bindings
cd bindings/c
cargo build --release

# Build python bindings
cd bindings/python
maturin build --release

# Build wasm bindings
cd bindings/wasm
wasm-pack build --release

# Build node bindings
cd bindings/node
nj-cli build --release

```

## Language Bindings Usage

### WebAssembly
```javascript
// Example usage in JavaScript
```

### C

The header file is in `target/release/rdevid.h`

```c
#include <stdio.h>
#include "rdevid.h"
int main() {
    char *id = device_id();
    char *info = device_info();
    printf("Device ID: %s\n", id);
    printf("Device Info: %s\n", info);
    free_string(id);
    free_string(info);
}
```

### Python
```python
import rdevid

print(rdevid.device_id())
print(rdevid.device_info())
```

### Node.js
```javascript
import { device_id, device_info } from 'rdevid';
console.log(device_id());
console.log(device_info());
```

## Development

To add a new binding:
1. Create a new directory under `bindings/`
2. Add the member to `Cargo.toml` workspace members
3. Implement the binding following existing patterns

## Contributing

Contributions welcome! Please:
1. Open an issue to discuss changes
2. Fork the repository and create a PR
3. Ensure all bindings are tested

## License

MIT
