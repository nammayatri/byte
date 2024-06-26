# Services and processes required by this project
{ inputs, ... }: {
  perSystem = { config, self', pkgs, lib, system, ... }:

    {
      process-compose."url-shortner-services" = {
        imports = [
          inputs.services-flake.processComposeModules.default
        ];
        services.redis-cluster."redis" = {
          enable = true;
        };
      };

      # Flake outputs
      devShells.services = pkgs.mkShell {
        nativeBuildInputs = [
          config.process-compose."url-shortner-services".outputs.package
        ];
      };
    };
}
