class DjsBin < Formula
  version '0.1.0'
  desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
  homepage "https://github.com/trevershick/djs"

  if OS.mac?
      url "https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "195e8fa0a93060f166d7aaa0411c48f4fa65ccba5241a44ed95dfe8683f208e3"
  elsif OS.linux?
      url "https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "32c5031f6ea17ff5e694518f945e3fcde0cbd2832a4d1da4fa6d6a183ec7635b"
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
