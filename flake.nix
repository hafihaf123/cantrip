{
  description = "CANTRIP Rust environment extension";
  inputs.master.url = "path:/home/passwd/nix_devenvs";
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
