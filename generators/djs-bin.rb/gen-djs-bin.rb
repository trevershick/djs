#!/usr/bin/env ruby
require 'toml'
require 'digest'
require 'erb'
require 'open-uri'
require 'ruby-progressbar'

def url_sha(uri)

    if File.exist?(".tmp") then
	    File.delete(".tmp")
    end
    pb = nil

    on_total = Proc.new do |total_bytes|
        pb = ProgressBar.create(:format         => "%a %b\u{15E7}%i %p%% %t",
                                :progress_mark  => ' ',
                                :remainder_mark => "\u{FF65}",
                                :starting_at => 0,
                                :total => total_bytes)
    end

    on_progress = Proc.new do |step|
        pb.progress = step
    end

    open(uri, content_length_proc: on_total, progress_proc: on_progress) do |page|
        File.open(".tmp", "wb") do |file|
            while chunk = page.read(1024)
                file.write(chunk)
            end
        end
    end
	sha256 = Digest::SHA256.file ".tmp"
	return sha256
end

class HomebrewRecipeGenerator
    attr_accessor :version, :architectures, :targets, :shas

    def initialize(version)
        @version = version
        # Linux and Darwin builds.
        @architectures = ["x86_64"]
        @targets = ["apple-darwin", "unknown-linux-gnu"]
        @shas = {}
        for arch in architectures do
          for target in targets do
            url="https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-#{arch}-#{target}.tar.gz"
            @shas[arch + "-" + target] = url_sha(url)
          end
        end
    end

	def render()
        tmp = ERB.new(self.get_template()).result(binding)
        tmp.split("\n").map! { |x| x[8..-1] }.join("\n")
	end

    def get_template()
        %{
        class DjsBin < Formula
          version '<%=version%>'
          desc "Targeted Jenkins Artifact Downloader - downloads artifacts from Jenkins based on branch, build and other information"
          homepage "https://github.com/trevershick/djs"

          if OS.mac?
              url "https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-x86_64-apple-darwin.tar.gz"
              sha256 "<%= shas['x86_64-apple-darwin'] %>"
          elsif OS.linux?
              url "https://github.com/trevershick/djs/releases/download/#{version}/djs-#{version}-x86_64-unknown-linux-gnu.tar.gz"
              sha256 "<%= shas['x86_64-unknown-linux-gnu'] %>"
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
        }
    end
end


# Load up the version from Cargo.toml
toml_path = ARGV[0]
output_path = ARGV[1]
puts "TOML Path #{toml_path}"
puts "Generate to #{output_path}"

toml = TOML.load_file(toml_path)
generator = HomebrewRecipeGenerator.new(toml.dig("package", "version"))

 File.open(output_path, "w") { |f|
    f.write(generator.render())
}
puts "Done."
