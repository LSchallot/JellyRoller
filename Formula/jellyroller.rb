class Jellyroller < Formula
  desc "CLI Jellyfin Controller Utility for Linux and Windows"
  homepage ""
  url "https://github.com/LSchallot/JellyRoller/archive/refs/tags/v0.6.0.tar.gz"
  sha256 "4b73fdd692c07cd698026fcaf3b53b0234afe707799f1afd67786e1efcde25b7"
  license "GPL-2.0"
  
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "false"
  end
end
