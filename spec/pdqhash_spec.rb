# frozen_string_literal: true

RSpec.describe Pdqhash do
  it "has a version number" do
    expect(Pdqhash::VERSION).not_to be nil
  end

  it "does something useful" do
    filepath = Bundler.root.join("spec", "fixtures", "files", "bridge-1-original.jpg").to_s
    puts filepath
    expect(Pdqhash.hash(filepath)).to eq("f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22")
  end
end
