const fs = require("fs");
const path = require("path");
const koffi = require("koffi");

function readCases(file) {
  return fs
    .readFileSync(file, "utf8")
    .split(/\r?\n/)
    .filter((line) => line.trim() && !line.trimStart().startsWith("#"))
    .map((line) => JSON.parse(line));
}

function parseArgs(argv) {
  const args = {
    cases: path.resolve(__dirname, "../../benchmarks/cases/postgres_sqlite.jsonl"),
    library: path.resolve(__dirname, "../../target/release/libsqlgrok.dylib"),
    iterations: 1000,
    samples: 5,
    warmup: 100,
  };
  for (let i = 2; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === "--cases") args.cases = path.resolve(argv[++i]);
    else if (arg === "--library") args.library = path.resolve(argv[++i]);
    else if (arg === "--iterations") args.iterations = Number(argv[++i]);
    else if (arg === "--samples") args.samples = Number(argv[++i]);
    else if (arg === "--warmup") args.warmup = Number(argv[++i]);
    else throw new Error(`unknown argument ${arg}`);
  }
  return args;
}

function percentile(values, q) {
  if (values.length === 0) return 0;
  const sorted = [...values].sort((a, b) => a - b);
  const index = Math.ceil((q / 100) * sorted.length) - 1;
  return sorted[Math.max(0, Math.min(sorted.length - 1, index))];
}

function summarize(values) {
  const sum = values.reduce((acc, value) => acc + value, 0);
  return {
    min_ns_per_op: Math.min(...values),
    mean_ns_per_op: sum / values.length,
    median_ns_per_op: percentile(values, 50),
    p95_ns_per_op: percentile(values, 95),
    max_ns_per_op: Math.max(...values),
  };
}

function main() {
  const args = parseArgs(process.argv);
  const cases = readCases(args.cases);
  const lib = koffi.load(args.library);
  const free = lib.func("sqlgrok_free", "void", ["void *"]);
  const RustString = koffi.disposable("RustString", "str", free);
  const transpile = lib.func("sqlgrok_transpile", RustString, ["str", "str", "str"]);

  let checksum = 0;
  function call(case_) {
    const output = transpile(case_.sql, case_.read, case_.write);
    if (output == null) throw new Error(`transpile failed for ${case_.id}`);
    checksum = (checksum + output.length) >>> 0;
  }

  for (let i = 0; i < args.warmup; i += 1) {
    for (const case_ of cases) call(case_);
  }

  const samples = [];
  let measuredChecksum = 0;
  for (let sample = 0; sample < args.samples; sample += 1) {
    checksum = 0;
    const started = process.hrtime.bigint();
    for (let i = 0; i < args.iterations; i += 1) {
      for (const case_ of cases) call(case_);
    }
    const elapsedNs = Number(process.hrtime.bigint() - started);
    measuredChecksum = checksum;
    samples.push({
      elapsed_ns: elapsedNs,
      ns_per_op: elapsedNs / (cases.length * args.iterations),
      checksum,
    });
  }

  const nsPerOp = samples.map((sample) => sample.ns_per_op);
  console.log(
    JSON.stringify({
      binding: "node-koffi",
      checksum: measuredChecksum,
      cases: cases.length,
      iterations: args.iterations,
      samples: args.samples,
      operations: cases.length * args.iterations,
      ...summarize(nsPerOp),
      per_sample: samples,
    })
  );
}

main();
