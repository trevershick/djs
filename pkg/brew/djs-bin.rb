
class DjsBin < Formula
  version '0.3.0'
  desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
  homepage "https://github.com/trevershick/djs"

  if OS.mac?
      url "https://github.com/trevershick/djs/releases/download/0.3.0/djs-0.3.0-x86_64-apple-darwin.tar.gz"
      sha256 "9ca9b871ae81feab6e30e41ec2c77ad729403cbf6520091ab16ee368a78afb87"
  elsif OS.linux?
      url "https://github.com/trevershick/djs/releases/download/0.3.0/djs-0.3.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "8f5fd646cf3538880900b883b003f79baa6d9cece1c269aa5196d75197be2bcb"
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
