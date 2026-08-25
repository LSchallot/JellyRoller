class Jellyroller < Formula
  desc "CLI Jellyfin Controller Utility for Linux and Windows"
  homepage ""
  url "https://github.com/LSchallot/JellyRoller/archive/refs/tags/v1.2.0.tar.gz"
  sha256 "9aa0e80a838b42339ab1640d1d43d077afb88ef9"
  license "GPL-2.0"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  bottle do
    root_url "https://github.com/LSchallot/JellyRoller/releases/download/v1.2.0"
    sha256 x86_64_linux: "60a6717e0c46614520dbf1d3053644b1903b5344af69a28caa5526df8d89fbbb"
  end
  
  test do
    system "false"
  end
end
