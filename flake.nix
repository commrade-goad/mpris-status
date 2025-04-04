{
    description = "A simple program that will output the current media metadata or status.";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = nixpkgs.legacyPackages.${system};
            in
                {
                packages.default = pkgs.rustPlatform.buildRustPackage {
                    pname = "mpris-status";
                    version = "0.1";
                    src = self;

                    cargoLock = {
                        lockFile = ./Cargo.lock;
                    };

                    buildInputs = with pkgs; [
                        dbus
                    ];

                    nativeBuildInputs = with pkgs; [
                        pkg-config
                    ];

                    PKG_CONFIG_PATH = "${pkgs.dbus.dev}/lib/pkgconfig";

                    meta = {
                        description = "A simple program that will output the current media metadata or status.";
                        homepage = "https://github.com/commrade-goad/mpris-status";
                    };
                };

                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        rustc
                        cargo
                        rust-analyzer
                        rustfmt
                        clippy
                        dbus
                    ];

                    nativeBuildInputs = with pkgs; [
                        pkg-config
                    ];

                    shellHook = ''
                        export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
                        echo "Development environment ready with pkgconfig path set for dbus"
                    '';
                };
            });
}
