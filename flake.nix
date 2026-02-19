{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  nixConfig = {
    extra-substituters = [
      "https://cache.nixos.org"
      "https://nix-community.cachix.org"
      "https://toyvo.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "toyvo.cachix.org-1:s++CG1te6YaS9mjICre0Ybbya2o/S9fZIyDNGiD4UXs="
    ];
    allow-import-from-derivation = true;
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      devshell,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [
        devshell.flakeModule
        flake-parts.flakeModules.easyOverlay
        inputs.treefmt-nix.flakeModule
      ];

      flake = {
        nixosModules.discord_bot =
          {
            pkgs,
            lib,
            config,
            ...
          }:
          let
            cfg = config.services.discord_bot;
          in
          {
            options.services.discord_bot = {
              enable = lib.mkEnableOption "enable discord bot";
              env_file = lib.mkOption {
                type = lib.types.path;
                description = ''
                  Path to the environment file, to be piped through xargs, must include the following variables:
                  DISCORD_CLIENT_ID
                  DISCORD_CLIENT_SECRET
                  DISCORD_PUBLIC_KEY
                  DISCORD_TOKEN
                '';
              };
              env = lib.mkOption {
                type = lib.types.attrs;
                default = { };
                description = ''
                  Public Environment variables to be passed to the server on startup
                '';
              };
            };
            config = lib.mkIf cfg.enable {
              nixpkgs.overlays = [ self.overlays.default ];
              services.postgresql = {
                ensureDatabases = [ "discord_bot" ];
                ensureUsers = [
                  {
                    name = "discord_bot";
                    ensureDBOwnership = true;
                    ensureClauses.login = true;
                  }
                ];
              };
              users = {
                users.discord_bot = {
                  isSystemUser = true;
                  group = "discord_bot";
                };
                groups.discord_bot = { };
              };
              systemd.services = {
                discord_bot = {
                  serviceConfig.User = "discord_bot";
                  wantedBy = [ "multi-user.target" ];
                  script = ''
                    export $(cat ${cfg.env_file} | xargs)
                    export RUST_BACKTRACE=full
                    ${lib.concatStringsSep "\n" (
                      lib.mapAttrsToList (name: value: "export ${name}=${toString value}") cfg.env
                    )}
                    ${lib.getExe pkgs.discord_bot}
                  '';
                };
              };
            };
          };
      };

      perSystem =
        {
          self',
          system,
          pkgs,
          lib,
          config,
          ...
        }:
        let
          androidComposition = pkgs.androidenv.composeAndroidPackages {
            buildToolsVersions = [
              "34.0.0"
              "35.0.0"
            ];
            platformVersions = [
              "33"
              "34"
              "35"
            ];
            abiVersions = [
              "armeabi-v7a"
              "arm64-v8a"
              "x86"
              "x86_64"
            ];
            includeNDK = true;
            includeEmulator = false;
          };
          nativeBuildInputs =
            with pkgs;
            [
              dioxus-cli
              wasm-bindgen-cli_0_2_108
              (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              androidComposition.androidsdk
              openssl
              libiconv
              libpq
              pkg-config
              rustPlatform.bindgenHook
              binaryen
              diesel-cli
              diesel-cli-ext
              jdk21_headless
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [ darwin.sigtool ];
          buildInputs = with pkgs; [
            openssl
            libiconv
            pkg-config
          ];
          rev = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "dioxus-attributes-0.1.0" = "sha256-81dTwKzebIVIDwzfK99JGF8ohstz4IVGPGHk/LzSnkE=";
            };
          };
          desktopDir = if pkgs.stdenv.isDarwin then "macos" else "linux";
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
            config = {
              allowUnfree = true;
              android_sdk.accept_license = true;
            };
          };

          treefmt = {
            programs = {
              nixfmt.enable = true;
              rustfmt.enable = true;
              prettier.enable = true;
            };
          };

          packages = {
            discord_bot = pkgs.rustPlatform.buildRustPackage rec {
              pname = "discord_bot";
              version = "${(builtins.fromTOML (builtins.readFile ./discord_bot/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              # dx build already compiles both server and client; cargo check would
              # redundantly try to check the entire workspace (including desktop crates
              # that need GTK/WebKit/glib which aren't in our buildInputs).
              doCheck = false;
              buildPhase = ''
                dx build --package discord_bot --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/web $out/bin
              '';
              meta.mainProgram = pname;
              # Single-threaded wasm-opt avoids SIGABRT from multithreading
              # race conditions in binaryen (github.com/WebAssembly/binaryen/issues/3006)
              BINARYEN_CORES = 1;
              inherit cargoLock;
            };
            game_manager = pkgs.rustPlatform.buildRustPackage rec {
              pname = "game_manager";
              version = "${(builtins.fromTOML (builtins.readFile ./game_manager/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              doCheck = false;
              buildPhase = ''
                dx build --package game_manager --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/web $out/bin
              '';
              meta.mainProgram = pname;
              BINARYEN_CORES = 1;
              inherit cargoLock;
            };
            httpui = pkgs.rustPlatform.buildRustPackage rec {
              pname = "httpui";
              version = "${(builtins.fromTOML (builtins.readFile ./httpui/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              doCheck = false;
              buildPhase = ''
                dx build --package httpui --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/${desktopDir} $out/bin
              '';
              meta.mainProgram = pname;
              inherit cargoLock;
            };
            dioxus_music_desktop = pkgs.rustPlatform.buildRustPackage rec {
              pname = "dioxus_music_desktop";
              version = "${(builtins.fromTOML (builtins.readFile ./dioxus_music/packages/desktop/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              doCheck = false;
              buildPhase = ''
                dx build --package dioxus_music_desktop --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/${desktopDir} $out/bin
              '';
              meta.mainProgram = pname;
              inherit cargoLock;
            };
            dioxus_music_web = pkgs.rustPlatform.buildRustPackage rec {
              pname = "dioxus_music_web";
              version = "${(builtins.fromTOML (builtins.readFile ./dioxus_music/packages/web/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              doCheck = false;
              buildPhase = ''
                dx build --package dioxus_music_web --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/web $out/bin
              '';
              meta.mainProgram = pname;
              BINARYEN_CORES = 1;
              inherit cargoLock;
            };
            dioxus_music_android = pkgs.rustPlatform.buildRustPackage rec {
              pname = "dioxus_music_mobile";
              version = "${(builtins.fromTOML (builtins.readFile ./dioxus_music/packages/mobile/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              doCheck = false;
              buildPhase = ''
                dx build --package dioxus_music_mobile --platform android --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/android $out/bin
              '';
              meta.mainProgram = pname;
              ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
              ANDROID_NDK_HOME = "${androidComposition.androidsdk}/libexec/android-sdk/ndk-bundle";
              inherit cargoLock;
            };
            dioxus_music_ios = pkgs.rustPlatform.buildRustPackage rec {
              pname = "dioxus_music_mobile";
              version = "${(builtins.fromTOML (builtins.readFile ./dioxus_music/packages/mobile/Cargo.toml)).package.version}-${rev}";
              src = ./.;
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
              doCheck = false;
              buildPhase = ''
                dx build --package dioxus_music_mobile --platform ios --release --verbose --trace
              '';
              installPhase = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/ios $out/bin
              '';
              meta.mainProgram = pname;
              inherit cargoLock;
            };
          };
          overlayAttrs = {
            inherit (self'.packages) discord_bot;
          };
          devShells.default = pkgs.mkShell {
            shellHook = ''
              export RUST_LOG="discord_bot=trace"
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              export ANDROID_HOME="${androidComposition.androidsdk}/libexec/android-sdk"
              export ANDROID_NDK_HOME="${androidComposition.androidsdk}/libexec/android-sdk/ndk-bundle"
              export BINARYEN_CORES=1
            ''
            + lib.optionalString pkgs.stdenv.isDarwin ''
              # Unset SDKROOT so xcrun can discover Xcode's iOS/tvOS/watchOS SDKs.
              # Nix sets this to its own macOS-only SDK which breaks iOS cross-compilation.
              # Requires Xcode to be installed on the host for iOS/Android builds.
              unset SDKROOT

              # Point xcrun at Xcode.app for iOS SDK discovery (even if xcode-select
              # points at CommandLineTools, which lack iOS SDKs).
              if [ -d "/Applications/Xcode.app/Contents/Developer" ]; then
                export DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer"
              fi

              # Use system clang for iOS cross-compilation targets.
              # The Nix-wrapped clang injects -mmacos-version-min and links Nix's
              # macOS-only dylibs (e.g. libiconv), both of which break iOS builds.
              export CC_aarch64_apple_ios="/usr/bin/clang"
              export CXX_aarch64_apple_ios="/usr/bin/clang++"
              export CC_aarch64_apple_ios_sim="/usr/bin/clang"
              export CXX_aarch64_apple_ios_sim="/usr/bin/clang++"

              # Tell rustc to use the system linker for iOS targets (avoids Nix's
              # cc-wrapper which injects macOS library paths into iOS link lines).
              export CARGO_TARGET_AARCH64_APPLE_IOS_LINKER="/usr/bin/cc"
              export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_LINKER="/usr/bin/cc"
            '';
            inherit nativeBuildInputs;
          };
        };
    };
}
