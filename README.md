<div align="center">

  <a href="https://github.com/orhun/daktilo">
    <img src="assets/daktilo-logo.png" width="400">
  </a>

<h4>Turn your keyboard into a typewriter! 📇</h4>

<a href="https://github.com/orhun/daktilo/releases"><img src="https://img.shields.io/github/v/release/orhun/daktilo?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=GitHub&amp;logoColor=white" alt="GitHub Release"></a>
<a href="https://crates.io/crates/daktilo/"><img src="https://img.shields.io/crates/v/daktilo?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://codecov.io/gh/orhun/daktilo"><img src="https://img.shields.io/codecov/c/gh/orhun/daktilo?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=Codecov&amp;logoColor=white" alt="Coverage"></a>
<br>
<a href="https://github.com/orhun/daktilo/actions?query=workflow%3A%22Continuous+Integration%22"><img src="https://img.shields.io/github/actions/workflow/status/orhun/daktilo/ci.yml?branch=main&amp;style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=GitHub%20Actions&amp;logoColor=white" alt="Continuous Integration"></a>
<a href="https://github.com/orhun/daktilo/actions?query=workflow%3A%22Release%22"><img src="https://img.shields.io/github/actions/workflow/status/orhun/daktilo/release.yml?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=release" alt="Continuous Deployment"></a>
<a href="https://docs.rs/daktilo/"><img src="https://img.shields.io/docsrs/daktilo?style=flat&amp;labelColor=1D272B&amp;color=819188&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a>

<video src="https://github.com/orhun/daktilo/assets/24392180/b2b0d9c5-422f-401d-a25d-e6bc797d8976" alt="daktilo demo">

</div>

