{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  packages = with pkgs; [ dbus bashInteractive ];

  languages.rust.enable = true;
}
