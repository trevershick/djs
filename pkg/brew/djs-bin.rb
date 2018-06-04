
class DjsBin < Formula
  version '0.4.0'
  desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
  homepage "https://github.com/trevershick/djs"

  if OS.mac?
      url "https://github.com/trevershick/djs/releases/download/0.4.0/djs-0.4.0-x86_64-apple-darwin.tar.gz"
      sha256 "7ff7cd1cc20d941aed1b26bbe1bc8562c6fd328d8f568a20e635fd8511a87415"
  elsif OS.linux?
      url "https://github.com/trevershick/djs/releases/download/0.4.0/djs-0.4.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "71c00a79027f9b9a2d10e7de37c4252d501aa59007dba7353637988b8a94f659"
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
