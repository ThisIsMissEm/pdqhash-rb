# frozen_string_literal: true

require_relative "pdqhash/version"
# require_relative "pdqhash/pdqhash"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "pdqhash/#{$1}/pdqhash"
rescue LoadError
  require "pdqhash/pdqhash"
end

module Pdqhash
  class Error < StandardError; end
  # Your code goes here...
end
