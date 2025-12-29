# BlockModCore
~~mc-like not-game, you need to install mods to play~~  
just 3d chunk and cam control

tested on NixOS (2025-12-28) and a little bit on W11


## Dependencies

- gcc
- alsa-lib.dev
- udev.dev
- wayland.dev


## Environment variables

set PKG_CONFIG_PATH and LD_LIBRARY_PATH if needed,  
my nixos home-manager configuration piece - [home.nix](./.doc/home.nix) for it


## Build and run

```bash
git clone https://github.com/blockmodcore/bmc.git
cd bmc
```

### dev
```bash
cargo run --profile=dev
```
> [!NOTE]
> Bevy can write some errors in terminal, if window started and you can control - just ignore the same errors.

### release
```bash
cargo build --profile=release
```

## License
[AGPL LICENSE](./LICENSE)

If anyone wants a personal license, please email at george.noise.dev+bmclicense@gmail.com .  
Indie developers are welcome!


## Author
- **Name**: George Noise
- **Nickname**: VulpesDust
- **Email**: george.noise.dev+github@gmail.com
- **Alternate Email**: vulpesdust+github@gmail.com
- **GitHub**: [George-Noise](https://github.com/George-Noise)
