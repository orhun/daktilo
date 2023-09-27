<div align="center">

  <a href="https://github.com/orhun/daktilo">
    <img src="assets/daktilo-logo.jpeg" width="400">
  </a>

<h4>Turn your keyboard into a typewriter! 📇</h4>

<a href="https://github.com/orhun/halp/releases"><img src="https://img.shields.io/github/v/release/orhun/halp?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=GitHub&amp;logoColor=white" alt="GitHub Release"></a>
<a href="https://crates.io/crates/halp/"><img src="https://img.shields.io/crates/v/halp?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://codecov.io/gh/orhun/halp"><img src="https://img.shields.io/codecov/c/gh/orhun/halp?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=Codecov&amp;logoColor=white" alt="Coverage"></a>
<br>
<a href="https://github.com/orhun/halp/actions?query=workflow%3A%22Continuous+Integration%22"><img src="https://img.shields.io/github/actions/workflow/status/orhun/halp/ci.yml?branch=main&amp;style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=GitHub%20Actions&amp;logoColor=white" alt="Continuous Integration"></a>
<a href="https://github.com/orhun/halp/actions?query=workflow%3A%22Continuous+Deployment%22"><img src="https://img.shields.io/github/actions/workflow/status/orhun/halp/cd.yml?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=deploy" alt="Continuous Deployment"></a>
<a href="https://hub.docker.com/r/orhunp/halp"><img src="https://img.shields.io/github/actions/workflow/status/orhun/halp/docker.yml?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;label=docker&amp;logo=Docker&amp;logoColor=white" alt="Docker Builds"></a>
<a href="https://docs.rs/halp/"><img src="https://img.shields.io/docsrs/halp?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a>

</div>

**daktilo** ("typewriter" in Turkish, pronounced _"duck-til-oh"_) is a small command-line program that plays typewriter sounds every time you press a key. It also offers the flexibility to customize keypress sounds to your liking. You can use the built-in sound presets to create an enjoyable typing experience, whether you're crafting emails or up to some prank on your boss.

✨ Inspiration: ["Taking notes in class with my typewriter"](https://www.youtube.com/watch?v=Bs5TEuZPQl8)

Now you can recreate this moment without the actual need for a physical typewriter!

<details>
  <summary>Table of Contents</summary>

<!-- vim-markdown-toc GFM -->

- [Getting Started](#getting-started)
- [Installation](#installation)
  - [Cargo](#cargo)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Adding custom presets](#adding-custom-presets)
- [Acknowledgements](#acknowledgements)
- [Donations](#donations)
- [License](#license)
- [Copyright](#copyright)

<!-- vim-markdown-toc -->

</details>

## Getting Started

Simply run `daktilo` for the classic typewriter effect.

There are also different presets available:

| Preset Name | Description                                     |
| ----------- | ----------------------------------------------- |
| `default`   | the classic typewriter effect                   |
| `basic`     | an alternative and more basic typewriter effect |
| `musicbox`  | plays random notes like a music box             |
| `ducktilo`  | quack quack 🦆                                  |

To list the presets:

```sh
daktilo --list
```

To use a preset:

```sh
daktilo --preset musicbox
```

## Installation

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/daktilo.svg)](https://repology.org/project/daktilo/versions)

</details>

### Cargo

**daktilo** can be installed from [crates.io](https://crates.io/crates/daktilo):

```sh
cargo install daktilo
```

The minimum supported Rust version is `1.70.0`.

## Usage

```sh
daktilo [OPTIONS]
```

**Options**:

```sh
-v, --verbose          Enables verbose logging [env: VERBOSE=]
-p, --preset <PRESET>  Sets the name of the sound preset to use [env: PRESET=]
-l, --list             Lists the available presets
-c, --config <PATH>    Sets the configuration file [env: DAKTILO_CONFIG=]
-i, --init             Writes the default configuration file
-h, --help             Print help
-V, --version          Print version
```

## Configuration

**daktilo** can be configured with a configuration file using the [TOML](https://en.wikipedia.org/wiki/TOML) format.

The path of the configuration file can be specified via `--config` argument or `DAKTILO_CONFIG` environment variable.

It can also be placed in one of the following global locations:

- `<config_dir>` `/` `daktilo.toml`
- `<config_dir>` `/` `daktilo/daktilo.toml`
- `<config_dir>` `/` `daktilo/config`

`<config_dir>` depends on the platform as shown in the following table:

| Platform | Value                                 | Example                                  |
| -------- | ------------------------------------- | ---------------------------------------- |
| Linux    | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/orhun/.config                      |
| macOS    | `$HOME`/Library/Application Support   | /Users/Orhun/Library/Application Support |
| Windows  | `{FOLDERID_RoamingAppData}`           | C:\Users\Orhun\AppData\Roaming           |

See [daktilo.toml](config/daktilo.toml) for the default configuration options.

You can also create the default configuration file in the current directory with `--init` flag:

```sh
daktilo --init
```

### Adding custom presets

TODO

## Acknowledgements

Huge thanks to [H. Arda Güler](https://github.com/arda-guler/) for giving me the idea for this project, sharing the inspiration behind it and implementing the first iteration in Python.

Kudos! 👾

## Donations

[![Support me on GitHub Sponsors](https://img.shields.io/github/sponsors/orhun?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](https://github.com/sponsors/orhun)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpatrons&style=flat&logo=Patreon&labelColor=1D272B&color=819188&logoColor=white)](https://patreon.com/join/orhunp)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpledges&style=flat&logo=Patreon&labelColor=1D272B&color=819188&logoColor=white&label=)](https://patreon.com/join/orhunp)

If you find **daktilo** and/or other projects on my [GitHub](https://github.com/orhun) useful, consider supporting me on [GitHub Sponsors](https://github.com/sponsors/orhun) or [becoming a patron](https://www.patreon.com/join/orhunp)!

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-APACHE)

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2023, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
