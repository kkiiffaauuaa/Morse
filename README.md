<div align="center">
    <img src="https://raw.githubusercontent.com/teacond/Morse/main/data/icons/hicolor/scalable/apps/io.github.teacond.Morse.svg" width="300"></img>
</div>
<br>
<br>

<div align="center">
    <a href="https://github.com/teacond/Morse/actions/workflows/build.yml">
        <img alt="CI" src="https://github.com/teacond/Morse/actions/workflows/build.yml/badge.svg"></img>
    </a>
</div>

# Morse

Morse is an open-source program for learning Morse code and training High Speed Telegraphy skills written in Rust language using GTK4 and Adwaita.

## Installation

### Linux (Flathub)

The recommended way to install Morse is using the flatpak package

<a href="https://flathub.org/apps/io.github.teacond.Morse">
  <img src="https://flathub.org/api/badge?svg&locale=en" alt="Download on Flathub">
</a>
<br>

### Arch Linux (AUR)

You can also install Morse with the AUR package

[![AUR badge](https://img.shields.io/aur/version/morse-git?style=flat&label=morse-git)](https://aur.archlinux.org/packages/morse-git)

Please see [the Arch Wiki](https://wiki.archlinux.org/title/Arch_User_Repository#Installing_and_upgrading_packages) for more information

## Building

**Requirements:**

* Flatpak
* Flatpak-Builder

Add the `flathub` repo:

```bash
flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo
```

Build and install a Flatpak package:

```bash
flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir build-aux/flatpak/io.github.teacond.Morse.json
```

Run the app:

```bash
flatpak run io.github.teacond.Morse
```

## Translations

If you'd like to help translating Morse into your language, please head over
to [Weblate](https://hosted.weblate.org/projects/morse-app/).

<a href="https://hosted.weblate.org/engage/morse-app/">
<img src="https://hosted.weblate.org/widget/morse-app/app/svg-badge.svg" alt="Translation status" />
</a>

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Please see COPYING file in the root of this repository for the complete license
text. Alternatively see
[the official license](https://www.gnu.org/licenses/gpl-3.0.html) as written
by the Free Software Foundation.
