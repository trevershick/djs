
class DjsBin < Formula
  version '0.2.0'
  desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
  homepage "https://github.com/trevershick/djs"

  if OS.mac?
      url "https://github.com/trevershick/djs/releases/download/0.2.0/djs-0.2.0-x86_64-apple-darwin.tar.gz"
      sha256 "a3add78e426800452e2d4673e58036890ea5cca3736a9d085ab70d40b0508820"
  elsif OS.linux?
      url "https://github.com/trevershick/djs/releases/download/0.2.0/djs-0.2.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "19b5df78932b37a6a5c17fddc7fa19462a51d6a7e5b91c98e71f9c070c2d90a2"
  end

  conflicts_with "djs"

  def install
    bin.install "djs"
    man1.install "djs.1"

    #bash_completion.install "complete/djs.bash-completion"
    #fish_completion.install "complete/djs.fish"
    #zsh_completion.install "complete/_djs"
  end
end
