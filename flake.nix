{
    description = "
      ```bash
      nix develop
      ```
    ";


    inputs = {
        # nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        # flake-utils.url = "github:numtide/flake-utils";
    };


    outputs = {
        self,
        nixpkgs,
        # flake-utils,
        ...
        # flake-utils.lib.eachDefaultSystem (system
    } @inputs
    
      : let
          system = "x86_64-linux"; # your version
          # pkgs = import nixpkgs { inherit system; };
          pkgs = nixpkgs.legacyPackages.${system};
      in 
      rec {
        devShells.${system}.default = pkgs.mkShell {

          buildInputs = with pkgs; [

            # build
            rustup
            pkg-config
            wayland
            wayland.dev
            alsa-lib
            alsa-lib.dev
            udev
            udev.dev

            # run
            libxkbcommon
            vulkan-loader

          ];

          # вообще хз зачем мне это нейросеть говорила писать
          # nativeBuildInputs = with pkgs; [
            # pkg-config
          # ];

          # run
          LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
            libxkbcommon
            vulkan-loader
          ];

        };
      };
    # )
}