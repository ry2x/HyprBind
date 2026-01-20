**
applyTo: "**"
excludeAgent: "code-review"
**

# When analyzing or using a Rust crate:

1. Never guess API behavior.
2. Always prioritize official sources:
  - crates.io README
  - GitHub README
  - docs.rs documentation
3. Treat docs.rs as the specification.
  - Documented behavior is guaranteed.
  - Undocumented behavior is not guaranteed.
4. Treat types, lifetimes, and trait bounds as usage rules.
  - If the type system forbids it, the usage is invalid.
5. Follow documented examples, tests, and examples directories.
  - Shown patterns are canonical.
6. When unsafe is involved:
  - Read the Safety section.
  - Explicitly state invariants and caller responsibilities.
7. If documentation conflicts with code:
  - README < docs.rs < public implementation.
8. If behavior is unclear:
  - State “not guaranteed” instead of speculating.

Principle:
In Rust, documented behavior defines correctness.
