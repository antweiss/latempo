# CI/CD Pipeline Documentation

## Overview

Latempo uses GitHub Actions for comprehensive CI/CD automation across all development phases. The pipeline is designed to grow with the project, with placeholder jobs for future implementation phases.

## Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     GitHub Repository                            │
└────────────────────────┬────────────────────────────────────────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
          ▼              ▼              ▼
    ┌──────────┐   ┌──────────┐   ┌──────────┐
    │   Push   │   │    PR    │   │ Schedule │
    └────┬─────┘   └────┬─────┘   └────┬─────┘
         │              │              │
         └──────────────┼──────────────┘
                        │
         ┌──────────────┼──────────────┐
         │              │              │
         ▼              ▼              ▼
    ┌─────────┐   ┌─────────┐   ┌─────────┐
    │   CI    │   │  PR     │   │ Nightly │
    │ Build   │   │ Checks  │   │  Build  │
    └────┬────┘   └────┬────┘   └────┬────┘
         │              │              │
         └──────────────┼──────────────┘
                        │
         ┌──────────────┼──────────────┐
         │              │              │
         ▼              ▼              ▼
    ┌─────────┐   ┌─────────┐   ┌─────────┐
    │Security │   │Coverage │   │ Deploy  │
    └─────────┘   └─────────┘   └─────────┘
```

## Workflows

### 1. Main CI/CD Pipeline (`ci.yml`)

**Trigger:** Push to `main`, `develop`, or `claude/**` branches; Pull requests

**Jobs:**

#### Build and Test
- **Matrix:** Ubuntu, macOS
- **Rust Version:** Stable
- **Steps:**
  - Code checkout
  - Rust toolchain setup
  - Dependency caching (registry, git, build)
  - Format check (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Build (debug and release)
  - Test execution with mock credentials

**Cache Strategy:**
```yaml
~/.cargo/registry → cargo-registry-{lock-hash}
~/.cargo/git      → cargo-git-{lock-hash}
target/           → cargo-build-target-{lock-hash}
```

#### Code Quality
- Documentation build verification
- Clippy with all features
- Format checking
- Warning-as-error enforcement

#### Security Audit
- `cargo-audit` for dependency vulnerabilities
- Continuous monitoring of security advisories

#### Coverage
- `cargo-tarpaulin` for coverage generation
- Codecov integration (when configured)
- Minimum coverage tracking (future)

#### Phase 2-4 Jobs (Placeholders)
- **Phase 2:** AI analysis integration tests
- **Phase 3:** Python integration tests
- **Phase 4:** AWS deployment automation

#### Release
- **Trigger:** Git tags matching `v*`
- Binary compilation for multiple platforms
- GitHub Release creation with artifacts

### 2. Pull Request Checks (`pull-request.yml`)

**Trigger:** PR opened, synchronized, or reopened

**Purpose:** Fast feedback loop for contributors

**Jobs:**

#### Quick Validation
- Format and clippy checks
- Compilation verification
- Unit test execution

#### PR Size Check
- Warns if PR changes > 50 files
- Warns if PR changes > 1000 lines
- Encourages smaller, reviewable PRs

#### TODO Comments
- Scans for new TODO comments
- Reports count and locations
- Non-blocking, informational only

#### Documentation Check
- Verifies `cargo doc` builds cleanly
- Checks for undocumented public items
- Enforces documentation standards

#### Auto Labeling
- Labels based on file paths
- Helps with organization and filtering
- See `.github/labeler.yml` for rules

### 3. Nightly Builds (`nightly.yml`)

**Trigger:** Daily at 2 AM UTC; Manual dispatch

**Purpose:** Early detection of issues with nightly Rust

**Jobs:**

#### Nightly Rust Build
- Tests against latest nightly compiler
- Non-blocking (continue-on-error: true)
- Helps prepare for future Rust versions

#### Outdated Dependencies
- Runs `cargo-outdated`
- Creates issues automatically when outdated deps found
- Deduplicates issues

#### Performance Benchmarks (Placeholder)
- Future: Performance regression testing
- Benchmark result tracking
- Historical comparison

### 4. Security Scanning (`security.yml`)

**Trigger:** Push to main/develop; PRs; Weekly schedule; Manual

**Jobs:**

#### Cargo Security Audit
- `cargo-audit` for known vulnerabilities
- `cargo-deny` for policy enforcement
- Fails on high-severity issues

#### Dependency Review
- Analyzes new dependencies in PRs
- Checks for license compatibility
- Severity-based failure (moderate+)
- Blocks GPL-2.0, GPL-3.0 licenses

#### Secret Scanning
- TruffleHog for credential detection
- Scans entire git history
- Prevents credential leaks

#### CodeQL Analysis
- Static application security testing
- Python support (Rust when available)
- Security vulnerability detection

#### Semgrep SAST
- Rule-based code scanning
- Auto-configuration for language
- Detects common security patterns

#### License Compliance
- `cargo-license` for license inventory
- Blocks restrictive licenses
- Generates license report

#### SBOM Generation
- Software Bill of Materials
- Attached to releases
- Supply chain transparency

## Phase-Based Pipeline Evolution

### Phase 1: Foundation (Current) ✅

**Active Workflows:**
- ✅ Build and test
- ✅ Code quality checks
- ✅ Security auditing
- ✅ Coverage tracking
- ✅ PR validation
- ✅ Nightly builds

**Infrastructure:**
- ✅ Dependency caching
- ✅ Multi-platform testing
- ✅ Automated labeling
- ✅ Dependabot configuration

### Phase 2: Core Logic (Planned)

**New Jobs:**
```yaml
phase2-tests:
  - AI integration tests with real Anthropic API
  - DSL compilation validation
  - Template rendering tests
  - Mock code generation tests
```

**Requirements:**
- `ANTHROPIC_API_KEY` secret
- Extended timeout for AI calls
- Artifact storage for generated code

### Phase 3: Testing Infrastructure (Planned)

**New Jobs:**
```yaml
phase3-integration:
  - Python environment setup
  - Dry-run execution tests
  - Mock data generation validation
  - Trace collector tests
  - Cost estimation accuracy
```

**Requirements:**
- Python 3.11+ setup
- Temporal test environment
- Python dependency caching

### Phase 4: Deployment (Planned)

**New Jobs:**
```yaml
phase4-deploy:
  - Terraform plan/apply
  - Docker image builds
  - ECS deployment
  - Integration tests against deployed services
  - Rollback on failure
```

**Requirements:**
- AWS credentials (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
- Terraform backend configuration
- Docker registry access
- Environment-specific secrets

## Secrets Management

### Required Secrets

**Phase 1:**
- None (uses mock credentials)

**Phase 2:**
```
ANTHROPIC_API_KEY - For AI analysis tests
```

**Phase 3:**
```
ANTHROPIC_API_KEY - For AI tests
TEMPORAL_* - Temporal Cloud credentials (optional)
```

**Phase 4:**
```
AWS_ACCESS_KEY_ID - AWS deployment
AWS_SECRET_ACCESS_KEY - AWS deployment
ANTHROPIC_API_KEY - Full integration tests
SLACK_TEST_TOKEN - Slack integration tests
AMADEUS_API_KEY - Flight API tests
AMADEUS_API_SECRET - Flight API tests
```

**Optional:**
```
CODECOV_TOKEN - Coverage reporting
SEMGREP_APP_TOKEN - Enhanced SAST scanning
```

### Setting Secrets

```bash
# Via GitHub CLI
gh secret set ANTHROPIC_API_KEY --body "sk-ant-..."

# Via GitHub UI
Settings → Secrets and variables → Actions → New repository secret
```

## Caching Strategy

### Cargo Dependencies

**Registry Cache:**
```yaml
path: ~/.cargo/registry
key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
```

**Git Dependencies:**
```yaml
path: ~/.cargo/git
key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}
```

**Build Artifacts:**
```yaml
path: target
key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
```

### Python Dependencies (Phase 3+)

```yaml
path: ~/.cache/pip
key: ${{ runner.os }}-pip-${{ hashFiles('**/requirements.txt') }}
```

## Artifact Storage

### Coverage Reports
- **Format:** Cobertura XML
- **Upload:** Codecov
- **Retention:** Indefinite

### License Reports
- **Format:** JSON
- **Upload:** GitHub Artifacts
- **Retention:** 90 days

### SBOM
- **Format:** JSON
- **Upload:** GitHub Releases
- **Retention:** Indefinite

### Release Binaries
- **Platforms:** Linux (amd64), macOS (future)
- **Upload:** GitHub Releases
- **Retention:** Indefinite

## Notifications

### Failure Notifications
- **Job:** `notify-failure`
- **Trigger:** Any main job fails
- **Action:** Log to workflow run (extend for Slack/email)

### Issue Creation
- **Outdated Dependencies:** Auto-creates issue
- **Security Vulnerabilities:** Reported in Security tab
- **License Violations:** Workflow fails

## Best Practices

### For Contributors

1. **Pre-push Checks:**
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features
   cargo test --all-features
   ```

