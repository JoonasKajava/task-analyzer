{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = forEachSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devenv-up = self.devShells.${system}.default.config.procfileScript;
      devenv-test = self.devShells.${system}.default.config.test;
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "task-analyzer";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        src = nixpkgs.lib.cleanSource ./.;
        meta = {
          mainProgram = "task-analyzer";
          license = pkgs.lib.licenses.mit;
          maintainers = ["Joonas Kajava"];
        };
      };
    });

    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              packages = with pkgs; [
                at-spi2-atk
                atkmm
                cairo
                dioxus-cli
                gdk-pixbuf
                glib
                gobject-introspection
                gtk3
                harfbuzz
                libiconv
                librsvg
                libsoup_3
                lld
                openssl
                pango
                pkg-config
                tailwindcss
                wasm-bindgen-cli
                webkitgtk_4_1
                xdotool
              ];
              languages.rust.enable = true;
              process.manager.implementation = "mprocs";
              processes = {
                tailwind-watch.exec = "(while true; do sleep 10; done) | ${pkgs.tailwindcss}/bin/tailwindcss --input $DEVENV_ROOT/tailwind.css --output $DEVENV_ROOT/assets/tailwind.css --watch;";
                dx-serve.exec = "dx serve --platform web ";
              };
            }
          ];
        };
      });
  };
}
