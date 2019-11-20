self: super: {
  pmserver = self.callPackage ./pmserver.nix {};
  pmserver-docker = self.callPackage ./pmserver-docker.nix {};
}
