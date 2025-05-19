{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs = { nixpkgs, systems, ... }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgsFor = nixpkgs.legacyPackages;
    in {
      devShells = eachSystem (system:
        let
          pkgs = pkgsFor.${system};
          commonPkgs = with pkgs;[
              libxkbcommon
              cargo
              rustc
              wayland
              pkg-config
              glib
          ];
          # for the files application in apps/files
          fonts = with pkgs; [
            noto-fonts
            noto-fonts-emoji
            dejavu_fonts
            freefont_ttf
          ];
          dlopenLibraries = with pkgs; [
            libxkbcommon
            # GPU backend
            vulkan-loader
            # libGL

            ## required specifically by settings application
            libllvm
            libclang
            llvmPackages.libclang

            # required by launcher 
            libpulseaudio
          ];
        in {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              gst_all_1.gstreamer
              # Common plugins like "filesrc" to combine within e.g. gst-launch
              gst_all_1.gst-plugins-base
              # Some one might need these Specialized plugins separated by quality
              # gst_all_1.gst-plugins-good
              # gst_all_1.gst-plugins-bad
              # gst_all_1.gst-plugins-ugly
              # # Plugins to reuse ffmpeg to play almost every video format
              # gst_all_1.gst-libav
              # # Support the Video Audio (Hardware) Acceleration API
              # gst_all_1.gst-vaapi

              ## required specifically by settings application
              dbus
              pam
              rustPlatform.bindgenHook
              protobuf

            ]++ dlopenLibraries ++ commonPkgs ++ fonts;

            env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${nixpkgs.lib.makeLibraryPath dlopenLibraries}";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            PROTOC = "${pkgs.protobuf}/bin/protoc";
          };
        });
    };
}