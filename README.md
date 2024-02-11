# Invaders

Invaders is an open source terminal arcade game with audio, based off of the "Space Invaders" classic arcade game.

### Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev pkg-config
```
**Arch Linux**

```bash
sudo pacman -S alsa-lib pkgconf libx11
```
You will also need `pipewire-alsa` or `pulseaudio-alsa` depending on the sound server you are using.

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).
