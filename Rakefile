# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("pdqhash.gemspec")

RbSys::ExtensionTask.new("pdqhash", GEMSPEC) do |ext|
  ext.lib_dir = "lib/pdqhash"
end

task :build do
  require "bundler"

  spec = Bundler.load_gemspec("pdqhash.gemspec")

  FileUtils.rm_rf("pkg/pdqhash")

  spec.files.each do |file|
    FileUtils.mkdir_p("pkg/pdqhash/#{File.dirname(file)}")
    FileUtils.cp(file, "pkg/pdqhash/#{file}")
  end

  FileUtils.cp("pdqhash.gemspec", "pkg/pdqhash")

  full_path = File.expand_path("./../../../crates/rb-sys", __FILE__)
  cargo_toml_path = "pkg/pdqhash/ext/pdqhash/Cargo.toml"
  new_contents = File.read(cargo_toml_path).gsub("./../../../../crates/rb-sys", full_path)
  FileUtils.rm(cargo_toml_path)
  File.write(cargo_toml_path, new_contents)

  Dir.chdir("pkg/pdqhash") do
    sh "gem build pdqhash.gemspec --output=../pdqhash.gem"
  end
end

task default: %i[compile spec standard]
