{ config, pkgs, lib, ... }: {
  # ...

  home = {
    # ...

    packages = with pkgs; [
      rustup
      gcc
      pkg-config
      wayland
      wayland.dev # cause: The system library `wayland-client` required by crate `wayland-sys` was not found.
      alsa-lib
      alsa-lib.dev # cause: The system library `alsa` required by crate `alsa-sys` was not found.
      udev
      udev.dev # cause: The system library `libudev` required by crate `libudev-sys` was not found.

      libxkbcommon
    ];
  };

  # I have some troubles with Gnome, so look at bash.initExtra or do how you want
  # home.sessionVariables = {
  #   MY_VARIABLE = "my_custom_value";
  # };

  programs.bash = {
    enable = true;
    shellAliases = {
      nix-rs = "sudo nixos-rebuild switch";
      home-s = "home-manager switch";
      vim = "nvim";
    };

    # PKG_CONFIG_PATH to build and LD_LIBRARY_PATH for run
    initExtra = ''
      export PKG_CONFIG_PATH="${
        pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" (with pkgs; [
          wayland.dev
          alsa-lib.dev
          udev.dev
        ])
      }:$PKG_CONFIG_PATH"

      export LD_LIBRARY_PATH="${
        pkgs.lib.makeLibraryPath (with pkgs; [
          libxkbcommon
          vulkan-loader
          wayland
          alsa-lib
        ])
      }:$LD_LIBRARY_PATH"
    '';
  };

  # ...
}