#!/usr/bin/env ruby
# frozen_string_literal: true

require "fiddle"
require "fiddle/import"
require "json"
require "optparse"

options = {
  cases: File.expand_path("../../benchmarks/cases/postgres_sqlite.jsonl", __dir__),
  library: File.expand_path("../../target/release/libsqlgrok.dylib", __dir__),
  iterations: 1000,
  samples: 5,
  warmup: 100
}

OptionParser.new do |opts|
  opts.on("--cases PATH") { |value| options[:cases] = File.expand_path(value) }
  opts.on("--library PATH") { |value| options[:library] = File.expand_path(value) }
  opts.on("--iterations N", Integer) { |value| options[:iterations] = value }
  opts.on("--samples N", Integer) { |value| options[:samples] = value }
  opts.on("--warmup N", Integer) { |value| options[:warmup] = value }
end.parse!

cases = File.readlines(options[:cases], chomp: true)
            .reject { |line| line.strip.empty? || line.lstrip.start_with?("#") }
            .map { |line| JSON.parse(line) }

lib = Fiddle.dlopen(options[:library])
transpile = Fiddle::Function.new(
  lib["sqlgrok_transpile"],
  [Fiddle::TYPE_VOIDP, Fiddle::TYPE_VOIDP, Fiddle::TYPE_VOIDP],
  Fiddle::TYPE_VOIDP
)
free = Fiddle::Function.new(lib["sqlgrok_free"], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)

checksum = 0
call = lambda do |case_|
  ptr = transpile.call(case_["sql"], case_["read"], case_["write"])
  raise "transpile failed for #{case_['id']}" if ptr.null?

  begin
    output = ptr.to_s
    checksum = (checksum + output.length) & 0xffffffff
  ensure
    free.call(ptr)
  end
end

options[:warmup].times do
  cases.each { |case_| call.call(case_) }
end

def percentile(values, q)
  return 0.0 if values.empty?

  sorted = values.sort
  index = ((q / 100.0) * sorted.length).ceil - 1
  sorted[[[index, 0].max, sorted.length - 1].min]
end

samples = []
measured_checksum = 0
options[:samples].times do
  checksum = 0
  started = Process.clock_gettime(Process::CLOCK_MONOTONIC, :nanosecond)
  options[:iterations].times do
    cases.each { |case_| call.call(case_) }
  end
  elapsed_ns = Process.clock_gettime(Process::CLOCK_MONOTONIC, :nanosecond) - started
  measured_checksum = checksum
  samples << {
    elapsed_ns: elapsed_ns,
    ns_per_op: elapsed_ns.to_f / (cases.length * options[:iterations]),
    checksum: checksum
  }
end
operations = cases.length * options[:iterations]
ns_per_op = samples.map { |sample| sample[:ns_per_op] }

puts JSON.generate(
  binding: "ruby-fiddle",
  checksum: measured_checksum,
  cases: cases.length,
  iterations: options[:iterations],
  samples: options[:samples],
  operations: operations,
  min_ns_per_op: ns_per_op.min,
  mean_ns_per_op: ns_per_op.sum / ns_per_op.length,
  median_ns_per_op: percentile(ns_per_op, 50),
  p95_ns_per_op: percentile(ns_per_op, 95),
  max_ns_per_op: ns_per_op.max,
  per_sample: samples
)
