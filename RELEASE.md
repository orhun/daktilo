# Creating a Release

The releases are automated via [`release-plz`](https://github.com/MarcoIeni/release-plz), [`cargo-dist`](https://github.com/axodotdev/cargo-dist) and [GitHub Actions](https://docs.github.com/en/actions).

1. Run `release-plz release-pr` to create a pull request representing the new release.

   - Use `release-plz update` to see the changes locally if needed.

2. After merging the release PR, `git pull` your changes and run `release-plz release`.

   - This will create a tag and update the [crates.io](https://crates.io/crates/daktilo) release.

3. Run `git push --tags` the tag.

   - This will trigger `cargo-dist` and release binaries will be built in via [release workflow](.github/workflows/release.yml).

4. Mark the package out-of-date for package managers.

5. Announce the release on social platforms. 🥳