2. **Keep PRs Small:**
   - < 50 files changed
   - < 1000 lines changed
   - Single logical change

3. **Write Tests:**
   - Aim for >70% coverage
   - Test new functionality
   - Update tests when changing behavior

4. **Document Code:**
   - Public items must be documented
   - Use `cargo doc` to verify
   - Include examples in docs

### For Maintainers

1. **Merge Strategy:**
   - Squash commits for cleaner history
   - Ensure all checks pass
   - Review coverage reports

2. **Release Process:**
   ```bash
   # Create and push tag
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0

   # CI automatically:
   # 1. Builds release binaries
   # 2. Generates SBOM
   # 3. Creates GitHub Release
   ```

3. **Security Updates:**
   - Review Dependabot PRs promptly
   - Address security advisories within 7 days
   - Test dependency updates thoroughly

## Troubleshooting

### Common Issues

**Cache Corruption:**
```bash
# In GitHub UI: Actions → Caches → Delete corrupted cache
# CI will rebuild on next run
```

**Flaky Tests:**
```yaml
# Add retry logic in workflow
- name: Run tests
  uses: nick-invision/retry@v2
  with:
    timeout_minutes: 10
    max_attempts: 3
    command: cargo test --all-features
```

**Rate Limiting:**
```yaml
# Add delays between API calls in tests
# Use mocks for external services
# Configure rate limiting in test fixtures
```

## Metrics and Monitoring

### Key Metrics

- **Build Success Rate:** Target >95%
- **Average Build Time:** Target <10 minutes
- **Test Coverage:** Target >70% (Phase 1), >80% (Phase 2+)
- **Security Vulnerabilities:** Target 0 high/critical

### Dashboards

- **Actions:** https://github.com/antweiss/latempo/actions
- **Security:** https://github.com/antweiss/latempo/security
- **Insights:** https://github.com/antweiss/latempo/pulse

## Future Enhancements

### Phase 5+

- **Multi-platform Binaries:** Windows, Linux ARM, macOS ARM
- **Container Scanning:** Trivy, Grype for Docker images
- **Performance Testing:** Continuous benchmarking with criterion
- **Chaos Engineering:** Resilience testing for deployed services
- **Blue-Green Deployments:** Zero-downtime releases
- **Canary Releases:** Gradual rollout with monitoring
- **Automatic Rollbacks:** Based on error rates, metrics

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI Best Practices](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book - Continuous Integration](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
