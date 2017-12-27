class RipgrepBin < Formula
  version '0.1.0'
  desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
  homepage "https://github.com/trevershick/djs"

  if OS.mac?
      url "https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "ee670b0fba46323ee9a2d1c5b8bee46fa3e45778f6f105f2e8e9ee29e8bd0d45"
  elsif OS.linux?
      url "https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "ac595c2239b9a30e0e0744578afa6b73e32cdd8ae61d4f1c0ee5d6b55adbadcf"
  end

  conflicts_with "djs"

  def install
    bin.install "djs"
    #man1.install "djs.1"

    #bash_completion.install "complete/djs.bash-completion"
    #fish_completion.install "complete/djs.fish"
    #zsh_completion.install "complete/_djs"
  end
end
