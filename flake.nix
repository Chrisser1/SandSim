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
    ];
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];

      buildInputs = with pkgs; [ 
        cargo 
        rustc 
        rustfmt 
        clippy 
        rust-analyzer
      ] ++ runtimeLibs;

      env = {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"
        
        # Conditionally force Vulkan to use NVIDIA ONLY if the driver exists on this machine
        NVIDIA_ICD="/run/opengl-driver/share/vulkan/icd.d/nvidia_icd.json"
        if [ -f "$NVIDIA_ICD" ]; then
            export VK_DRIVER_FILES="$NVIDIA_ICD"
            echo "NVIDIA GPU Detected: Forcing Vulkan to use NVIDIA ICD."
        else
            echo "No NVIDIA ICD found. Letting Vulkan auto-select the GPU."
        fi

        # --- VSCODE WGSL BEVY IMPORTS AUTO-DISCOVERY ---
        mkdir -p .vscode
        
        # Find the bevy_render folder in the cargo cache (Updated for v0.18.x)
        BEVY_RENDER_DIR=$(ls -d ~/.cargo/registry/src/index.crates.io-*/bevy_render-0.18.*/src 2>/dev/null | head -n 1)
        
        if [ -n "$BEVY_RENDER_DIR" ]; then
          cat <<EOF > .vscode/settings.json
          {
            "wgsl-analyzer.customImports": {
              "bevy_render": "$BEVY_RENDER_DIR"
            },
            "wgsl-analyzer.diagnostics.nagaVersion": "main"
          }
EOF
        else
          cat <<EOF > .vscode/settings.json
          {
              "wgsl-analyzer.diagnostics.nagaVersion": "main"
          }
EOF
        fi
      '';
    };
  };
}