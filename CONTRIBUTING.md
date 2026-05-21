# Contributing to NFL

Thank you for your interest in contributing to NFL!

## How to Contribute

### 1. Reporting Issues

Found a bug? Please open an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs. actual behavior
- System information (OS, Rust version, etc.)

### 2. Feature Requests

Have an idea? Submit an RFC (Request for Comments):
1. Create an issue titled `[RFC] Feature name`
2. Describe the use case and motivation
3. Provide examples and alternatives
4. Wait for community feedback

### 3. Code Contributions

#### Setup

```bash
git clone https://github.com/refineid-ien/nfl
cd nfl
cargo build
cargo test
```

#### Process

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/my-feature
   ```
3. **Make changes**
   - Follow Rust naming conventions
   - Write tests for new functionality
   - Update documentation
4. **Test your changes**
   ```bash
   cargo test --all
   cargo clippy --all
   cargo fmt
   ```
5. **Submit a Pull Request**
   - Link to related issues
   - Describe your changes
   - Include benchmarks if performance-critical

## Coding Guidelines

### Style

- Use `cargo fmt` for formatting
- Run `cargo clippy` before submitting
- Follow Rust API guidelines

### Documentation

Every public item should have documentation:

```rust
/// Loads an NFL file from disk
///
/// # Arguments
/// * `path` - Path to the .nfl file
///
/// # Returns
/// * `NflResult<NflFile>` - Parsed NFL file or error
///
/// # Examples
/// ```
/// let file = NflLoader::load("model.nfl")?;
/// ```
pub fn load<P: AsRef<Path>>(path: P) -> NflResult<NflFile> {
    // implementation
}
```

### Testing

- Write unit tests alongside code
- Add integration tests for workflows
- Test edge cases and error conditions

```rust
#[test]
fn test_my_function() {
    let result = my_function();
    assert_eq!(result, expected);
}
```

### Performance

- Profile before and after changes
- Include benchmarks for performance-critical code
- Don't optimize prematurely

## Design Constraints

All contributions must respect NFL's core constraints:

1. **No full-model RAM loading** by default
2. **No Python dependency** in runtime
3. **No JSON-based runtime parsing** in hot path
4. **All tensor operations** must be SIMD-aware
5. **File format** must remain forward-compatible
6. **Zero-copy principle** wherever possible

## Commit Messages

Use clear, descriptive commit messages:

```
[core] Fix memory alignment issue in mmap loader

- Ensure tensors are aligned to 64-byte boundaries
- Add alignment validation tests
- Update documentation

Fixes #123
```

## Pull Request Checklist

- [ ] Tests pass: `cargo test --all`
- [ ] Code formatted: `cargo fmt`
- [ ] Linter passes: `cargo clippy --all`
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] No breaking changes (or RFC approved)

## Areas for Contribution

### High Priority

- [ ] Model conversion (GGUF, safetensors, PyTorch)
- [ ] Full inference implementation
- [ ] SIMD optimizations (AVX2, NEON)
- [ ] Performance benchmarks
- [ ] Error handling improvements

### Medium Priority

- [ ] WebAssembly support
- [ ] CLI improvements
- [ ] Documentation
- [ ] Examples and tutorials
- [ ] Test coverage

### Low Priority

- [ ] Code refactoring
- [ ] Minor optimizations
- [ ] Internal improvements

## Questions?

- Open an issue with `[Question]` prefix
- Ask in GitHub Discussions
- Check existing documentation

## License

By contributing, you agree that your code will be licensed under MIT/Apache 2.0 dual license.

---

**Happy contributing! 🎉**
