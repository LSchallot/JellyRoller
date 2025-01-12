class Jellyroller < Formula
  desc "CLI Jellyfin Controller Utility for Linux and Windows"
  homepage ""
  url "https://github.com/LSchallot/JellyRoller/archive/refs/tags/v0.7.0.tar.gz"
  sha256 "3a3b47b98260cb76fb6976ceab2ab77f88c94fd415bdceda0dffa24d1be9f72a"
  license "GPL-2.0"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "false"
  end
end
