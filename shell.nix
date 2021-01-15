let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
in
pkgs.mkShell {
  name = "aws-node";

  buildInputs = [
    pkgs.nodejs-12_x
    pkgs.bashInteractive
    pkgs.jq
  ];

  shellHook = ''
    export PATH="$PWD/node_modules/.bin/:$PATH"
    alias run='npm run'
    alias scripts='jq ".scripts" package.json'
  '';

  # Environment variables
  TOP_SECRET_KEY="ieu5nvtu2a;mtq9ut"; # not actually a key ;) 
}