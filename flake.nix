{
  description = "A Nix-flake-based tauri app";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nixpkgs-old.url = "github:nixos/nixpkgs/080a4a27f206d07724b88da096e27ef63401a504";
    rust-overlay = {
	url = "github:oxalica/rust-overlay";
        inputs.nixpkgs.follows = "nixpkgs"; 
      }; 
    };

  outputs = { self , nixpkgs, rust-overlay, nixpkgs-old, ... }: 
  let
    overlays = [ (import rust-overlay) ];
    system = "x86_64-linux";
  in 
  {
    devShells."${system}".default = 
    let
      libraries = with pkgs;[
        webkitgtk
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl_3
        librsvg
      ];

      build_inp = with pkgs; [
        curl
        wget
        pkg-config
        nasm
        dbus
        openssl_3
        glib
        jython
        gtk3
        libsoup
        webkitgtk
        librsvg
        cmake
        clang
        perl
        go
      ];
      
      pkgs = import nixpkgs { inherit system overlays; } ;
      pkgs-old = import nixpkgs-old {inherit system; };
      in pkgs.mkShell 
      {
        packages = [
          pkgs.bashInteractive
          pkgs.burpsuite
          pkgs.nodePackages_latest.nodejs
          pkgs.typescript
          pkgs.nodePackages.typescript-language-server
          pkgs-old.rust-analyzer
          pkgs.pkg-config
          pkgs.rust-bin.stable.latest.default
        ];


         buildInputs = build_inp;

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
          export WEBKIT_DISABLE_COMPOSITING_MODE=1
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
          export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS

          if [ -d "f1tnr-app/" ]; then
            echo -e "\033[0;32m[+]-> Ready <-[+]"
          else
            npx create-next-app@latest f1tnr-app --typescript --tailwind --eslint
            cd /home/xis/src/f1dev/f1dev2/f1tnr-app
            npx shadcn-ui@latest init -y -c f1tnr-app/
          fi
           '';
    };
  };
}
