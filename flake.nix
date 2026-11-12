{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
      };
      
      # Libraries required at runtime by winit / wgpu / OpenGL on Wayland & X11
      runtimeLibs = with pkgs; [
        wayland
        libxkbcommon
        libGL
        vulkan-loader
        # Optional: include X11 fallback libraries just in case
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
      ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          toolchain
          pkgs.pkg-config
          pkgs.cargo-watch
        ] ++ runtimeLibs;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
      };
    };
}
