class Jellyroller < Formula
  desc "CLI Jellyfin Controller Utility for Linux and Windows"
  homepage ""
  url "https://github.com/LSchallot/JellyRoller/archive/refs/tags/v1.2.1.tar.gz"
  sha256 "6b7755b6f15f786abb48afb65d81acf4c6a5f613330d1d8c4fc55de7677b10e6"
  license "GPL-2.0"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  bottle do
    root_url "https://github.com/LSchallot/JellyRoller/releases/download/v1.2.1"
    sha256 x86_64_linux: "46632da1cc1b4c6cde61c781f8fdf6f82c4f297328017d04e9c0a2892ed02705"
  end
  
  test do
    system "false"
  end
end
