# dusk-bytes

Serialization traits using const generics for fixed-size types. Workspace with two `no_std` crates: core serialization traits and a companion proc-macro for hex formatting. This is a foundational dependency for most Dusk repos — changes here ripple widely.

## Repository Map

```
bytes/
├── dusk-bytes/          # dusk-bytes — Serializable<N>, DeserializableSlice, Read/Write, hex parsing
│   ├── src/
│   │   ├── lib.rs       # Re-exports, crate attributes
│   │   ├── serialize.rs # Serializable<N>, DeserializableSlice, Read, Write traits
│   │   ├── parse.rs     # ParseHexStr trait, const hex() function
│   │   ├── errors.rs    # Error enum, BadLength/InvalidChar traits
│   │   └── primitive.rs # Serializable impls for integer primitives (u8..u128, i8..i128)
│   └── tests/
├── derive-hex/          # derive-hex — #[derive(Hex)] and #[derive(HexDebug)] proc macros
│   ├── src/
│   │   └── lib.rs       # LowerHex, UpperHex, Debug derive implementations
│   └── tests/
├── Makefile             # Build targets (run `make help`)
└── rustfmt.toml
```

## Commands

Run `make help` to see all available targets.

## Architecture

### Core Traits (`dusk-bytes`)

- **`Serializable<const N: usize>`** — defines `from_bytes(&[u8; N])` and `to_bytes() -> [u8; N]` for fixed-size serialization. The const generic `N` is the wire size.
- **`DeserializableSlice<N>`** — auto-implemented for all `Serializable<N>` types. Adds `from_slice(&[u8])` (with length check) and `from_reader<R: Read>()` (streaming).
- **`Read` / `Write`** — minimal byte-oriented IO traits (not `std::io`). `Read` is implemented for `&[u8]` (advances the slice), `Write` for `&mut [u8]`.
- **`ParseHexStr<N>`** — auto-implemented for all `Serializable<N>` types. Parses hex strings into the target type.
- **`hex::<N, M>()`** — const function for compile-time hex-to-bytes conversion.

### Proc Macros (`derive-hex`)

- **`#[derive(Hex)]`** — generates `LowerHex` and `UpperHex` implementations using the type's `to_bytes()` method.
- **`#[derive(HexDebug)]`** — generates `Hex` plus a `Debug` implementation that delegates to hex formatting.

### Key Design Points

- All serialization is little-endian (see primitive impls).
- `derive-hex` is a proc-macro crate — it cannot be built for `no_std` targets like `thumbv6m-none-eabi`. The default `dusk-bytes` `derive` feature re-exports its macros; disabling default features leaves the serialization crate dependency-free. The `no-std` Makefile target builds that minimal configuration.

## Conventions

- **`no_std`**: Both crates. Do not add `std` dependencies.
- **Edition 2024**: MSRV 1.85.
- **Wide downstream impact**: This crate is a dependency of most Dusk repos. Check `Cargo.lock` in downstream repos before releasing. See the Change Propagation table below.

## Change Propagation

| Changed | Also verify |
|---------|-------------|
| `dusk-bytes` or `derive-hex` | Most repos — check `Cargo.lock` for users. Key dependents: `bls12_381`, `jubjub`, `phoenix`, `safe`, `Poseidon252`, `merkle`, `rusk` |

## Git Conventions

- Default branch: `main`
- License: MPL-2.0

### Commit messages

Format: `<scope>: <Description>` — imperative mood, capitalize first word after colon.

**One commit per crate per concern.** Each commit touches exactly one crate and one logical concern. Never bundle changes to different crates in one commit, and don't mix unrelated changes within the same crate either.

Canonical scopes:

| Scope | Crate/Directory |
|-------|----------------|
| `dusk-bytes` | `dusk-bytes/` |
| `derive-hex` | `derive-hex/` |
| `workspace` | Root `Cargo.toml`, root Makefile |
| `ci` | `.github/workflows/` |
| `chore` | Makefile, rustfmt, etc. |

Examples:
- `dusk-bytes: Add Read impl for Vec<u8>`
- `derive-hex: Fix category slugs`
- `workspace: Update edition to 2024`

### Changelog

`derive-hex` has a `CHANGELOG.md`. Add entries under `[Unreleased]` using [keep-a-changelog](https://keepachangelog.com/) format. If a change traces to a GitHub issue, reference it as a link: `[#42](https://github.com/dusk-network/dusk-bytes/issues/42)`. Only link to GitHub issues — do not reference any other tracking system.
