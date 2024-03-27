{
  description = "A Nix-flake-based tauri app";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    };

  outputs = { self , nixpkgs, rust-overlay, ... }: 
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
        dbus
        openssl_3
        glib
        gtk3
        libsoup
        webkitgtk
        librsvg
	cmake
	llvmPackages_17.libcxxClang
	perl
	go

      ];
      
      pkgs = import nixpkgs { inherit system overlays; } ;
      
      in pkgs.mkShell 
      {
        packages =  with pkgs; [
	  nodejs_21
          typescript
	  rust-analyzer
	  pkg-config
	  rust-bin.stable.latest.default
	  bashInteractive
         ];

         buildInputs = build_inp;

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
