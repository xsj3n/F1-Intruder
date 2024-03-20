{
  description = "A Nix-flake-based tauri app";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

  outputs = { self , nixpkgs ,... }: let

    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      # create an environment with nodejs_18, pnpm, and yarn
      packages = with pkgs; [
	rustc
	cargo
	nodejs_21
        typescript
       ];

      shellHook = ''
        if [ -d "/home/xis/src/f1dev/f1dev2/f1tnr-app" ]; then
	  echo -e "${GREEN}[+]-> Ready <-[+]  
	else
	  npx create-next-app@latest f1tnr-app --typescript --tailwind --eslint
	  npx shadcn-ui@latest init -y
	fi
           '';
    };
  };
}
