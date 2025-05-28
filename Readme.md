# License-RS

A Rust implementation for software license generation and validation.

## Features

- Generate cryptographic key pairs
- Create license files with expiration and machine binding
- Validate license files
- Cross-platform machine identification

## Installation

```bash
# Clone the repository
git clone https://github.com/your-repo/license-rs.git
cd license-rs

# Build the project
cargo build --release

# Run tests
cargo test
```

## Usage Examples

### Generate Key Pair

```bash
cargo run --example key_gen
```

Sample output:
```
Public key:
13 196 43 242 52 6 87 80 0 88 180 251 124 44 240 7 153 17 27 72 251 84 105 11 155 87 157 192 173 93 97 64
Secret key:
238 111 167 200 2 92 90 49 153 33 45 187 220 216 191 23 24 254 101 101 176 172 157 171 179 132 110 64 110 234 185 175 13 196 43 242 52 6 87 80 0 88 180 251 124 44 240 7 153 17 27 72 251 84 105 11 155 87 157 192 173 93 97 64
```

### Generate License

```bash
cargo run --example license_gen
```

### Check License

```bash
cargo run --example license_check
```

### Get Machine ID

```bash
cargo run --example machine_id
```

Sample output:
```
BFEBFBFF000A06552CF05D40ED86
```

## API Documentation

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please open an issue or pull request.

## License

This project is licensed under the MIT License.