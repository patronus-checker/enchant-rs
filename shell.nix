{
  system ? builtins.currentSystem,
  sources ? import ./nix/sources.nix,
}:

let
  pkgs = import sources.nixpkgs {
    inherit system;
  };
in

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    pkg-config

    niv
  ];

  buildInputs = with pkgs; [
    enchant
  ];

  XDG_DATA_DIRS =
    with pkgs;
    lib.makeSearchPath "share" [
      hunspellDicts.en-us
    ];
}
