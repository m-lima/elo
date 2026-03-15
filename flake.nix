{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    helper.url = "github:m-lima/nix-template";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      treefmt-nix,
      helper,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        back =
          (helper.lib.rust.helper inputs system ./back {
            allowFilesets = [
              ./back/migrations
              ./back/db
              ./back/.env
            ];
            devPackages = pkgs: [
              pkgs.git-crypt
              (pkgs.writeShellScriptBin "sqlite" "exec ${pkgs.sqlite}/bin/sqlite3 -init ${pkgs.writeText "sqliteconfig" ".mode columns"} $@")
            ];
          }).outputs;
        prefixCheck =
          prefix: check:
          pkgs.lib.mapAttrs' (key: value: {
            inherit value;
            name = "${prefix}_${key}";
          }) (builtins.removeAttrs check [ "formatting" ]);

        treeFmt = {
          projectRootFile = "flake.nix";
          programs = {
            beautysh.enable = true;
            nixfmt.enable = true;
            prettier = {
              enable = true;
              settings = builtins.fromJSON (builtins.readFile ./front/.prettierrc.json);
            };
            rustfmt = {
              enable = true;
              edition = "2024";
            };
            stylua = {
              enable = true;
              settings = {
                indent_type = "Spaces";
                indent_width = 2;
                quote_style = "AutoPreferSingle";
              };
            };
            taplo.enable = true;
            xmllint.enable = true;
          };
          settings = {
            on-unmatched = "warn";
            excludes = [
              "*.lock"
              "*.sqlite"
              "*/.dockerignore"
              "*/.env"
              "*/.env.production"
              "*/.envrc"
              "*/.gitignore"
              "*/Dockerfile*"
              ".git-crypt/*"
              "LICENSE"
              "back/deploy.sh"
              "back/migrations/*"
              "front/creation/*.svg"
            ];
          };
        };
        sharedFront = {
          nodejs = pkgs.nodejs;

          src = pkgs.lib.fileset.toSource {
            root = ./front;
            fileset = pkgs.lib.fileset.unions [
              ./front/.env.production
              ./front/.prettierrc.json
              ./front/eslint.config.js
              ./front/index.html
              ./front/package.json
              ./front/public
              ./front/src
              ./front/tsconfig.json
              ./front/tsconfig.node.json
              ./front/vite.config.ts
              ./front/yarn.lock
            ];
          };

          doCheck = true;
          doDist = false;
        };
        frontChecks = {
          lint = pkgs.mkYarnPackage (
            sharedFront
            // {
              pname = "front-lint";
              dontBuild = true;

              checkPhase = ''
                runHook preCheck
                yarn --offline lint:eslint
                yarn --offline lint:tsc
                runHook postCheck
              '';

              installPhase = "mkdir -p $out";
            }
          );
        };
      in
      {
        packages = {
          back = back.packages.default;
          front = pkgs.mkYarnPackage (
            sharedFront
            // {
              nativeBuildInputs = [ pkgs.writableTmpDirAsHomeHook ];

              buildPhase = ''
                runHook preBuild
                yarn --offline build
                runHook postBuild
              '';

              installPhase = ''
                runHook preInstall
                mv deps/$pname/dist $out
                runHook postInstall
              '';
            }
          );
        };

        checks = {
          formatting = (treefmt-nix.lib.evalModule pkgs treeFmt).config.build.check self;
        }
        // (prefixCheck "back" back.checks)
        // (prefixCheck "front" frontChecks);

        formatter = (treefmt-nix.lib.evalModule pkgs treeFmt).config.build.wrapper;

        devShells = {
          back = back.devShells.default;
          front = pkgs.mkShell {
            buildInputs = [
              pkgs.yarn
              pkgs.git-crypt
            ];
          };
        };
      }
    );
}
