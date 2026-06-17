use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use sqlgrok::{Dialect, generate, parse, tokens::Tokenizer};

struct CountingAllocator;

static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);
static DEALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);

const SCOPE_COUNT: usize = 5;
const SCOPE_UNSCOPED: usize = 0;
const SCOPE_TOKENIZE: usize = 1;
const SCOPE_PARSE: usize = 2;
const SCOPE_GENERATE: usize = 3;
const SCOPE_PREPARE: usize = 4;

static CURRENT_SCOPE: AtomicU64 = AtomicU64::new(SCOPE_UNSCOPED as u64);
static SCOPE_ALLOCATED_BYTES: [AtomicU64; SCOPE_COUNT] = [const { AtomicU64::new(0) }; SCOPE_COUNT];
static SCOPE_ALLOCATIONS: [AtomicU64; SCOPE_COUNT] = [const { AtomicU64::new(0) }; SCOPE_COUNT];

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            record_scope_allocation(layout.size() as u64);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        DEALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, old_layout, new_size) };
        if !new_ptr.is_null() {
            DEALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            DEALLOCATED_BYTES.fetch_add(old_layout.size() as u64, Ordering::Relaxed);
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            ALLOCATED_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
            record_scope_allocation(new_size as u64);
        }
        new_ptr
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

