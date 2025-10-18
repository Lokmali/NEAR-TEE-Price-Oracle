# Contributing to NEAR TEE Oracle

Thank you for your interest in contributing to NEAR TEE Oracle! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in your interactions.

## Ways to Contribute

### 1. Report Bugs

Found a bug? Please create a GitHub issue with:
- Clear description of the bug
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, versions, etc.)
- Screenshots if applicable

### 2. Suggest Features

Have an idea? We'd love to hear it! Create an issue with:
- Clear description of the feature
- Use cases and benefits
- Possible implementation approach
- Any related examples

### 3. Improve Documentation

Documentation is crucial! You can:
- Fix typos or unclear explanations
- Add examples and tutorials
- Translate documentation
- Improve API references

### 4. Submit Code

#### Before You Start

1. Check existing issues and PRs
2. Discuss significant changes in an issue first
3. Follow our coding standards
4. Write tests for new features
5. Update documentation

#### Development Setup

```bash
# Clone the repository
git clone https://github.com/near-tee-oracle/near-tee-oracle.git
cd near-tee-oracle

# Install dependencies
# For smart contract
cd contracts/oracle
cargo build

# For oracle node
cd ../../oracle-node
cargo build

# For dashboard
cd ../dashboard
npm install

# For shade agent
cd ../shade-agent
npm install
```

#### Coding Standards

**Rust Code:**
- Follow Rust naming conventions
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write tests: `cargo test`
- Add documentation comments

**TypeScript/JavaScript:**
- Use TypeScript for type safety
- Follow ESLint rules: `npm run lint`
- Use Prettier for formatting
- Write tests with Jest
- Add JSDoc comments

**Git Commits:**
- Use clear, descriptive commit messages
- Follow conventional commits format:
  - `feat:` New feature
  - `fix:` Bug fix
  - `docs:` Documentation changes
  - `test:` Test additions/changes
  - `refactor:` Code refactoring
  - `chore:` Maintenance tasks

Example:
```
feat: add support for additional price sources

- Added support for Huobi exchange
- Implemented Huobi API client
- Added tests for Huobi integration
```

#### Pull Request Process

1. **Fork the repository** and create a new branch
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clean, documented code
   - Add tests
   - Update documentation

3. **Test your changes**
   ```bash
   # Rust tests
   cargo test
   
   # JavaScript tests
   npm test
   
   # Integration tests
   ./scripts/test-integration.sh
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Use a clear title and description
   - Reference related issues
   - Include screenshots if UI changes
   - Ensure CI passes

7. **Code Review**
   - Address review comments
   - Keep PR focused and small
   - Be patient and responsive

### 5. Review Code

Help review pull requests:
- Test the changes
- Check code quality
- Provide constructive feedback
- Approve if everything looks good

## Development Guidelines

### Smart Contract Development

```rust
// Good: Clear, documented, tested
/// Calculates the median price from multiple sources
/// 
/// # Arguments
/// * `sources` - Vector of price sources
/// 
/// # Returns
/// Median price value
fn calculate_median(&self, sources: &[PriceSource]) -> u128 {
    let mut prices: Vec<u128> = sources.iter().map(|s| s.price.0).collect();
    prices.sort();
    // ... implementation
}

#[test]
fn test_calculate_median() {
    // Test implementation
}
```

### Oracle Node Development

```rust
// Good: Error handling, logging, testing
async fn fetch_price(&self, symbol: &str) -> Result<PriceData> {
    debug!("Fetching price for {}", symbol);
    
    let mut sources = Vec::new();
    
    // Try multiple sources
    for source in &self.sources {
        match source.fetch_price(symbol).await {
            Ok(price) => sources.push(price),
            Err(e) => warn!("Source {} failed: {}", source.name(), e),
        }
    }
    
    if sources.len() < MIN_SOURCES {
        return Err(anyhow!("Insufficient sources"));
    }
    
    Ok(PriceData::aggregate(sources))
}
```

### Dashboard Development

```typescript
// Good: TypeScript, hooks, error handling
const PriceCard: React.FC<PriceCardProps> = ({ symbol }) => {
  const { data, error, isLoading } = usePrice(symbol)
  
  if (isLoading) return <Skeleton />
  if (error) return <ErrorMessage error={error} />
  if (!data) return <EmptyState />
  
  return (
    <div className="price-card">
      <h3>{symbol}</h3>
      <Price value={data.price} />
      <Confidence score={data.confidence} />
    </div>
  )
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_price_validation() {
        // Test implementation
    }
    
    #[tokio::test]
    async fn test_async_operation() {
        // Async test implementation
    }
}
```

### Integration Tests

```bash
# Run integration tests
./scripts/test-integration.sh

# Test specific component
cargo test --package oracle-node --test integration_tests
```

### End-to-End Tests

```bash
# Start test environment
./scripts/start-test-env.sh

# Run e2e tests
npm run test:e2e

# Cleanup
./scripts/stop-test-env.sh
```

## Documentation

### Code Documentation

- Add doc comments to all public APIs
- Include examples in documentation
- Document panics, errors, and safety

### User Documentation

- Update relevant guides
- Add new tutorials if needed
- Include screenshots for UI changes

## Release Process

1. Update version numbers
2. Update CHANGELOG.md
3. Create release branch
4. Test thoroughly
5. Create GitHub release
6. Deploy to production
7. Announce release

## Community

- **GitHub Discussions**: General questions and ideas
- **Discord**: Real-time chat and support
- **Twitter**: Follow [@NearTeeOracle](https://twitter.com/NearTeeOracle)
- **Email**: contributing@near-tee-oracle.io

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Featured on our website

## Questions?

- Check existing documentation
- Search GitHub issues
- Ask in Discord
- Email: support@near-tee-oracle.io

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to NEAR TEE Oracle! Every contribution, no matter how small, helps make the project better. 🚀