**daktilo** ("typewriter" in Turkish, pronounced _"duck-til-oh"_, derived from the Ancient Greek word [δάκτυλος](https://lsj.gr/wiki/%CE%B4%CE%AC%CE%BA%CF%84%CF%85%CE%BB%CE%BF%CF%82) for "finger") is a small command-line program that plays typewriter sounds every time you press a key. It also offers the flexibility to customize keypress sounds to your liking. You can use the built-in sound presets to create an enjoyable typing experience, whether you're crafting emails or up to some prank on your boss.

✨ Inspiration: ["Taking notes in class with my typewriter"](https://www.youtube.com/watch?v=Bs5TEuZPQl8)

Now you can recreate this moment without the actual need for a physical typewriter!

<details>
  <summary>Table of Contents</summary>

<!-- vim-markdown-toc GFM -->

- [Getting Started](#getting-started)
- [Supported Platforms](#supported-platforms)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Arch Linux](#arch-linux)
  - [Alpine Linux](#alpine-linux)
  - [MacPorts](#macports)
  - [Binary releases](#binary-releases)
  - [Build from source](#build-from-source)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Adding custom presets](#adding-custom-presets)
  - [Sound Variation](#sound-variation)
- [Similar Projects](#similar-projects)
- [Acknowledgements](#acknowledgements)
- [Donations](#donations)
- [Contributing](#contributing)
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
| `drumkit`   | dum, tss, cha! 🥁                               |

To list the presets:

```sh
daktilo --list-presets
```

To use a preset:

```sh
daktilo --preset musicbox
```

You can also use multiple presets at the same time:

```sh
# orchestra
daktilo -p default -p musicbox -p drumkit
```

To use a different output device:

```sh
daktilo --device pipewire
```

Also, you can use `--list-devices` to list the available output devices.

To variate the sounds and have a more realistic typewriter experience:

```sh
daktilo --variate-tempo 0.9,0.4 --variate-volume 0.1,0.5
```

<details>
  <summary>Spoiler warning</summary>

There are easter eggs. If that is something you do not like you can disable them either via the config file option `no_surprises = true` or using the command-line flag:

```sh
daktilo --no-surprises
```

</details>

## Supported Platforms

- [x] Linux
  - [x] X11
  - [ ] Wayland[\*](https://github.com/Narsil/rdev#linux)
- [x] Windows
- [x] MacOS

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/daktilo.svg)](https://repology.org/project/daktilo/versions)

### Cargo

**daktilo** can be installed from [crates.io](https://crates.io/crates/daktilo) using [`cargo`](https://doc.rust-lang.org/cargo/) if [Rust](https://www.rust-lang.org/tools/install) is installed.

```sh
cargo install daktilo
```

The minimum supported Rust version is `1.70.0`.

On Linux, the following packages should be installed:

- Arch Linux: `alsa-lib libxtst libxi`
- Alpine Linux: `alsa-lib-dev libxi-dev libxtst-dev`
- Debian/Ubuntu: `libasound2-dev libxi-dev libxtst-dev`

### Arch Linux

**daktilo** can be installed from the [official repositories](https://archlinux.org/packages/extra/x86_64/daktilo/) using [pacman](https://wiki.archlinux.org/title/Pacman):

```sh
pacman -S daktilo
```

### Alpine Linux

**daktilo** is available for [Alpine Edge](https://pkgs.alpinelinux.org/packages?name=daktilo&branch=edge). It can be installed via [apk](https://wiki.alpinelinux.org/wiki/Alpine_Package_Keeper) after enabling the [testing repository](https://wiki.alpinelinux.org/wiki/Repositories).

```sh
apk add daktilo
```

### MacPorts

On macOS, **daktilo** can be installed via [MacPorts](https://www.macports.org/):

```sh
sudo port install daktilo
```

More info [here](https://ports.macports.org/port/daktilo/).

### Binary releases

See the available binaries for different targets from the [releases page](https://github.com/orhun/daktilo/releases).

### Build from source

1. Clone the repository.

```sh
git clone https://github.com/orhun/daktilo && cd daktilo/
```

2. Build.

```sh
CARGO_TARGET_DIR=target cargo build --release
```

Binary will be located at `target/release/daktilo`.

## Usage

```sh
daktilo [OPTIONS]
```

**Options**:

```sh
-v, --verbose                                    Enables verbose logging [env: VERBOSE=]
-p, --preset [<PRESET>...]                       Sets the name of the sound preset to use [env: PRESET=]
-l, --list-presets                               Lists the available presets
    --list-devices                               Lists the available output devices
-d, --device <DEVICE>                            Sets the device for playback [env: DAKTILO_DEVICE=]
-c, --config <PATH>                              Sets the configuration file [env: DAKTILO_CONFIG=]
-i, --init                                       Writes the default configuration file
    --variate-volume <PERCENT_UP[,PERCENT_DOWN]> Variate volume +/- in percent [env: DAKTILO_VOLUME=]
    --variate-tempo <PERCENT_UP[,PERCENT_DOWN]>  Variate tempo +/- in percent [env: DAKTILO_TEMPO=]
-h, --help                                       Print help (see more with '--help')
-V, --version                                    Print version
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

The configuration file consists of an array of `sound_preset` entries.

To define an array in TOML, you can create different sections as follows:

```toml
[[sound_preset]]
name = "custom"
key_config = []

[[sound_preset]]
name = "another_custom"
key_config = []
disabled_keys = []
variation = { volume: [0.1, 0.1], tempo: [0.05, 0.05] }
```

As shown above, `sound_preset` consists of 2 entries:

- `name`: The name of the preset. It will be used in conjunction with `--preset` flag. e.g. `--preset custom`
- `key_config`: An array of key press/release events for assigning audio files to the specified keys. It can also be used to control the volume etc.
- `disabled_keys`: An array of keys that will not be used for playback.
- `variation`: Variate the sound on each event for `key_config`s that do not specify variations[\*](#sound-variation)

<details>
  <summary>Click for the <a href="https://docs.rs/rdev/latest/rdev/enum.Key.html">list of available keys</a>.</summary>

`Alt`, `AltGr`, `Backspace`, `CapsLock`, `ControlLeft`, `ControlRight`, `Delete`, `DownArrow`, `End`, `Escape`, `F1`, `F10`, `F11`, `F12`, `F2`, `F3`, `F4`, `F5`, `F6`, `F7`, `F8`, `F9`, `Home`, `LeftArrow`, `MetaLeft`, `MetaRight`, `PageDown`, `PageUp`, `Return`, `RightArrow`, `ShiftLeft`, `ShiftRight`, `Space`, `Tab`, `UpArrow`, `PrintScreen`, `ScrollLock`, `Pause`, `NumLock`, `BackQuote`, `Num1`, `Num2`, `Num3`, `Num4`, `Num5`, `Num6`, `Num7`, `Num8`, `Num9`, `Num0`, `Minus`, `Equal`, `KeyQ`, `KeyW`, `KeyE`, `KeyR`, `KeyT`, `KeyY`, `KeyU`, `KeyI`, `KeyO`, `KeyP`, `LeftBracket`, `RightBracket`, `KeyA`, `KeyS`, `KeyD`, `KeyF`, `KeyG`, `KeyH`, `KeyJ`, `KeyK`, `KeyL`, `SemiColon`, `Quote`, `BackSlash`, `IntlBackslash`, `KeyZ`, `KeyX`, `KeyC`, `KeyV`, `KeyB`, `KeyN`, `KeyM`, `Comma`, `Dot`, `Slash`, `Insert`, `KpReturn`, `KpMinus`, `KpPlus`, `KpMultiply`, `KpDivide`, `Kp0`, `Kp1`, `Kp2`, `Kp3`, `Kp4`, `Kp5`, `Kp6`, `Kp7`, `Kp8`, `Kp9`, `KpDelete`, `Function`, `Unknown`

</details>

As an example, here is how you can configure `key_config`:

```toml
key_config = [
  { event = "press", keys = "Return", files = [{ path = "ding.mp3", volume = 1.0 }] },
]
```

- `event`: "press" or "release"
- `keys`: A regular expression (regex) for matching the keys.
- `files`: An array of files.
  - `path`: The absolute path of the file. If the file is embedded in the binary (i.e. if it is inside `sounds/` directory) then it is the name of the file without full path.
  - `volume`: The volume of the sound. The value 1.0 is the "normal" volume (unfiltered input). Any value other than 1.0 will multiply each sample by this value.
- `variation`: Variate the sound on each `event`[\*](#sound-variation)

If you have defined multiple files for a key event, you can also specify a strategy for how to play them:

```toml
key_config = [
  { event = "press", keys = ".*", files = [{ path = "1.mp3" }, { path = "2.mp3" }], strategy = "random" },
]
```

Currently supported strategies are:

- `strategy = "random"`: pick a random file from the list and play it.
- `strategy = "sequential"`: play the files sequentially.

Here is how you can combine everything together:

```toml
[[sound_preset]]
# Custom sound preset named "custom"
name = "custom"

# Key configurations for various events
key_config = [
  # When a key starting with "Key" is pressed, play 1.mp3, 2.mp3, and 3.mp3 sequentially
  { event = "press", keys = "Key*", files = [
    { path = "1.mp3" },
    { path = "2.mp3" },
    { path = "3.mp3" },
  ], strategy = "sequential" },

  # When a key starting with "Key" is released, play 4.mp3
  { event = "release", keys = "Key*", files = [
    { path = "4.mp3" },
  ] },

  # When a key starting with "Num" is pressed, play num.mp3 at a very high volume (10.0)
  { event = "press", keys = "Num*", files = [
    { path = "num.mp3", volume = 10.0 },
  ] },

  # When any key is pressed, play a random sound from cat.mp3, dog.mp3, or bird.mp3
  { event = "press", keys = ".*", files = [
    { path = "cat.mp3" },
    { path = "dog.mp3" },
    { path = "bird.mp3" },
  ], strategy = "random", variation = { volume: [0.1, 0.1], tempo: [0.05, 0.05] } },
]

# Disabled keys that won't trigger any sound events
disabled_keys = ["CapsLock", "NumLock"]
```

### Sound Variation

To make the keyboard sounds more varied it is possible to variate both volume and playback speed (the later also varies the pitch).

Values are in percent, where the first value determines the maximum increase and the second the maximum decrease. The actual value is determined randomly on each keypress.

- If command line arguments or environment variables are set configurations made in the presets are overridden.
  - Values need to be separated by a ",".
  - If only one value is supplied it is used for both increase and decrease
- If a `key_config` is set the preset values are overridden.
- The configuration on a preset applies to all `key_config`'s that do not have any values set.

## Similar Projects

- [`bucklespring`](https://github.com/zevv/bucklespring): Nostalgia bucklespring keyboard sound
- [`selectric-mode`](https://github.com/rbanffy/selectric-mode): Make your Emacs sound like a proper typewriter

## Acknowledgements

Huge thanks to [H. Arda Güler](https://github.com/arda-guler/) for giving me the idea for this project, sharing the inspiration behind it and implementing the first iteration in Python.

Kudos! 👾

## Donations

[![Support me on GitHub Sponsors](https://img.shields.io/github/sponsors/orhun?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](https://github.com/sponsors/orhun)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpatrons&style=flat&logo=Patreon&labelColor=1D272B&color=819188&logoColor=white)](https://patreon.com/join/orhunp)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpledges&style=flat&logo=Patreon&labelColor=1D272B&color=819188&logoColor=white&label=)](https://patreon.com/join/orhunp)

If you find **daktilo** and/or other projects on my [GitHub](https://github.com/orhun) useful, consider supporting me on [GitHub Sponsors](https://github.com/sponsors/orhun) or [becoming a patron](https://www.patreon.com/join/orhunp)!

## Contributing

See our [Contribution Guide](./CONTRIBUTING.md) and please follow the [Code of Conduct](./CODE_OF_CONDUCT.md) in all your interactions with the project.

Also, see how you can add new presets [here](CONTRIBUTING.md#how-to-add-new-presets).

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-APACHE)

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2023, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
