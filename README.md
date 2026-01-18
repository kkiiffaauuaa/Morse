<div align="center">
<img src="https://raw.githubusercontent.com/teacond/Morse/main/data/icons/hicolor/scalable/apps/io.github.teacond.Morse.svg" width="300"></img>
</div><br><br>

# Morse

Morse is an open-source program for learning Morse code and training High Speed Telegraphy skills written in Rust language using GTK4 and Adwaita.

## Installation

### Arch (AUR)

Currently you can install Morse only with AUR package

- [![AUR badge](https://img.shields.io/aur/version/morse-git?style=flat&label=morse-git)](https://aur.archlinux.org/packages/morse-git)

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
flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir build-aux/io.github.teacond.Morse.Devel.json
```

Run the app:

```bash
flatpak run io.github.teacond.Morse.Devel
```

## Translations

If you'd like to help translating Morse into your language, please head over
to [Weblate](https://hosted.weblate.org/projects/morse-app/).

<a href="https://hosted.weblate.org/engage/morse-app/">
<img src="https://hosted.weblate.org/widget/morse-app/svg-badge.svg" alt="Translation status" />
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
