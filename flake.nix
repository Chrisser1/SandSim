{
  description = "SandSim Native Bevy Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;

    runtimeLibs = with pkgs; [
      vulkan-loader
      libxkbcommon
      wayland
      alsa-lib
      udev
      libX11
      libXcursor
      libXi
      libXrandr
      shaderc
    ];
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      # nativeBuildInputs is for tools executed on the build machine (your host)
      nativeBuildInputs = with pkgs; [
        pkg-config
        cargo 
        rustc 
        rustfmt 
        clippy 
        rust-analyzer
        cmake
      ];

      # buildInputs is for libraries your code links against
      buildInputs = runtimeLibs;

      env = {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"
      '';
    };
  };
}