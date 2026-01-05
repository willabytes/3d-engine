with (import <nixpkgs> {});
  let
    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libxkbcommon
	  vulkan-headers
      vulkan-loader
      vulkan-tools
      wayland
    ];
  in {
    devShell = with pkgs; mkShell {
      buildInputs = [
        cargo
        rustup
        rust-analyzer
      ];
      
      RUST_LOG = "debug";
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LD_LIBRARY_PATH = libPath;
    };
  }
