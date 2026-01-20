---
applyTo: "assets/**,PKGBUILD,cargo.toml,LICENSE.**,README.md"
excludeAgent: ["code-review"]
---

# AGENT RULE — Release Packaging
## Supported Package Formats

* **Only** the following packaging formats are supported:

  * Arch Linux package (`PKGBUILD`)
  * Cargo package (`crates.io`)
* Other packaging formats **must not** be introduced or modified.

## Arch Linux Packaging Rules (`PKGBUILD`)

* `PKGBUILD` **must** match the current project release version exactly.
* All runtime and build dependencies **must** be explicitly declared.
* The file **must** follow Arch Linux packaging standards.
* The following metadata **must** be present and accurate:

  * `pkgname`
  * `pkgver`
  * `pkgrel`
  * `pkgdesc`
  * `arch`
  * `license`
  * `url`
  * `source`
* The package **must** build and install successfully using:

  ```sh
  makepkg
  ```

## Cargo Packaging Rules (`crates.io`)

* `Cargo.toml` **must** contain accurate metadata:

  * `name`
  * `version`
  * `authors`
  * `license`
  * `description`
  * `repository`
* Dependencies **must** be intentional and compatible.
* `README.md` **must** be included and referenced from `Cargo.toml`.
* The package **must** pass validation using:

  ```sh
  cargo package
  ```
* Versioning and dependency management **must** follow Cargo best practices.

## Licensing Rules

* All applicable `LICENSE` files **must** be included in the repository and in release artifacts.
* The declared license **must** accurately reflect the project’s licensing terms.
* All third-party assets and dependencies **must** be license-compatible.
* Required attributions and license notices **must** be preserved.

## Asset Rules

* All required assets **must** reside under the `assets/` directory.
* Required runtime assets **must not** be omitted from release packages.
* Assets **should** be optimized for size and performance.
* The asset directory structure **must** remain clear and intentional.

## Documentation Rules

* `README.md` **must** include:

  * Installation instructions
  * Usage information
  * Project purpose
* Documentation **must** be updated to reflect packaging or release changes.
* Additional documentation **must** be included when required for users or contributors.

### Notes

* These rules apply **only** to release packaging scope.
* Any change violating a **MUST / MUST NOT** rule is considered **release-blocking**.
