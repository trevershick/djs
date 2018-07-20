
class DjsBin < Formula
  version '0.5.0'
  desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
  homepage "https://github.com/trevershick/djs"

  if OS.mac?
      url "https://github.com/trevershick/djs/releases/download/0.5.0/djs-0.5.0-x86_64-apple-darwin.tar.gz"
      sha256 "af148a5854ea2fe599e8425474d2189aeec5f9d2000dc7a9619337a9592ac4b5"
  elsif OS.linux?
      url "https://github.com/trevershick/djs/releases/download/0.5.0/djs-0.5.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "b147de0ed423395a5f0e294f747080d98a0fc9e7887d281f9d368f35e39235da"
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