#[derive(Debug)]
struct Args {
    cases: PathBuf,
    phase: AllocationPhase,
    iterations: usize,
    warmup: usize,
    per_case: bool,
    output: PathBuf,
    json_output: Option<PathBuf>,
    dry_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum AllocationPhase {
    Tokenize,
    Parse,
    Generate,
    Transpile,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BenchCase {
    id: String,
    sql: String,
    read: String,
    write: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    feature: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct AllocationCounters {
    allocated_bytes: u64,
    deallocated_bytes: u64,
    allocations: u64,
    deallocations: u64,
}

#[derive(Debug, Clone, Serialize)]
struct AllocationStats {
    operations: usize,
    checksum: usize,
    allocated_bytes: u64,
    deallocated_bytes: u64,
    net_bytes: i64,
    allocations: u64,
    deallocations: u64,
    bytes_per_op: f64,
    allocations_per_op: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ScopeStats {
    name: &'static str,
    allocated_bytes: u64,
    allocations: u64,
    bytes_per_op: f64,
    allocations_per_op: f64,
}

#[derive(Debug, Clone, Serialize)]
struct AllocationCaseProfile {
    id: String,
    read: String,
    write: String,
    sql: String,
    tags: Vec<String>,
    feature: String,
    stats: AllocationStats,
    scopes: Vec<ScopeStats>,
}

#[derive(Debug, Clone)]
struct ProfileResult {
    stats: AllocationStats,
    scopes: Vec<ScopeStats>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse(env::args().skip(1))?;
    args.validate()?;

    let cases = read_bench_cases(&args.cases)?;
    let aggregate = profile_allocation_cases(
        &cases,
        args.phase,
        args.iterations,
        args.warmup,
        "aggregate",
    )?;
    let per_case = if args.per_case {
        Some(profile_allocation_cases_individually(
            &cases,
            args.phase,
            args.iterations,
            args.warmup,
        )?)
    } else {
        None
    };

    let report = render_report(&args, &cases, &aggregate, per_case.as_deref());
    let json_report = render_json(&args, &cases, &aggregate, per_case.as_deref())?;

    if args.dry_run {
        print!("{report}");
        return Ok(());
    }

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&args.output, report)
        .map_err(|err| format!("failed to write {}: {err}", args.output.display()))?;
    eprintln!("wrote {}", args.output.display());

    if let Some(json_output) = &args.json_output {
        if let Some(parent) = json_output.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
        fs::write(json_output, json_report)
            .map_err(|err| format!("failed to write {}: {err}", json_output.display()))?;
        eprintln!("wrote {}", json_output.display());
    }

    Ok(())
}

impl Args {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut cases = None;
        let mut phase = AllocationPhase::Transpile;
        let mut iterations = 1_000;
        let mut warmup = 100;
        let mut per_case = false;
        let mut output = None;
        let mut json_output = None;
        let mut dry_run = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--cases" => cases = Some(next_value(&mut args, "--cases")?.into()),
                "--phase" => phase = AllocationPhase::parse(&next_value(&mut args, "--phase")?)?,
                "--iterations" => {
                    let raw = next_value(&mut args, "--iterations")?;
                    iterations = raw.parse().map_err(|_| {
                        format!("--iterations must be a positive integer, got {raw:?}")
                    })?;
                }
                "--warmup" => {
                    let raw = next_value(&mut args, "--warmup")?;
                    warmup = raw.parse().map_err(|_| {
                        format!("--warmup must be a non-negative integer, got {raw:?}")
                    })?;
                }
                "--per-case" => per_case = true,
                "--output" => output = Some(next_value(&mut args, "--output")?.into()),
                "--json-output" => {
                    json_output = Some(next_value(&mut args, "--json-output")?.into())
                }
                "--dry-run" => dry_run = true,
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let cases = cases.ok_or_else(|| "--cases is required".to_string())?;
        let output =
            output.unwrap_or_else(|| PathBuf::from("benchmarks/reports/allocation_profile.md"));
        let json_output = Some(json_output.unwrap_or_else(|| output.with_extension("json")));

        Ok(Self {
            cases,
            phase,
            iterations,
            warmup,
            per_case,
            output,
            json_output,
            dry_run,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if !self.cases.is_file() {
            return Err(format!("{} does not exist", self.cases.display()));
        }
        if self.iterations == 0 {
            return Err("--iterations must be greater than zero".to_string());
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --release --bin sqlgrok_alloc_profile -- --cases benchmarks/cases/postgres_sqlite.jsonl [--phase tokenize|parse|generate|transpile] [--iterations 1000] [--warmup 100] [--per-case] [--output benchmarks/reports/allocation_profile.md] [--json-output benchmarks/reports/allocation_profile.json] [--dry-run]".to_string()
    }
}

impl AllocationPhase {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "tokenize" => Ok(Self::Tokenize),
            "parse" => Ok(Self::Parse),
            "generate" => Ok(Self::Generate),
            "transpile" => Ok(Self::Transpile),
            _ => Err(format!(
                "unknown allocation phase {value:?}; expected tokenize, parse, generate, or transpile"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Tokenize => "tokenize",
            Self::Parse => "parse",
            Self::Generate => "generate",
            Self::Transpile => "transpile",
        }
    }
}

fn next_value(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
    flag: &str,
) -> Result<String, String> {
    let Some(value) = args.next() else {
        return Err(format!("{flag} requires a value"));
    };
    if value.starts_with("--") {
        return Err(format!("{flag} requires a value, got flag {value:?}"));
    }
    Ok(value)
}

fn read_bench_cases(path: &Path) -> Result<Vec<BenchCase>, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let mut cases = Vec::new();
    let mut ids = HashSet::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let mut case: BenchCase = serde_json::from_str(line)
            .map_err(|err| format!("{}:{}: invalid JSON: {err}", path.display(), index + 1))?;
        if case.id.trim().is_empty() {
            return Err(format!(
                "{}:{}: id must not be empty",
                path.display(),
                index + 1
            ));
        }
        if !ids.insert(case.id.clone()) {
            return Err(format!("duplicate benchmark case id {:?}", case.id));
        }
        if case.sql.trim().is_empty() {
            return Err(format!("{}: sql must not be empty", case.id));
        }
        if case.feature.is_empty() {
            case.feature = case
                .tags
                .first()
                .cloned()
                .unwrap_or_else(|| "uncategorized".to_string());
        }
        cases.push(case);
    }
    if cases.is_empty() {
        return Err(format!("{} contains no benchmark cases", path.display()));
    }
    Ok(cases)
}

fn bench_case_dialects(cases: &[BenchCase]) -> Result<Vec<(Dialect, Dialect)>, String> {
    cases
        .iter()
        .map(|case| {
            let read = Dialect::from_str(&case.read)
                .ok_or_else(|| format!("{}: unknown read dialect {:?}", case.id, case.read))?;
            let write = Dialect::from_str(&case.write)
                .ok_or_else(|| format!("{}: unknown write dialect {:?}", case.id, case.write))?;
            Ok((read, write))
        })
        .collect()
}

fn reset_allocation_counters() {
    ALLOCATED_BYTES.store(0, Ordering::Relaxed);
    DEALLOCATED_BYTES.store(0, Ordering::Relaxed);
    ALLOCATIONS.store(0, Ordering::Relaxed);
    DEALLOCATIONS.store(0, Ordering::Relaxed);
    CURRENT_SCOPE.store(SCOPE_UNSCOPED as u64, Ordering::Relaxed);
    for counter in &SCOPE_ALLOCATED_BYTES {
        counter.store(0, Ordering::Relaxed);
    }
    for counter in &SCOPE_ALLOCATIONS {
        counter.store(0, Ordering::Relaxed);
    }
}

fn allocation_counters() -> AllocationCounters {
    AllocationCounters {
        allocated_bytes: ALLOCATED_BYTES.load(Ordering::Relaxed),
        deallocated_bytes: DEALLOCATED_BYTES.load(Ordering::Relaxed),
        allocations: ALLOCATIONS.load(Ordering::Relaxed),
        deallocations: DEALLOCATIONS.load(Ordering::Relaxed),
    }
}

fn record_scope_allocation(bytes: u64) {
    let scope = CURRENT_SCOPE.load(Ordering::Relaxed) as usize;
    if scope < SCOPE_COUNT {
        SCOPE_ALLOCATED_BYTES[scope].fetch_add(bytes, Ordering::Relaxed);
        SCOPE_ALLOCATIONS[scope].fetch_add(1, Ordering::Relaxed);
    }
}

struct ScopeGuard {
    previous: u64,
}

impl ScopeGuard {
    fn enter(scope: usize) -> Self {
        let previous = CURRENT_SCOPE.swap(scope as u64, Ordering::Relaxed);
        Self { previous }
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        CURRENT_SCOPE.store(self.previous, Ordering::Relaxed);
    }
}

fn scoped<T>(scope: usize, f: impl FnOnce() -> T) -> T {
    let _guard = ScopeGuard::enter(scope);
    f()
}

fn scope_name(scope: usize) -> &'static str {
    match scope {
        SCOPE_UNSCOPED => "unscoped",
        SCOPE_TOKENIZE => "tokenize",
        SCOPE_PARSE => "parse",
        SCOPE_GENERATE => "generate",
        SCOPE_PREPARE => "prepare",
        _ => "unknown",
    }
}

fn scope_stats(operations: usize) -> Vec<ScopeStats> {
    (0..SCOPE_COUNT)
        .filter_map(|scope| {
            let allocated_bytes = SCOPE_ALLOCATED_BYTES[scope].load(Ordering::Relaxed);
            let allocations = SCOPE_ALLOCATIONS[scope].load(Ordering::Relaxed);
            (allocated_bytes > 0 || allocations > 0).then(|| ScopeStats {
                name: scope_name(scope),
                allocated_bytes,
                allocations,
                bytes_per_op: allocated_bytes as f64 / operations as f64,
                allocations_per_op: allocations as f64 / operations as f64,
            })
        })
        .collect()
}

fn profile_allocation_cases(
    cases: &[BenchCase],
    phase: AllocationPhase,
    iterations: usize,
    warmup: usize,
    label: &str,
) -> Result<ProfileResult, String> {
    let dialects = bench_case_dialects(cases)?;
    let prepared = PreparedCases::new(cases, &dialects, phase)?;
    let mut checksum = 0usize;

    for _ in 0..warmup {
        for index in 0..cases.len() {
            checksum = checksum.wrapping_add(
                run_phase_operation(cases, &dialects, &prepared, phase, index).map_err(|err| {
                    format!("{}: allocation warmup failed: {err}", cases[index].id)
                })?,
            );
        }
    }

    checksum = 0;
    reset_allocation_counters();
    for _ in 0..iterations {
        for index in 0..cases.len() {
            checksum = checksum.wrapping_add(
                run_phase_operation(cases, &dialects, &prepared, phase, index).map_err(|err| {
                    format!("{}: allocation profile failed: {err}", cases[index].id)
                })?,
            );
        }
    }
    let operations = iterations * cases.len();
    let counters = allocation_counters();
    Ok(ProfileResult {
        stats: allocation_stats_from_counters(counters, operations, checksum, label)?,
        scopes: scope_stats(operations),
    })
}

enum PreparedCases {
    None,
    Parsed(Vec<sqlgrok::ast::Statement>),
}

impl PreparedCases {
    fn new(
        cases: &[BenchCase],
        dialects: &[(Dialect, Dialect)],
        phase: AllocationPhase,
    ) -> Result<Self, String> {
        match phase {
            AllocationPhase::Tokenize | AllocationPhase::Parse | AllocationPhase::Transpile => {
                Ok(Self::None)
            }
            AllocationPhase::Generate => {
                let parsed = cases
                    .iter()
                    .zip(dialects)
                    .map(|(case, (read, _))| {
                        parse(&case.sql, *read).map_err(|err| {
                            format!("{}: allocation prepare parse failed: {err}", case.id)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Parsed(parsed))
            }
        }
    }
}

fn run_phase_operation(
    cases: &[BenchCase],
    dialects: &[(Dialect, Dialect)],
    prepared: &PreparedCases,
    phase: AllocationPhase,
    index: usize,
) -> Result<usize, String> {
    let case = &cases[index];
    let (read, write) = dialects[index];
    match phase {
        AllocationPhase::Tokenize => {
            let tokens = scoped(SCOPE_TOKENIZE, || {
                let mut tokenizer = Tokenizer::new(std::hint::black_box(&case.sql));
                tokenizer.tokenize().map_err(|err| err.to_string())
            })?;
            Ok(tokens.len())
        }
        AllocationPhase::Parse => {
            let ast = scoped(SCOPE_PARSE, || {
                parse(std::hint::black_box(&case.sql), read).map_err(|err| err.to_string())
            })?;
            let marker = (&ast as *const _) as usize;
            std::hint::black_box(&ast);
            Ok(marker & 0xff)
        }
        AllocationPhase::Generate => {
            let PreparedCases::Parsed(parsed) = prepared else {
                return Err("generate phase missing prepared parsed ASTs".to_string());
            };
            let output = scoped(SCOPE_GENERATE, || {
                generate(std::hint::black_box(&parsed[index]), write)
            });
            Ok(output.len())
        }
        AllocationPhase::Transpile => {
            let ast = scoped(SCOPE_PARSE, || {
                parse(std::hint::black_box(&case.sql), read).map_err(|err| err.to_string())
            })?;
            let output = scoped(SCOPE_GENERATE, || generate(&ast, write));
            Ok(output.len())
        }
    }
}

fn allocation_stats_from_counters(
    counters: AllocationCounters,
    operations: usize,
    checksum: usize,
    label: &str,
) -> Result<AllocationStats, String> {
    if operations == 0 {
        return Err(format!(
            "{label}: allocation profile requires at least one operation"
        ));
    }
    Ok(AllocationStats {
        operations,
        checksum,
        allocated_bytes: counters.allocated_bytes,
        deallocated_bytes: counters.deallocated_bytes,
        net_bytes: counters.allocated_bytes as i64 - counters.deallocated_bytes as i64,
        allocations: counters.allocations,
        deallocations: counters.deallocations,
        bytes_per_op: counters.allocated_bytes as f64 / operations as f64,
        allocations_per_op: counters.allocations as f64 / operations as f64,
    })
}

fn profile_allocation_cases_individually(
    cases: &[BenchCase],
    phase: AllocationPhase,
    iterations: usize,
    warmup: usize,
) -> Result<Vec<AllocationCaseProfile>, String> {
    let mut rows = Vec::with_capacity(cases.len());
    for case in cases {
        let result = profile_allocation_cases(
            std::slice::from_ref(case),
            phase,
            iterations,
            warmup,
            &format!("case {}", case.id),
        )?;
        rows.push(AllocationCaseProfile {
            id: case.id.clone(),
            read: case.read.clone(),
            write: case.write.clone(),
            sql: case.sql.clone(),
            tags: case.tags.clone(),
            feature: case.feature.clone(),
            stats: result.stats,
            scopes: result.scopes,
        });
    }
    Ok(rows)
}

fn render_report(
    args: &Args,
    cases: &[BenchCase],
    aggregate: &ProfileResult,
    per_case: Option<&[AllocationCaseProfile]>,
) -> String {
    let mut out = String::new();
    out.push_str("# sqlgrok Allocation Profile\n\n");
    out.push_str(&format!(
        "Counts allocations in a dedicated helper binary while repeatedly measuring the `{}` phase.\n\n",
        args.phase.as_str()
    ));
    out.push_str("## Summary\n\n");
    out.push_str(&format!("- Case file: `{}`\n", args.cases.display()));
    out.push_str(&format!("- Phase: `{}`\n", args.phase.as_str()));
    out.push_str(&format!("- Cases: `{}`\n", cases.len()));
    out.push_str(&format!("- Iterations per case: `{}`\n", args.iterations));
    out.push_str(&format!(
        "- Warmup iterations per case: `{}`\n",
        args.warmup
    ));
    out.push_str(&format!("- Operations: `{}`\n", aggregate.stats.operations));
    out.push_str(&format!(
        "- Output checksum: `{}`\n",
        aggregate.stats.checksum
    ));
    out.push_str(&format!(
        "- Allocated: `{:.2} KiB/op` across `{:.2}` allocations/op\n",
        aggregate.stats.bytes_per_op / 1024.0,
        aggregate.stats.allocations_per_op
    ));
    out.push_str(&format!(
        "- Total allocated: `{:.2} MiB`\n",
        aggregate.stats.allocated_bytes as f64 / (1024.0 * 1024.0)
    ));
    out.push_str(&format!(
        "- Net bytes after drops: `{}`\n\n",
        aggregate.stats.net_bytes
    ));

    out.push_str("## Notes\n\n");
    out.push_str(
        "- This is allocation accounting, not wall-clock timing. Pair it with `bench-sqlglot` and Criterion phase benches.\n",
    );
    match args.phase {
        AllocationPhase::Generate | AllocationPhase::Transpile => out.push_str(
            "- Counts include the output `String`, because normal callers also receive that allocation.\n",
        ),
        AllocationPhase::Tokenize | AllocationPhase::Parse => out.push_str(
            "- Counts exclude later phases; prepared inputs for generate are built before counters are reset.\n",
        ),
    }
    out.push_str(
        "- The counting allocator lives only in this helper binary, so normal `xtask bench-sqlglot` timing is not perturbed.\n\n",
    );

    if !aggregate.scopes.is_empty() {
        let mut rows = aggregate.scopes.iter().collect::<Vec<_>>();
        rows.sort_by(|left, right| right.bytes_per_op.total_cmp(&left.bytes_per_op));
        out.push_str("## Scoped Allocation Breakdown\n\n");
        out.push_str("| scope | KiB/op | allocations/op |\n");
        out.push_str("| --- | ---: | ---: |\n");
        for row in rows {
            out.push_str(&format!(
                "| `{}` | {:.2} | {:.2} |\n",
                row.name,
                row.bytes_per_op / 1024.0,
                row.allocations_per_op
            ));
        }
        out.push('\n');
    }

    if let Some(per_case) = per_case {
        let mut rows = per_case.iter().collect::<Vec<_>>();
        rows.sort_by(|left, right| right.stats.bytes_per_op.total_cmp(&left.stats.bytes_per_op));
        out.push_str("## Per-Case Breakdown\n\n");
        out.push_str("| id | KiB/op | allocations/op | net bytes/op | tags |\n");
        out.push_str("| --- | ---: | ---: | ---: | --- |\n");
        for row in rows {
            let tags = if row.tags.is_empty() {
                row.feature.clone()
            } else {
                row.tags.join(",")
            };
            out.push_str(&format!(
                "| `{}` | {:.2} | {:.2} | {:.2} | `{}` |\n",
                row.id,
                row.stats.bytes_per_op / 1024.0,
                row.stats.allocations_per_op,
                row.stats.net_bytes as f64 / row.stats.operations as f64,
                tags
            ));
        }
        out.push('\n');
    }

    out.push_str("## Workload\n\n");
    out.push_str("| id | read | write | tags | SQL |\n");
    out.push_str("| --- | --- | --- | --- | --- |\n");
    for case in cases {
        let tags = if case.tags.is_empty() {
            case.feature.clone()
        } else {
            case.tags.join(",")
        };
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            case.id,
            case.read,
            case.write,
            tags,
            case.sql.replace('|', "\\|")
        ));
    }

    out
}

fn render_json(
    args: &Args,
    cases: &[BenchCase],
    aggregate: &ProfileResult,
    per_case: Option<&[AllocationCaseProfile]>,
) -> Result<String, String> {
    let report = serde_json::json!({
        "case_file": args.cases,
        "phase": args.phase,
        "cases": cases,
        "case_count": cases.len(),
        "iterations": args.iterations,
        "warmup": args.warmup,
        "aggregate": aggregate.stats,
        "scopes": aggregate.scopes,
        "per_case": per_case,
    });
    serde_json::to_string_pretty(&report)
        .map_err(|err| format!("failed to serialize allocation JSON: {err}"))
}
