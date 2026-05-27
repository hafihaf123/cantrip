{
  description = "CANTRIP Rust environment extension";

  inputs.master.url = "github:hafihaf123/nix_devenvs";

  outputs =
    { master, ... }:
    {

      devShells = builtins.mapAttrs (
        system: shells:
        let
          pkgs = master.inputs.nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ shells.rust ];

            buildInputs = [
              pkgs.dbus.dev
            ];
          };
        }
      ) master.devShells;
    };
}
