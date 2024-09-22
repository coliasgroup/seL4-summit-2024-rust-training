#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

let

  nixpkgsPath =
    let
      rev = "3a458f7c763ca62c6bf454b8d828bd86b7250671";
    in
      builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
        sha256 = "sha256:119wjr45ypvxhz7qxhydcrllrvlyqxla0ymb0l0wgaqrkdhff9xb";
      };

  pkgs = import nixpkgsPath {};

  inherit (pkgs) lib;

in {
  inherit pkgs;

  shell = with pkgs; mkShell {
    nativeBuildInputs = [
      pkg-config
      openssl
      rustup
      mdbook
      linkchecker
    ] ++ lib.optionals hostPlatform.isDarwin [
      libiconv
      darwin.apple_sdk.frameworks.Security
    ];
  };
}
