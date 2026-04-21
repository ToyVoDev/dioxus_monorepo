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
                  after = [ "postgresql.service" ];
                  requires = [ "postgresql.service" ];
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
          # xcrun shim: routes iOS/watchOS/tvOS SDK queries to Xcode so that
          # build scripts (objc2-exception-helper, aws-lc-sys, etc.) can find
          # the simulator and device SDKs, while letting macOS SDK queries fall
          # through unchanged so Nix's cc-wrapper keeps using its own Nix SDK
          # (which has arm64 TBDs — required on macOS 26 / Tahoe where Apple's
          # SDK has arm64e-only stubs that Nix's ld 1010.6 cannot resolve).
          xcrunShim = pkgs.writeShellScriptBin "xcrun" ''
            for arg in "$@"; do
              case "$arg" in
                iphoneos*|iphonesimulator*|watchos*|watchsimulator*|appletvos*|appletvsimulator*|xros*|xrsimulator*)
                  exec env DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer" \
                    /usr/bin/xcrun "$@"
                  ;;
              esac
            done
            exec /usr/bin/xcrun "$@"
          '';
          # Shell snippet that configures CC/CXX/LINKER/CFLAGS/RUSTFLAGS for
          # iOS cross-compilation. Used verbatim in both dioxus_music_ios
          # buildPhase and devShells.default shellHook so the two environments
          # stay in sync. xcrunShim must be on PATH when this runs (it is,
          # because it's in nativeBuildInputs for both contexts).
          iosEnvSetup = ''
            # clang-ios-wrapper is in PATH via nativeBuildInputs. It unsets
            # MACOSX_DEPLOYMENT_TARGET and sets DEVELOPER_DIR=Xcode so
            # /usr/bin/clang resolves to the actual Xcode clang, not the Nix
            # cc-wrapper (which would inject -mmacos-version-min and conflict
            # with cc-rs's -mios-simulator-version-min for iOS sim targets).
            export CC_aarch64_apple_ios="clang-ios-wrapper"
            export CXX_aarch64_apple_ios="clang-ios-wrapper"
            export CC_aarch64_apple_ios_sim="clang-ios-wrapper"
            export CXX_aarch64_apple_ios_sim="clang-ios-wrapper"
            export CARGO_TARGET_AARCH64_APPLE_IOS_LINKER="clang-ios-wrapper"
            export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_LINKER="clang-ios-wrapper"

            # Resolve iOS device and simulator SDK paths via the xcrun shim.
            # The shim routes these calls to Xcode while leaving DEVELOPER_DIR
            # pointing at the Nix SDK for all macOS cc-wrapper invocations.
            _ios_sdk=$(xcrun --sdk iphoneos --show-sdk-path 2>/dev/null)
            _sim_sdk=$(xcrun --sdk iphonesimulator --show-sdk-path 2>/dev/null)
            if [ -n "$_ios_sdk" ]; then
              export CFLAGS_aarch64_apple_ios="-isysroot $_ios_sdk"
              export CXXFLAGS_aarch64_apple_ios="-isysroot $_ios_sdk"
              export CARGO_TARGET_AARCH64_APPLE_IOS_RUSTFLAGS="-C link-arg=-isysroot -C link-arg=$_ios_sdk"
            fi
            if [ -n "$_sim_sdk" ]; then
              export CFLAGS_aarch64_apple_ios_sim="-isysroot $_sim_sdk"
              export CXXFLAGS_aarch64_apple_ios_sim="-isysroot $_sim_sdk"
              export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_RUSTFLAGS="-C link-arg=-isysroot -C link-arg=$_sim_sdk"
            fi
            unset _ios_sdk _sim_sdk
          '';
          # Clang wrapper for iOS cross-compilation targets.
          # Nix's Darwin stdenv exports MACOSX_DEPLOYMENT_TARGET (e.g. "14.0")
          # into the shell. System clang reads this env var and auto-injects
          # -mmacos-version-min=14.0 into every invocation. When cc-rs also
          # injects -mios-simulator-version-min=<sdk-version> for iOS sim
          # targets, both flags land in the same clang call and conflict:
          #   "invalid argument '-mmacos-version-min=14.0' not allowed with
          #    '-mios-simulator-version-min=26.4'"
          # Additionally, /usr/bin/clang on macOS is a stub; when DEVELOPER_DIR
          # points at the Nix SDK (not a real Xcode install) the stub can't find
          # its toolchain and falls back through PATH, picking up the Nix
          # cc-wrapper (which also injects -mmacos-version-min). Explicitly
          # setting DEVELOPER_DIR=Xcode and unsetting MACOSX_DEPLOYMENT_TARGET
          # fixes both issues without affecting macOS builds (Nix's cc-wrapper
          # bakes the version min flag directly into its wrapper script).
          clangIosWrapper = pkgs.writeShellScriptBin "clang-ios-wrapper" ''
            # /usr/bin/clang is a macOS stub that resolves to the actual clang
            # binary via DEVELOPER_DIR. When DEVELOPER_DIR points at the Nix
            # SDK store path (not a real Xcode install), the stub fails to find
            # the toolchain and falls back through PATH, where it picks up the
            # Nix cc-wrapper instead of real clang. That wrapper then injects
            # -mmacos-version-min=14.0, which conflicts with cc-rs's injected
            # -mios-simulator-version-min=<sdk-ver> for iOS sim targets.
            #
            # Setting DEVELOPER_DIR=Xcode lets the stub find the real clang.
            # Unsetting MACOSX_DEPLOYMENT_TARGET prevents real clang from
            # auto-injecting -mmacos-version-min for macOS deployment.
            exec env -u MACOSX_DEPLOYMENT_TARGET \
              DEVELOPER_DIR="/Applications/Xcode.app/Contents/Developer" \
              /usr/bin/clang "$@"
          '';
          nativeBuildInputs =
            with pkgs;
            [
              dioxus-cli
              wasm-bindgen-cli_0_2_114
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
            ++ lib.optionals pkgs.stdenv.isDarwin [
              darwin.sigtool
              xcrunShim
              clangIosWrapper
            ];
          linuxDesktopDeps = with pkgs; lib.optionals pkgs.stdenv.isLinux [
            gtk3
            webkitgtk_4_1
            glib
            atk
            cairo
            gdk-pixbuf
            pango
            libsoup_3
            libxkbcommon
            libGL
            fontconfig
            freetype
            libxml2
            libxslt
            icu
            sqlite
            xdotool
          ];
          buildInputs = with pkgs; [
            openssl
            libiconv
            pkg-config
          ] ++ linuxDesktopDeps;
          rev = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "dioxus-attributes-0.1.0" = "sha256-tI26vv7fvNR18KsUJvBTXZ0c7Wc/63Qq88NAWuWMoHs=";
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
                # HOME=/var/empty (read-only) in macOS Nix builds; NIX_BUILD_TOP
                # is the writable build scratch directory. dx's AndroidTools and
                # Gradle both need a writable home for caches and downloads.
                # Java resolves user.home via getpwuid(), not $HOME, so setting
                # GRADLE_USER_HOME explicitly is required to redirect Gradle's
                # wrapper/distribution downloads.
                export HOME="$NIX_BUILD_TOP"
                export GRADLE_USER_HOME="$NIX_BUILD_TOP/.gradle"
                mkdir -p "$GRADLE_USER_HOME"

                #### For work machine that has corperate CA
                # The Nix Zulu JDK's bundled cacerts lack the corporate SSL inspection
                # proxy root CA (RQProxyCA), which is deployed via MDM to the user's
                # login keychain but is inaccessible to the nixbld build user.
                # Without it, Gradle's Maven Central HTTPS connections are intercepted
                # by the proxy and fail with "Received fatal alert: internal_error"
                # (proxy can't complete TLS 1.3 handshake) or "PKIX path building
                # failed" (proxy cert not trusted).
                #
                # Fix: copy the Zulu cacerts to a writable JKS and import the proxy CA
                # cert (checked into the repo at certs/rqproxy-ca.pem). Then point
                # JAVA_TOOL_OPTIONS at the augmented store. Force TLS 1.2 globally
                # because the corporate proxy only supports TLS 1.2 (not TLS 1.3).
                #
                # JAVA_TOOL_OPTIONS is applied to ALL JVM processes including the
                # Gradle Wrapper download. That's fine because services.gradle.org
                # is not behind the proxy, and its cert is trusted by the Zulu roots
                # that are included in our custom JKS (copied from the original).
                # _CACERTS="$NIX_BUILD_TOP/cacerts.jks"
                # _CACERTS_SRC="$JAVA_HOME/lib/security/cacerts"
                # cp "$_CACERTS_SRC" "$_CACERTS"
                # chmod 644 "$_CACERTS"
                # keytool -importcert -noprompt -trustcacerts \
                #   -keystore "$_CACERTS" -storepass "changeit" \
                #   -alias "rqproxy-ca" -file "''${./certs/rqproxy-ca.pem}"
                # Also override user.home: the Android Gradle Plugin looks up
                # ~/.android/ via Java's user.home (set from getpwuid(), not $HOME),
                # which is /var/empty for nixbld — a read-only filesystem.
                # export JAVA_TOOL_OPTIONS="-Duser.home=$NIX_BUILD_TOP -Djavax.net.ssl.trustStore=$_CACERTS -Djavax.net.ssl.trustStorePassword=changeit -Djdk.tls.client.protocols=TLSv1.2"

                # JAVA_TOOL_OPTIONS is applied to ALL JVM processes including the
                # Gradle Wrapper download. Override user.home: the Android Gradle 
                # Plugin looks up ~/.android/ via Java's user.home (set from getpwuid(),
                # not $HOME),
                export JAVA_TOOL_OPTIONS="-Duser.home=$NIX_BUILD_TOP"

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
                ${iosEnvSetup}
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
              export RUST_LOG="debug"
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              export ANDROID_HOME="${androidComposition.androidsdk}/libexec/android-sdk"
              export ANDROID_NDK_HOME="${androidComposition.androidsdk}/libexec/android-sdk/ndk-bundle"
              export BINARYEN_CORES=1
            ''
            + lib.optionalString pkgs.stdenv.isDarwin ''
              # Do NOT touch DEVELOPER_DIR or SDKROOT here.
              #
              # Nix's stdenv already sets DEVELOPER_DIR to the Nix SDK store
              # path (e.g. /nix/store/...-apple-sdk-14.4). Leaving it there
              # means Nix's cc-wrapper will use its own SDK (which has arm64
              # TBDs) rather than calling xcrun and landing on Xcode 26's
              # arm64e-only TBDs. On macOS 26 (Tahoe), using Xcode's macOS SDK
              # causes Nix's ld 1010.6 to fail with undefined symbol errors for
              # every arm64 binary (web, desktop, server).
              #
              # iOS SDK discovery (iphoneos / iphonesimulator) is handled by the
              # xcrunShim derivation in nativeBuildInputs: it intercepts xcrun
              # calls that mention an iOS/watchOS/tvOS SDK and re-execs
              # /usr/bin/xcrun with DEVELOPER_DIR=Xcode.app, while all other
              # xcrun calls (including --sdk macosx) pass through unchanged so
              # Nix's cc-wrapper continues to use its own SDK for macOS targets.

              ${iosEnvSetup}
            '';
            inherit nativeBuildInputs buildInputs;
          };
        };
    };
}
