# AST Inventory

Generated from Python SQLGlot's `sqlglot/expressions/` package and sqlgrok's `src/ast/types.rs`.

This is a planning document, not a conformance claim. `supported` means sqlgrok has a clear AST home for the construct. `partial` means there is a home but missing SQLGlot behavior is likely. `unsupported` means imported fixtures need AST design before implementation. `out-of-scope` means the construct is not currently on the parity critical path.

## Snapshot

- SQLGlot expression classes: `1024`
- SQLGlot expression files: `15`
- Rust AST enums inspected: `Statement, Expr, TypedFunction, DataType, JoinType, TableSource`
- Supported: `106`
- Partial: `707`
- Unsupported: `205`
- Out of scope: `6`

## Priority Gaps

| Gap | Status | Why it matters |
| --- | --- | --- |
| DDL properties and constraints | partial | First imported MySQL->SQLite batch fails on DDL/type normalization before deeper fixtures run. |
| Dedicated GroupConcat AST | partial | Current parser/generator handles key MySQL cases, but SQLGlot has GroupConcat as a first-class aggregate. |
| Ordered/filter aggregate modifiers | partial | Needed for broader aggregate fixture imports beyond simple COUNT/SUM/AVG/MIN/MAX. |
| Struct/map/object literals | partial | Data types exist, but expression-level constructors and dialect generation are thin. |
| JSON path/operator model | partial | Common JSON functions exist; SQLGlot has richer paths, operators, and dialect spellings. |
| Table alias and lateral richness | partial | Aliases exist, but SQLGlot models richer alias columns, lateral views, and source metadata. |
| Set operation options | partial | UNION/INTERSECT/EXCEPT exist; modifiers and nested ordering need more parity coverage. |
| Window frame/null treatment | partial | Window specs exist, but SQLGlot models IGNORE/RESPECT NULLS and ordered aggregate nuance. |
| Command/cache/export/load statements | unsupported | SQLGlot supports operational statements outside current core parser scope. |
| Long-tail function variants | partial | Hundreds of SQLGlot Func/AggFunc classes currently collapse to generic function or unsupported typed variants. |

## Rust AST Surface

- `Statement`: `Select`, `Insert`, `Update`, `Delete`, `CreateTable`, `DropTable`, `CreateIndex`, `DropIndex`, `SetOperation`, `AlterTable`, `CreateView`, `DropView`, `Truncate`, `Transaction`, `Explain`, `Use`, `Merge`, `Expression`
- `Expr`: `Column`, `Number`, `StringLiteral`, `Boolean`, `Null`, `BinaryOp`, `UnaryOp`, `Function`, `Between`, `InList`, `InSubquery`, `AnyOp`, `AllOp`, `IsNull`, `IsBool`, `Like`, `ILike`, `Case`, `Nested`, `Wildcard`, `Subquery`, `Exists`, `Cast`, `TryCast`, `Extract`, `Interval`, `ArrayLiteral`, `Tuple`, `Coalesce`, `If`, `NullIf`, `Collate`, `Parameter`, `TypeExpr`, `QualifiedWildcard`, `Star`, `Alias`, `ArrayIndex`, `JsonAccess`, `Lambda`, `Default`, `Cube`, `Rollup`, `GroupingSets`, `TypedFunction`, `Commented`
- `TypedFunction`: `DateAdd`, `DateDiff`, `DateTrunc`, `DateSub`, `CurrentDate`, `CurrentTimestamp`, `StrToTime`, `TimeToStr`, `TsOrDsToDate`, `Year`, `Month`, `Day`, `Trim`, `Substring`, `Upper`, `Lower`, `RegexpLike`, `RegexpExtract`, `RegexpReplace`, `ConcatWs`, `Split`, `Initcap`, `Length`, `Replace`, `Reverse`, `Left`, `Right`, `Lpad`, `Rpad`, `Count`, `Sum`, `Avg`, `Min`, `Max`, `ArrayAgg`, `ApproxDistinct`, `Variance`, `Stddev`, `ArrayConcat`, `ArrayContains`, `ArraySize`, `Explode`, `GenerateSeries`, `Flatten`, `JSONExtract`, `JSONExtractScalar`, `ParseJSON`, `JSONFormat`, `RowNumber`, `Rank`, `DenseRank`, `NTile`, `Lead`, `Lag`, `FirstValue`, `LastValue`, `Abs`, `Ceil`, `Floor`, `Round`, `Log`, `Ln`, `Pow`, `Sqrt`, `Greatest`, `Least`, `Mod`, `Hex`, `Unhex`, `Md5`, `Sha`, `Sha2`
- `DataType`: `TinyInt`, `SmallInt`, `Int`, `BigInt`, `Float`, `Double`, `Decimal`, `Numeric`, `Real`, `Varchar`, `Char`, `Text`, `String`, `Binary`, `Varbinary`, `Boolean`, `Date`, `Time`, `Timestamp`, `Interval`, `DateTime`, `Blob`, `Bytea`, `Bytes`, `Json`, `Jsonb`, `Uuid`, `Array`, `Map`, `Struct`, `Tuple`, `Null`, `Unknown`, `Variant`, `Object`, `Xml`, `Inet`, `Cidr`, `Macaddr`, `Bit`, `Money`, `Serial`, `BigSerial`, `SmallSerial`, `Regclass`, `Regtype`, `Hstore`, `Geography`, `Geometry`, `Super`
- `JoinType`: `Inner`, `Left`, `Right`, `Full`, `Cross`, `Comma`, `Natural`, `Lateral`
- `TableSource`: `Table`, `Subquery`, `TableFunction`, `Lateral`, `Unnest`, `Pivot`, `Unpivot`

## Coverage By SQLGlot Module

| Module | Supported | Partial | Unsupported | Out of scope | Total |
| --- | ---: | ---: | ---: | ---: | ---: |
| aggregate.py | 10 | 61 | 0 | 2 | 73 |
| array.py | 5 | 55 | 6 | 0 | 66 |
| constraints.py | 0 | 45 | 0 | 0 | 45 |
| core.py | 22 | 55 | 33 | 0 | 110 |
| datatypes.py | 2 | 0 | 4 | 0 | 6 |
| ddl.py | 4 | 4 | 43 | 0 | 51 |
| dml.py | 4 | 2 | 10 | 0 | 16 |
| functions.py | 10 | 86 | 0 | 0 | 96 |
| json.py | 2 | 32 | 0 | 0 | 34 |
| math.py | 7 | 53 | 0 | 0 | 60 |
| properties.py | 0 | 120 | 0 | 4 | 124 |
| query.py | 15 | 12 | 108 | 0 | 135 |
| string.py | 17 | 86 | 1 | 0 | 104 |
| temporal.py | 8 | 96 | 0 | 0 | 104 |

## Full Generated Inventory

| SQLGlot class | Module | Bases | Status | Rust representation / notes |
| --- | --- | --- | --- | --- |
| ExplodeOuter | array.py:239 | Expr | unsupported | no clear Rust AST representation yet |
| ExplodingGenerateSeries | array.py:221 | GenerateSeries | unsupported | no clear Rust AST representation yet |
| Posexplode | array.py:247 | Explode | unsupported | no clear Rust AST representation yet |
| PosexplodeOuter | array.py:251 | Posexplode, ExplodeOuter | unsupported | no clear Rust AST representation yet |
| PositionalColumn | array.py:255 | Expression | unsupported | no clear Rust AST representation yet |
| _ExplodeOuter | array.py:243 | Explode, ExplodeOuter | unsupported | no clear Rust AST representation yet |
| Aliases | core.py:1888 | Expression | unsupported | no clear Rust AST representation yet |
| AtIndex | core.py:2226 | Expression | unsupported | no clear Rust AST representation yet |
| AtTimeZone | core.py:2230 | Expression | unsupported | no clear Rust AST representation yet |
| Cache | core.py:1558 | Expression | unsupported | no clear Rust AST representation yet |
| Check | core.py:2045 | Expression | unsupported | no clear Rust AST representation yet |
| ColumnConstraintKind | core.py:1580 | Expr | unsupported | no clear Rust AST representation yet |
| CombinedAggFunc | core.py:1957 | AnonymousAggFunc | unsupported | no clear Rust AST representation yet |
| CombinedParameterizedAgg | core.py:1961 | ParameterizedAgg | unsupported | no clear Rust AST representation yet |
| Distinct | core.py:2252 | Expression | unsupported | no clear Rust AST representation yet |
| Filter | core.py:2041 | Expression | unsupported | no clear Rust AST representation yet |
| ForIn | core.py:1915 | Expression | unsupported | no clear Rust AST representation yet |
| FormatPhrase | core.py:2238 | Expression | unsupported | no clear Rust AST representation yet |
| FromTimeZone | core.py:2234 | Expression | unsupported | no clear Rust AST representation yet |
| HavingMax | core.py:1927 | Expression | unsupported | no clear Rust AST representation yet |
| Hint | core.py:1758 | Expression | unsupported | no clear Rust AST representation yet |
| Identifier | core.py:1766 | Expression | unsupported | no clear Rust AST representation yet |
| IgnoreNulls | core.py:1919 | Expression | unsupported | no clear Rust AST representation yet |
| IntervalOp | core.py:2030 | TimeUnit | unsupported | no clear Rust AST representation yet |
| JoinHint | core.py:1762 | Expression | unsupported | no clear Rust AST representation yet |
| LockingStatement | core.py:1575 | Expression | unsupported | no clear Rust AST representation yet |
| Opclass | core.py:1780 | Expression | unsupported | no clear Rust AST representation yet |
| Ordered | core.py:2049 | Expression | unsupported | no clear Rust AST representation yet |
| PivotAlias | core.py:1880 | Alias | unsupported | no clear Rust AST representation yet |
| PivotAny | core.py:1884 | Expression | unsupported | no clear Rust AST representation yet |
| Pseudocolumn | core.py:1754 | Column | unsupported | no clear Rust AST representation yet |
| Refresh | core.py:1571 | Expression | unsupported | no clear Rust AST representation yet |
| RespectNulls | core.py:1923 | Expression | unsupported | no clear Rust AST representation yet |
| Slice | core.py:1980 | Expression | unsupported | no clear Rust AST representation yet |
| TimeUnit | core.py:1985 | Expr | unsupported | no clear Rust AST representation yet |
| Uncache | core.py:1567 | Expression | unsupported | no clear Rust AST representation yet |
| Var | core.py:1746 | Expression | unsupported | no clear Rust AST representation yet |
| WithinGroup | core.py:1750 | Expression | unsupported | no clear Rust AST representation yet |
| _TimeUnit | core.py:2023 | Expression, TimeUnit | unsupported | no clear Rust AST representation yet |
| DataTypeParam | datatypes.py:26 | Expression | unsupported | no clear Rust AST representation yet |
| IntervalSpan | datatypes.py:445 | DataType | unsupported | no clear Rust AST representation yet |
| ObjectIdentifier | datatypes.py:441 | DataType | unsupported | no clear Rust AST representation yet |
| PseudoType | datatypes.py:437 | DataType | unsupported | no clear Rust AST representation yet |
| AlterColumn | ddl.py:233 | Expression | unsupported | no clear Rust AST representation yet |
| AlterDistStyle | ddl.py:256 | Expression | unsupported | no clear Rust AST representation yet |
| AlterIndex | ddl.py:252 | Expression | unsupported | no clear Rust AST representation yet |
| AlterModifySqlSecurity | ddl.py:290 | Expression | unsupported | no clear Rust AST representation yet |
| AlterRename | ddl.py:282 | Expression | unsupported | no clear Rust AST representation yet |
| AlterSession | ddl.py:406 | Expression | unsupported | no clear Rust AST representation yet |
| AlterSet | ddl.py:264 | Expression | unsupported | no clear Rust AST representation yet |
| AlterSortKey | ddl.py:260 | Expression | unsupported | no clear Rust AST representation yet |
| Attach | ddl.py:134 | Expression | unsupported | no clear Rust AST representation yet |
| CharacterSet | ddl.py:229 | Expression | unsupported | no clear Rust AST representation yet |
| Clone | ddl.py:117 | Expression | unsupported | no clear Rust AST representation yet |
| Comment | ddl.py:298 | Expression | unsupported | no clear Rust AST representation yet |
| Commit | ddl.py:373 | Expression | unsupported | no clear Rust AST representation yet |
| Comprehension | ddl.py:308 | Expression | unsupported | no clear Rust AST representation yet |
| Declare | ddl.py:165 | Expression | unsupported | no clear Rust AST representation yet |
| DeclareItem | ddl.py:169 | Expression | unsupported | no clear Rust AST representation yet |
| Describe | ddl.py:121 | Expression | unsupported | no clear Rust AST representation yet |
| Detach | ddl.py:138 | Expression | unsupported | no clear Rust AST representation yet |
| DropPrimaryKey | ddl.py:361 | Expression | unsupported | no clear Rust AST representation yet |
| Execute | ddl.py:418 | Expression | unsupported | no clear Rust AST representation yet |
| ExecuteSql | ddl.py:426 | Execute | unsupported | no clear Rust AST representation yet |
| Heredoc | ddl.py:177 | Expression | unsupported | no clear Rust AST representation yet |
| Install | ddl.py:149 | Expression | unsupported | no clear Rust AST representation yet |
| Kill | ddl.py:157 | Expression | unsupported | no clear Rust AST representation yet |
| MergeTreeTTL | ddl.py:328 | Expression | unsupported | no clear Rust AST representation yet |
| MergeTreeTTLAction | ddl.py:318 | Expression | unsupported | no clear Rust AST representation yet |
| ModifyColumn | ddl.py:248 | Expression | unsupported | no clear Rust AST representation yet |
| Pragma | ddl.py:161 | Expression | unsupported | no clear Rust AST representation yet |
| RenameColumn | ddl.py:278 | Expression | unsupported | no clear Rust AST representation yet |
| RenameIndex | ddl.py:286 | Expression | unsupported | no clear Rust AST representation yet |
| Rollback | ddl.py:377 | Expression | unsupported | no clear Rust AST representation yet |
| SequenceProperties | ddl.py:64 | Expression | unsupported | no clear Rust AST representation yet |
| Set | ddl.py:173 | Expression | unsupported | no clear Rust AST representation yet |
| SetItem | ddl.py:181 | Expression | unsupported | no clear Rust AST representation yet |
| Show | ddl.py:191 | Expression | unsupported | no clear Rust AST representation yet |
| Summarize | ddl.py:153 | Expression | unsupported | no clear Rust AST representation yet |
| SwapTable | ddl.py:294 | Expression | unsupported | no clear Rust AST representation yet |
| TriggerEvent | ddl.py:96 | Expression | unsupported | no clear Rust AST representation yet |
| TriggerExecute | ddl.py:92 | Expression | unsupported | no clear Rust AST representation yet |
| TriggerProperties | ddl.py:76 | Expression | unsupported | no clear Rust AST representation yet |
| TriggerReferencing | ddl.py:100 | Expression | unsupported | no clear Rust AST representation yet |
| TruncateTable | ddl.py:104 | Expression | unsupported | no clear Rust AST representation yet |
| UserDefinedFunction | ddl.py:225 | Expression | unsupported | no clear Rust AST representation yet |
| Copy | dml.py:166 | Expression, DML | unsupported | no clear Rust AST representation yet |
| CopyParameter | dml.py:162 | Expression | unsupported | no clear Rust AST representation yet |
| Credentials | dml.py:177 | Expression | unsupported | no clear Rust AST representation yet |
| Directory | dml.py:187 | Expression | unsupported | no clear Rust AST representation yet |
| DirectoryStage | dml.py:191 | Expression | unsupported | no clear Rust AST representation yet |
| Export | dml.py:158 | Expression | unsupported | no clear Rust AST representation yet |
| LoadData | dml.py:281 | Expression | unsupported | no clear Rust AST representation yet |
| OnConflict | dml.py:265 | Expression | unsupported | no clear Rust AST representation yet |
| When | dml.py:519 | Expression | unsupported | no clear Rust AST representation yet |
| Whens | dml.py:523 | Expression | unsupported | no clear Rust AST representation yet |
| AddPartition | query.py:1938 | Expression | unsupported | no clear Rust AST representation yet |
| Analyze | query.py:1876 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeColumns | query.py:1930 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeDelete | query.py:1914 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeHistogram | query.py:1897 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeListChainedRows | query.py:1910 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeSample | query.py:1906 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeStatistics | query.py:1888 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeValidate | query.py:1922 | Expression | unsupported | no clear Rust AST representation yet |
| AnalyzeWith | query.py:1918 | Expression | unsupported | no clear Rust AST representation yet |
| AttachOption | query.py:1942 | Expression | unsupported | no clear Rust AST representation yet |
| Block | query.py:2104 | Expression | unsupported | no clear Rust AST representation yet |
| CTE | query.py:454 | Expression, DerivedTable | unsupported | no clear Rust AST representation yet |
| Changes | query.py:522 | Expression | unsupported | no clear Rust AST representation yet |
| Cluster | query.py:862 | Order | unsupported | no clear Rust AST representation yet |
| ColumnDef | query.py:502 | Expression | unsupported | no clear Rust AST representation yet |
| ColumnPosition | query.py:498 | Expression | unsupported | no clear Rust AST representation yet |
| ConditionalInsert | query.py:569 | Expression | unsupported | no clear Rust AST representation yet |
| Connect | query.py:526 | Expression | unsupported | no clear Rust AST representation yet |
| DerivedTable | query.py:96 | Selectable | unsupported | no clear Rust AST representation yet |
| Distribute | query.py:866 | Order | unsupported | no clear Rust AST representation yet |
| DropPartition | query.py:1946 | Expression | unsupported | no clear Rust AST representation yet |
| EndStatement | query.py:2116 | Expression | unsupported | no clear Rust AST representation yet |
| Except | query.py:1088 | SetOperation | unsupported | no clear Rust AST representation yet |
| Fetch | query.py:601 | Expression | unsupported | no clear Rust AST representation yet |
| Final | query.py:837 | Expression | unsupported | no clear Rust AST representation yet |
| ForClause | query.py:919 | Expression | unsupported | no clear Rust AST representation yet |
| FormatJson | query.py:2019 | Expression | unsupported | no clear Rust AST representation yet |
| Get | query.py:939 | Expression | unsupported | no clear Rust AST representation yet |
| Grant | query.py:609 | Expression | unsupported | no clear Rust AST representation yet |
| Having | query.py:554 | Expression | unsupported | no clear Rust AST representation yet |
| HistoricalData | query.py:931 | Expression | unsupported | no clear Rust AST representation yet |
| IfBlock | query.py:2108 | Expression | unsupported | no clear Rust AST representation yet |
| Index | query.py:558 | Expression | partial | standalone `CREATE INDEX`/`DROP INDEX` statements exist with basic expression and sort-direction parameters; included columns, predicates, and dialect-specific options remain shallow |
| IndexTableHint | query.py:927 | Expression | unsupported | no clear Rust AST representation yet |
| InputOutputFormat | query.py:878 | Expression | unsupported | no clear Rust AST representation yet |
| Intersect | query.py:1092 | SetOperation | unsupported | no clear Rust AST representation yet |
| Into | query.py:534 | Expression | unsupported | no clear Rust AST representation yet |
| Introducer | query.py:581 | Expression | unsupported | no clear Rust AST representation yet |
| JSON | query.py:1962 | Expression | unsupported | no clear Rust AST representation yet |
| JSONColumnDef | query.py:2027 | Expression | unsupported | no clear Rust AST representation yet |
| JSONExtractQuote | query.py:2059 | Expression | unsupported | no clear Rust AST representation yet |
| JSONKeyValue | query.py:2023 | Expression | unsupported | no clear Rust AST representation yet |
| JSONPath | query.py:1966 | Expression | unsupported | no clear Rust AST representation yet |
| JSONPathFilter | query.py:1979 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathKey | query.py:1983 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathPart | query.py:1975 | Expression | unsupported | no clear Rust AST representation yet |
| JSONPathRecursive | query.py:1987 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathRoot | query.py:1991 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathScript | query.py:1995 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathSelector | query.py:2003 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathSlice | query.py:1999 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathSubscript | query.py:2007 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathUnion | query.py:2011 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONPathWildcard | query.py:2015 | JSONPathPart | unsupported | no clear Rust AST representation yet |
| JSONSchema | query.py:2038 | Expression | unsupported | no clear Rust AST representation yet |
| JSONValue | query.py:2042 | Expression | unsupported | no clear Rust AST representation yet |
| LimitOptions | query.py:660 | Expression | unsupported | no clear Rust AST representation yet |
| Lock | query.py:1124 | Expression | unsupported | no clear Rust AST representation yet |
| MatchRecognize | query.py:824 | Expression | unsupported | no clear Rust AST representation yet |
| MatchRecognizeMeasure | query.py:817 | Expression | unsupported | no clear Rust AST representation yet |
| ModelAttribute | query.py:2074 | Expression | unsupported | no clear Rust AST representation yet |
| MultitableInserts | query.py:573 | Expression | unsupported | no clear Rust AST representation yet |
| National | query.py:585 | Expression | unsupported | no clear Rust AST representation yet |
| OnCondition | query.py:577 | Expression | unsupported | no clear Rust AST representation yet |
| OpenJSONColumnDef | query.py:2055 | Expression | unsupported | no clear Rust AST representation yet |
| OverflowTruncateBehavior | query.py:1958 | Expression | unsupported | no clear Rust AST representation yet |
| Partition | query.py:589 | Expression | unsupported | no clear Rust AST representation yet |
| PartitionId | query.py:597 | Expression | unsupported | no clear Rust AST representation yet |
| PartitionRange | query.py:593 | Expression | unsupported | no clear Rust AST representation yet |
| PreWhere | query.py:1868 | Expression | unsupported | no clear Rust AST representation yet |
| Prior | query.py:530 | Expression | unsupported | no clear Rust AST representation yet |
| ProjectionDef | query.py:464 | Expression | unsupported | no clear Rust AST representation yet |
| Put | query.py:935 | Expression | unsupported | no clear Rust AST representation yet |
| Qualify | query.py:874 | Expression | unsupported | no clear Rust AST representation yet |
| Query | query.py:112 | Selectable | unsupported | no clear Rust AST representation yet |
| QueryBand | query.py:438 | Expression | unsupported | no clear Rust AST representation yet |
| QueryOption | query.py:914 | Expression | unsupported | no clear Rust AST representation yet |
| RecursiveWithSearch | query.py:442 | Expression | unsupported | no clear Rust AST representation yet |
| ReplacePartition | query.py:1950 | Expression | unsupported | no clear Rust AST representation yet |
| Return | query.py:882 | Expression | unsupported | no clear Rust AST representation yet |
| Revoke | query.py:619 | Expression | unsupported | no clear Rust AST representation yet |
| Schema | query.py:1120 | Expression | unsupported | no clear Rust AST representation yet |
| ScopeResolution | query.py:2066 | Expression | unsupported | no clear Rust AST representation yet |
| Selectable | query.py:80 | Expr | unsupported | no clear Rust AST representation yet |
| Semicolon | query.py:2086 | Expression | unsupported | no clear Rust AST representation yet |
| SkipJSONColumn | query.py:858 | Expression | unsupported | no clear Rust AST representation yet |
| Sort | query.py:870 | Order | unsupported | no clear Rust AST representation yet |
| StoredProcedure | query.py:2100 | Expression | unsupported | no clear Rust AST representation yet |
| Stream | query.py:2070 | Expression | unsupported | no clear Rust AST representation yet |
| TableColumn | query.py:2090 | Expression | unsupported | no clear Rust AST representation yet |
| TableFromRows | query.py:807 | Expression, UDTF | unsupported | no clear Rust AST representation yet |
| TableSample | query.py:1723 | Expression | unsupported | no clear Rust AST representation yet |
| Tag | query.py:1737 | Expression | unsupported | no clear Rust AST representation yet |
| TranslateCharacters | query.py:1954 | Expression | unsupported | no clear Rust AST representation yet |
| UDTF | query.py:104 | DerivedTable | unsupported | no clear Rust AST representation yet |
| UnpivotColumns | query.py:1841 | Expression | unsupported | no clear Rust AST representation yet |
| UsingData | query.py:1934 | Expression | unsupported | no clear Rust AST representation yet |
| Values | query.py:1096 | Expression, UDTF | unsupported | no clear Rust AST representation yet |
| Variadic | query.py:2096 | Expression | unsupported | no clear Rust AST representation yet |
| Version | query.py:1106 | Expression | unsupported | no clear Rust AST representation yet |
| Where | query.py:1872 | Expression | unsupported | no clear Rust AST representation yet |
| WhileBlock | query.py:2112 | Expression | unsupported | no clear Rust AST representation yet |
| WindowSpec | query.py:1857 | Expression | unsupported | no clear Rust AST representation yet |
| WithFill | query.py:849 | Expression | unsupported | no clear Rust AST representation yet |
| WithTableHint | query.py:923 | Expression | unsupported | no clear Rust AST representation yet |
| XMLKeyValueOption | query.py:2082 | Expression | unsupported | no clear Rust AST representation yet |
| XMLNamespace | query.py:2078 | Expression | unsupported | no clear Rust AST representation yet |
| LowerHex | string.py:313 | Hex | unsupported | no clear Rust AST representation yet |
| AnyValue | aggregate.py:17 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxPercentileAccumulate | aggregate.py:25 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxPercentileCombine | aggregate.py:29 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxPercentileEstimate | aggregate.py:33 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxQuantile | aggregate.py:234 | Quantile | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxQuantiles | aggregate.py:37 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxTopK | aggregate.py:41 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxTopKAccumulate | aggregate.py:45 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxTopKCombine | aggregate.py:49 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxTopKEstimate | aggregate.py:53 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproxTopSum | aggregate.py:57 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ApproximateSimilarity | aggregate.py:21 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArgMax | aggregate.py:61 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArgMin | aggregate.py:66 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayAgg | aggregate.py:71 | Expression, AggFunc | partial | TypedFunction::ArrayAgg, limited ordered/filter/null behavior |
| ArrayConcatAgg | aggregate.py:75 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayUnionAgg | aggregate.py:79 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayUniqueAgg | aggregate.py:83 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Corr | aggregate.py:91 | Expression, AggFunc, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Count | aggregate.py:98 | Expression, AggFunc | partial | TypedFunction::Count, limited ordered/filter variants |
| CountIf | aggregate.py:103 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CovarPop | aggregate.py:107 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CovarSamp | aggregate.py:111 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CumeDist | aggregate.py:115 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| First | aggregate.py:125 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FirstValue | aggregate.py:129 | Expression, AggFunc | partial | TypedFunction::FirstValue, window/null treatment partial |
| GroupConcat | aggregate.py:133 | Expression, AggFunc | partial | supported through parsed GROUP_CONCAT function; AST lacks dedicated variant |
| Grouping | aggregate.py:137 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GroupingId | aggregate.py:142 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Kurtosis | aggregate.py:147 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Lag | aggregate.py:151 | Expression, AggFunc | partial | TypedFunction::Lag, window defaults partial |
| Last | aggregate.py:155 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LastValue | aggregate.py:159 | Expression, AggFunc | partial | TypedFunction::LastValue, window/null treatment partial |
| Lead | aggregate.py:163 | Expression, AggFunc | partial | TypedFunction::Lead, window defaults partial |
| LogicalAnd | aggregate.py:167 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LogicalOr | aggregate.py:171 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Median | aggregate.py:180 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Minhash | aggregate.py:189 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MinhashCombine | aggregate.py:194 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Mode | aggregate.py:198 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| NthValue | aggregate.py:206 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ObjectAgg | aggregate.py:210 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| PercentRank | aggregate.py:225 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| PercentileCont | aggregate.py:214 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| PercentileDisc | aggregate.py:218 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Quantile | aggregate.py:230 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrAvgx | aggregate.py:249 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrAvgy | aggregate.py:253 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrCount | aggregate.py:257 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrIntercept | aggregate.py:261 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrR2 | aggregate.py:265 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrSlope | aggregate.py:269 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrSxx | aggregate.py:273 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrSxy | aggregate.py:277 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrSyy | aggregate.py:281 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrValx | aggregate.py:285 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegrValy | aggregate.py:289 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Skewness | aggregate.py:297 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StddevPop | aggregate.py:305 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StddevSamp | aggregate.py:309 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| VariancePop | aggregate.py:321 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Apply | array.py:202 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayAll | array.py:100 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayAny | array.py:104 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayAppend | array.py:46 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayCompact | array.py:50 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayConstructCompact | array.py:29 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayContains | array.py:108 | Expression, Binary, Func | partial | TypedFunction::ArrayContains, dialect coverage partial |
| ArrayContainsAll | array.py:113 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| ArrayDistinct | array.py:138 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayExcept | array.py:117 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayFilter | array.py:60 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayFirst | array.py:142 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayInsert | array.py:65 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayIntersect | array.py:121 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayLast | array.py:146 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayMax | array.py:150 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayMin | array.py:154 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayOverlaps | array.py:127 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| ArrayPosition | array.py:131 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| ArrayPrepend | array.py:69 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayRemove | array.py:73 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayRemoveAt | array.py:77 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayReverse | array.py:81 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArraySize | array.py:158 | Expression, Func | partial | TypedFunction::ArraySize, dialect coverage partial |
| ArraySlice | array.py:85 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArraySort | array.py:89 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArraySum | array.py:163 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArrayToString | array.py:175 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ArraysZip | array.py:170 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Explode | array.py:229 | Expression, Func, UDTF | partial | TypedFunction::Explode, generator support partial |
| Generator | array.py:225 | Expression, Func, UDTF | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Inline | array.py:234 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| List | array.py:34 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Map | array.py:279 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapCat | array.py:293 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapContainsKey | array.py:297 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapDelete | array.py:301 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapFromEntries | array.py:306 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapInsert | array.py:310 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapKeys | array.py:314 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapPick | array.py:318 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MapSize | array.py:323 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Reduce | array.py:206 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SortArray | array.py:93 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StDistance | array.py:363 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StPoint | array.py:367 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StarMap | array.py:327 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StringToArray | array.py:190 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StrtokToArray | array.py:195 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Struct | array.py:351 | Expression, Func | partial | DataType::Struct exists; expression-level struct literals are limited |
| StructExtract | array.py:356 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToArray | array.py:39 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToMap | array.py:331 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Transform | array.py:210 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| VarMap | array.py:335 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AddConstraint | constraints.py:236 | Expression | partial | DDL metadata is represented only for common cases |
| AssumeColumnConstraint | constraints.py:60 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| AutoIncrementColumnConstraint | constraints.py:32 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| CaseSpecificColumnConstraint | constraints.py:48 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| CharacterSetColumnConstraint | constraints.py:52 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| CheckColumnConstraint | constraints.py:56 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| ClusteredColumnConstraint | constraints.py:64 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| CollateColumnConstraint | constraints.py:68 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| ColumnConstraint | constraints.py:24 | Expression | partial | DDL metadata is represented only for common cases |
| ColumnPrefix | constraints.py:215 | Expression | partial | DDL metadata is represented only for common cases |
| CommentColumnConstraint | constraints.py:72 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| CompressColumnConstraint | constraints.py:76 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| ComputedColumnConstraint | constraints.py:193 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| Constraint | constraints.py:201 | Expression | partial | DDL metadata is represented only for common cases |
| DateFormatColumnConstraint | constraints.py:80 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| DefaultColumnConstraint | constraints.py:84 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| EncodeColumnConstraint | constraints.py:88 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| EphemeralColumnConstraint | constraints.py:96 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| ExcludeColumnConstraint | constraints.py:92 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| ForeignKey | constraints.py:205 | Expression | partial | DDL metadata is represented only for common cases |
| GeneratedAsIdentityColumnConstraint | constraints.py:104 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| GeneratedAsRowColumnConstraint | constraints.py:119 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| InOutColumnConstraint | constraints.py:197 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| IndexColumnConstraint | constraints.py:123 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| IndexConstraintOption | constraints.py:8 | Expression | partial | DDL metadata is represented only for common cases |
| IndexParameters | constraints.py:223 | Expression | partial | DDL metadata is represented only for common cases |
| InlineLengthColumnConstraint | constraints.py:135 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| InvisibleColumnConstraint | constraints.py:36 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| MaskingPolicyColumnConstraint | constraints.py:147 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| NonClusteredColumnConstraint | constraints.py:139 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| NotForReplicationColumnConstraint | constraints.py:143 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| NotNullColumnConstraint | constraints.py:151 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| OnUpdateColumnConstraint | constraints.py:155 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| PathColumnConstraint | constraints.py:185 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| PeriodForSystemTimeConstraint | constraints.py:44 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| PrimaryKey | constraints.py:219 | Expression | partial | DDL metadata is represented only for common cases |
| PrimaryKeyColumnConstraint | constraints.py:159 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| ProjectionPolicyColumnConstraint | constraints.py:189 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| Reference | constraints.py:20 | Expression | partial | DDL metadata is represented only for common cases |
| TitleColumnConstraint | constraints.py:163 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| UniqueColumnConstraint | constraints.py:167 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| UppercaseColumnConstraint | constraints.py:177 | Expression, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| WatermarkColumnConstraint | constraints.py:181 | Expression | partial | DDL metadata is represented only for common cases |
| WithOperator | constraints.py:100 | Expression | partial | DDL metadata is represented only for common cases |
| ZeroFillColumnConstraint | constraints.py:40 | ColumnConstraint | partial | DDL metadata is represented only for common cases |
| Add | core.py:2057 | Expression, Binary | partial | covered by Expr::BinaryOp, but SQLGlot has operator-specific classes |
| Adjacent | core.py:2197 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| AggFunc | core.py:1669 | Func | partial | many common aggregates are typed; long tail falls back to Expr::Function |
| Anonymous | core.py:1943 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AnonymousAggFunc | core.py:1952 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Binary | core.py:1598 | Condition | partial | represented by generic operator/predicate nodes where parsed |
| BitwiseAnd | core.py:2061 | Expression, Binary | partial | BinaryOperator coverage, limited dialect spelling |
| BitwiseLeftShift | core.py:2065 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| BitwiseNot | core.py:2205 | Unary | partial | represented by generic operator/predicate nodes where parsed |
| BitwiseOr | core.py:2069 | Expression, Binary | partial | BinaryOperator coverage, limited dialect spelling |
| BitwiseRightShift | core.py:2073 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| BitwiseXor | core.py:2077 | Expression, Binary | partial | BinaryOperator coverage, limited dialect spelling |
| Bracket | core.py:1896 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| Condition | core.py:1549 | Expr | partial | represented by generic operator/predicate nodes where parsed |
| Connector | core.py:1611 | Binary | partial | represented by generic operator/predicate nodes where parsed |
| DPipe | core.py:2097 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Distance | core.py:2117 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| DistanceNd | core.py:2121 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Div | core.py:2081 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Dot | core.py:1830 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Escape | core.py:2125 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| ExtendsLeft | core.py:2089 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| ExtendsRight | core.py:2093 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| GT | core.py:2133 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| GTE | core.py:2137 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Glob | core.py:2129 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| HashAgg | core.py:1965 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Hll | core.py:1970 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| IntDiv | core.py:2145 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Kwarg | core.py:1868 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| LT | core.py:2161 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| LTE | core.py:2165 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Match | core.py:2157 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Mul | core.py:2173 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| NEQ | core.py:2177 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Neg | core.py:2219 | Unary | partial | represented by generic operator/predicate nodes where parsed |
| NestedJSONSelect | core.py:2181 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Not | core.py:2209 | Unary | partial | represented by generic operator/predicate nodes where parsed |
| NullSafeEQ | core.py:2105 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| NullSafeNEQ | core.py:2109 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Operator | core.py:2185 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| Overlaps | core.py:2085 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| ParameterizedAgg | core.py:1939 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Placeholder | core.py:1804 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| Predicate | core.py:1554 | Condition | partial | represented by generic operator/predicate nodes where parsed |
| PropertyEQ | core.py:2113 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| RegexpLike | core.py:2284 | Expression, Binary, Func | partial | TypedFunction::RegexpLike, dialect coverage partial |
| SafeFunc | core.py:1931 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SessionParameter | core.py:1800 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| SimilarTo | core.py:2189 | Expression, Binary, Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Sub | core.py:2193 | Expression, Binary | partial | represented by generic operator/predicate nodes where parsed |
| SubqueryPredicate | core.py:1585 | Predicate | partial | represented by generic operator/predicate nodes where parsed |
| Typeof | core.py:1935 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Unary | core.py:2201 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| Xor | core.py:2275 | Expression, Connector, Func | partial | represented by generic operator/predicate nodes where parsed |
| Alter | ddl.py:381 | Expression | partial | Statement::AlterTable exists, but operation coverage is shallow |
| Command | ddl.py:365 | Expression | partial | some commands map to dedicated statements; many are unsupported |
| DDL | ddl.py:16 | Selectable | partial | core DDL exists; options/constraints are partial |
| NextValueFor | ddl.py:414 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DML | dml.py:32 | Expr | partial | core DML exists; vendor-specific clauses are partial |
| Returning | dml.py:277 | Expression | partial | DML returning fields exist, but coverage varies |
| AIClassify | functions.py:327 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AIEmbed | functions.py:332 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AIForecast | functions.py:382 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AIGenerate | functions.py:344 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AISimilarity | functions.py:338 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CastToStrType | functions.py:80 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CheckXml | functions.py:178 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Collation | functions.py:170 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Columns | functions.py:480 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ConnectByRoot | functions.py:174 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Convert | functions.py:84 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentAccount | functions.py:232 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentAccountName | functions.py:236 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentAvailableRoles | functions.py:240 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentCatalog | functions.py:244 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentClient | functions.py:248 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentDatabase | functions.py:252 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentIpAddress | functions.py:256 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentOrganizationName | functions.py:260 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentOrganizationUser | functions.py:264 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentRegion | functions.py:268 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentRole | functions.py:272 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentRoleType | functions.py:276 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentSchema | functions.py:280 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentSchemas | functions.py:284 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentSecondaryRoles | functions.py:288 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentSession | functions.py:292 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentStatement | functions.py:296 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentTransaction | functions.py:300 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentUser | functions.py:304 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentUserId | functions.py:308 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentVersion | functions.py:312 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentWarehouse | functions.py:316 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DecodeCase | functions.py:128 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| EqualNull | functions.py:133 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FeaturesAtTime | functions.py:350 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Float64 | functions.py:189 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateBool | functions.py:366 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateDouble | functions.py:374 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateEmbedding | functions.py:354 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateInt | functions.py:370 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateTable | functions.py:362 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateText | functions.py:358 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GetIgnoreCase | functions.py:439 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Host | functions.py:461 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Int64 | functions.py:193 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| IsArray | functions.py:197 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| IsNullValue | functions.py:201 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONCast | functions.py:76 | Cast | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LaxBool | functions.py:205 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LaxFloat64 | functions.py:209 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LaxInt64 | functions.py:213 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LaxString | functions.py:217 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MLForecast | functions.py:378 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MLTranslate | functions.py:397 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| NetFunc | functions.py:465 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Normal | functions.py:484 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Nvl2 | functions.py:155 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ObjectTransform | functions.py:151 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ParseIp | functions.py:469 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Predict | functions.py:401 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Rand | functions.py:488 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Randn | functions.py:493 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Randstr | functions.py:497 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RangeBucket | functions.py:501 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RangeN | functions.py:505 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ReadCSV | functions.py:420 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ReadParquet | functions.py:426 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegDomain | functions.py:473 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Seq1 | functions.py:509 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Seq2 | functions.py:513 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Seq4 | functions.py:517 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Seq8 | functions.py:521 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SessionUser | functions.py:320 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToBoolean | functions.py:221 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToVariant | functions.py:225 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Try | functions.py:159 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Uniform | functions.py:525 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Uuid | functions.py:529 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| VectorSearch | functions.py:405 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| WeekStart | functions.py:535 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| WidthBucket | functions.py:539 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| XMLElement | functions.py:434 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| XMLGet | functions.py:443 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| XMLTable | functions.py:448 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Zipf | functions.py:549 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CheckJson | json.py:8 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONArray | json.py:12 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONArrayAgg | json.py:22 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONArrayAppend | json.py:32 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONArrayContains | json.py:38 | Expression, Binary, Predicate, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONArrayInsert | json.py:43 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONBContains | json.py:49 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONBContainsAllTopKeys | json.py:53 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONBContainsAnyTopKeys | json.py:57 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONBDeleteAtPath | json.py:61 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONBExists | json.py:65 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONBExtract | json.py:70 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONBExtractScalar | json.py:74 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| JSONBObjectAgg | json.py:79 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONBool | json.py:83 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONExists | json.py:87 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONExtract | json.py:97 | Expression, Binary, Func | partial | TypedFunction::JSONExtract, path/operator coverage partial |
| JSONExtractArray | json.py:119 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONExtractScalar | json.py:124 | Expression, Binary, Func | partial | TypedFunction::JSONExtractScalar, path/operator coverage partial |
| JSONKeys | json.py:146 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONKeysAtDepth | json.py:152 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONObject | json.py:156 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONObjectAgg | json.py:166 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONRemove | json.py:176 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONSet | json.py:182 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONStripNulls | json.py:188 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONTable | json.py:202 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JSONType | json.py:212 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ObjectId | json.py:217 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ObjectInsert | json.py:221 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| OpenJSON | json.py:230 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StripNullValue | json.py:198 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Acos | math.py:11 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Acosh | math.py:15 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Asin | math.py:19 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Asinh | math.py:23 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Atan | math.py:27 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Atan2 | math.py:35 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Atanh | math.py:31 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitmapBitPosition | math.py:235 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitmapBucketNumber | math.py:239 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitmapConstructAgg | math.py:243 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitmapCount | math.py:247 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitmapOrAgg | math.py:251 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitwiseAndAgg | math.py:219 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitwiseCount | math.py:223 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitwiseOrAgg | math.py:227 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitwiseXorAgg | math.py:231 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Booland | math.py:255 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Boolnot | math.py:259 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Boolor | math.py:263 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BoolxorAgg | math.py:267 | Expression, AggFunc | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Cbrt | math.py:130 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Cos | math.py:39 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Cosh | math.py:43 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CosineDistance | math.py:98 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Cot | math.py:47 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Coth | math.py:51 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Csc | math.py:55 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Csch | math.py:59 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Degrees | math.py:63 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DotProduct | math.py:102 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| EuclideanDistance | math.py:106 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Exp | math.py:139 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Factorial | math.py:143 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Getbit | math.py:271 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| IsInf | math.py:151 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| IsNan | math.py:155 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JarowinklerSimilarity | math.py:110 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ManhattanDistance | math.py:119 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Pi | math.py:167 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Radians | math.py:67 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SafeAdd | math.py:196 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SafeDivide | math.py:200 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SafeMultiply | math.py:204 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SafeNegate | math.py:208 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SafeSubtract | math.py:212 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Sec | math.py:71 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Sech | math.py:75 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Sign | math.py:180 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Sin | math.py:79 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Sinh | math.py:83 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Tan | math.py:87 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Tanh | math.py:91 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Trunc | math.py:188 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AlgorithmProperty | properties.py:28 | Property | partial | DDL metadata is represented only for common cases |
| AllowedValuesProperty | properties.py:24 | Expression | partial | DDL metadata is represented only for common cases |
| ApiProperty | properties.py:32 | Property | partial | DDL metadata is represented only for common cases |
| ApplicationProperty | properties.py:36 | Property | partial | DDL metadata is represented only for common cases |
| AutoIncrementProperty | properties.py:40 | Property | partial | DDL metadata is represented only for common cases |
| AutoRefreshProperty | properties.py:44 | Property | partial | DDL metadata is represented only for common cases |
| BackupProperty | properties.py:48 | Property | partial | DDL metadata is represented only for common cases |
| BlockCompressionProperty | properties.py:56 | Property | partial | DDL metadata is represented only for common cases |
| BuildProperty | properties.py:52 | Property | partial | DDL metadata is represented only for common cases |
| CatalogProperty | properties.py:66 | Property | partial | DDL metadata is represented only for common cases |
| CharacterSetProperty | properties.py:70 | Property | partial | DDL metadata is represented only for common cases |
| ChecksumProperty | properties.py:74 | Property | partial | DDL metadata is represented only for common cases |
| ClusteredByProperty | properties.py:222 | Property | partial | DDL metadata is represented only for common cases |
| ComputeProperty | properties.py:82 | Property | partial | DDL metadata is represented only for common cases |
| CopyGrantsProperty | properties.py:86 | Property | partial | DDL metadata is represented only for common cases |
| CredentialsProperty | properties.py:172 | Property | partial | DDL metadata is represented only for common cases |
| DataBlocksizeProperty | properties.py:90 | Property | partial | DDL metadata is represented only for common cases |
| DataDeletionProperty | properties.py:100 | Property | partial | DDL metadata is represented only for common cases |
| DatabaseProperty | properties.py:104 | Property | partial | DDL metadata is represented only for common cases |
| DefinerProperty | properties.py:108 | Property | partial | DDL metadata is represented only for common cases |
| DictProperty | properties.py:226 | Property | partial | DDL metadata is represented only for common cases |
| DictRange | properties.py:234 | Property | partial | DDL metadata is represented only for common cases |
| DictSubProperty | properties.py:230 | Property | partial | DDL metadata is represented only for common cases |
| DistKeyProperty | properties.py:112 | Property | partial | DDL metadata is represented only for common cases |
| DistStyleProperty | properties.py:120 | Property | partial | DDL metadata is represented only for common cases |
| DistributedByProperty | properties.py:116 | Property | partial | DDL metadata is represented only for common cases |
| DuplicateKeyProperty | properties.py:124 | Property | partial | DDL metadata is represented only for common cases |
| DynamicProperty | properties.py:238 | Property | partial | DDL metadata is represented only for common cases |
| EmptyProperty | properties.py:246 | Property | partial | DDL metadata is represented only for common cases |
| EncodeProperty | properties.py:570 | Property | partial | DDL metadata is represented only for common cases |
| EnviromentProperty | properties.py:218 | Property | partial | DDL metadata is represented only for common cases |
| ExecuteAsProperty | properties.py:156 | Property | partial | DDL metadata is represented only for common cases |
| ExternalProperty | properties.py:160 | Property | partial | DDL metadata is represented only for common cases |
| FallbackProperty | properties.py:164 | Property | partial | DDL metadata is represented only for common cases |
| ForceProperty | properties.py:578 | Property | partial | DDL metadata is represented only for common cases |
| FreespaceProperty | properties.py:176 | Property | partial | DDL metadata is represented only for common cases |
| GlobalProperty | properties.py:180 | Property | partial | DDL metadata is represented only for common cases |
| GrantPrincipal | properties.py:20 | Expression | partial | DDL metadata is represented only for common cases |
| GrantPrivilege | properties.py:16 | Expression | partial | DDL metadata is represented only for common cases |
| HandlerProperty | properties.py:144 | Property | partial | DDL metadata is represented only for common cases |
| HeapProperty | properties.py:136 | Property | partial | DDL metadata is represented only for common cases |
| HybridProperty | properties.py:140 | Property | partial | DDL metadata is represented only for common cases |
| IncludeProperty | properties.py:574 | Property | partial | DDL metadata is represented only for common cases |
| InheritsProperty | properties.py:188 | Property | partial | DDL metadata is represented only for common cases |
| InputModelProperty | properties.py:192 | Property | partial | DDL metadata is represented only for common cases |
| IsolatedLoadingProperty | properties.py:200 | Property | partial | DDL metadata is represented only for common cases |
| JournalProperty | properties.py:204 | Property | partial | DDL metadata is represented only for common cases |
| LanguageProperty | properties.py:214 | Property | partial | DDL metadata is represented only for common cases |
| LikeProperty | properties.py:250 | Property | partial | DDL metadata is represented only for common cases |
| LocationProperty | properties.py:254 | Property | partial | DDL metadata is represented only for common cases |
| LockProperty | properties.py:258 | Property | partial | DDL metadata is represented only for common cases |
| LockingProperty | properties.py:262 | Property | partial | DDL metadata is represented only for common cases |
| LogProperty | properties.py:272 | Property | partial | DDL metadata is represented only for common cases |
| MaskingProperty | properties.py:276 | Property | partial | DDL metadata is represented only for common cases |
| MaterializedProperty | properties.py:280 | Property | partial | DDL metadata is represented only for common cases |
| MergeBlockRatioProperty | properties.py:284 | Property | partial | DDL metadata is represented only for common cases |
| ModuleProperty | properties.py:288 | Property | partial | DDL metadata is represented only for common cases |
| NetworkProperty | properties.py:292 | Property | partial | DDL metadata is represented only for common cases |
| NoPrimaryIndexProperty | properties.py:296 | Property | partial | DDL metadata is represented only for common cases |
| OnCluster | properties.py:242 | Property | partial | DDL metadata is represented only for common cases |
| OnCommitProperty | properties.py:304 | Property | partial | DDL metadata is represented only for common cases |
| OnProperty | properties.py:300 | Property | partial | DDL metadata is represented only for common cases |
| OutputModelProperty | properties.py:196 | Property | partial | DDL metadata is represented only for common cases |
| ParameterStyleProperty | properties.py:148 | Property | partial | DDL metadata is represented only for common cases |
| PartitionBoundSpec | properties.py:362 | Expression | partial | DDL metadata is represented only for common cases |
| PartitionByListProperty | properties.py:340 | Property | partial | DDL metadata is represented only for common cases |
| PartitionByRangeProperty | properties.py:320 | Property | partial | DDL metadata is represented only for common cases |
| PartitionByRangePropertyDynamic | properties.py:324 | Expression | partial | DDL metadata is represented only for common cases |
| PartitionByTruncate | properties.py:316 | Property | partial | DDL metadata is represented only for common cases |
| PartitionList | properties.py:344 | Expression | partial | DDL metadata is represented only for common cases |
| PartitionedByBucket | properties.py:312 | Property | partial | DDL metadata is represented only for common cases |
| PartitionedByProperty | properties.py:308 | Property | partial | DDL metadata is represented only for common cases |
| PartitionedOfProperty | properties.py:372 | Property | partial | DDL metadata is represented only for common cases |
| Properties | properties.py:582 | Expression | partial | DDL metadata is represented only for common cases |
| Property | properties.py:12 | Expression | partial | DDL metadata is represented only for common cases |
| QueryTransform | properties.py:414 | Expression | partial | DDL metadata is represented only for common cases |
| RefreshTriggerProperty | properties.py:348 | Property | partial | DDL metadata is represented only for common cases |
| RemoteWithConnectionModelProperty | properties.py:381 | Property | partial | DDL metadata is represented only for common cases |
| ReturnsProperty | properties.py:385 | Property | partial | DDL metadata is represented only for common cases |
| RollupIndex | properties.py:332 | Expression | partial | DDL metadata is represented only for common cases |
| RollupProperty | properties.py:328 | Property | partial | DDL metadata is represented only for common cases |
| RowAccessProperty | properties.py:336 | Property | partial | DDL metadata is represented only for common cases |
| RowFormatDelimitedProperty | properties.py:397 | Property | partial | DDL metadata is represented only for common cases |
| RowFormatProperty | properties.py:393 | Property | partial | DDL metadata is represented only for common cases |
| RowFormatSerdeProperty | properties.py:410 | Property | partial | DDL metadata is represented only for common cases |
| SampleProperty | properties.py:426 | Property | partial | DDL metadata is represented only for common cases |
| SchemaCommentProperty | properties.py:430 | Property | partial | DDL metadata is represented only for common cases |
| SecureProperty | properties.py:497 | Property | partial | DDL metadata is represented only for common cases |
| SecurityIntegrationProperty | properties.py:501 | Property | partial | DDL metadata is represented only for common cases |
| SemanticView | properties.py:434 | Expression | partial | DDL metadata is represented only for common cases |
| SerdeProperties | properties.py:444 | Property | partial | DDL metadata is represented only for common cases |
| SetConfigProperty | properties.py:456 | Property | partial | DDL metadata is represented only for common cases |
| SetProperty | properties.py:448 | Property | partial | DDL metadata is represented only for common cases |
| SettingsProperty | properties.py:460 | Property | partial | DDL metadata is represented only for common cases |
| SharingProperty | properties.py:452 | Property | partial | DDL metadata is represented only for common cases |
| SortKeyProperty | properties.py:464 | Property | partial | DDL metadata is represented only for common cases |
| SqlReadWriteProperty | properties.py:468 | Property | partial | DDL metadata is represented only for common cases |
| SqlSecurityProperty | properties.py:472 | Property | partial | DDL metadata is represented only for common cases |
| StabilityProperty | properties.py:476 | Property | partial | DDL metadata is represented only for common cases |
| StorageHandlerProperty | properties.py:480 | Property | partial | DDL metadata is represented only for common cases |
| StreamingTableProperty | properties.py:377 | Property | partial | DDL metadata is represented only for common cases |
| StrictProperty | properties.py:389 | Property | partial | DDL metadata is represented only for common cases |
| Tags | properties.py:505 | Property, ColumnConstraintKind | partial | DDL metadata is represented only for common cases |
| TemporaryProperty | properties.py:489 | Property | partial | DDL metadata is represented only for common cases |
| ToTableProperty | properties.py:152 | Property | partial | DDL metadata is represented only for common cases |
| TransformModelProperty | properties.py:520 | Property | partial | DDL metadata is represented only for common cases |
| TransientProperty | properties.py:524 | Property | partial | DDL metadata is represented only for common cases |
| UniqueKeyProperty | properties.py:358 | Property | partial | DDL metadata is represented only for common cases |
| UnloggedProperty | properties.py:528 | Property | partial | DDL metadata is represented only for common cases |
| UsingProperty | properties.py:484 | Property | partial | DDL metadata is represented only for common cases |
| UsingTemplateProperty | properties.py:532 | Property | partial | DDL metadata is represented only for common cases |
| UuidProperty | properties.py:132 | Property | partial | DDL metadata is represented only for common cases |
| ViewAttributeProperty | properties.py:536 | Property | partial | DDL metadata is represented only for common cases |
| VirtualProperty | properties.py:493 | Property | partial | DDL metadata is represented only for common cases |
| VolatileProperty | properties.py:540 | Property | partial | DDL metadata is represented only for common cases |
| WithDataProperty | properties.py:544 | Property | partial | DDL metadata is represented only for common cases |
| WithJournalTableProperty | properties.py:548 | Property | partial | DDL metadata is represented only for common cases |
| WithProcedureOptions | properties.py:566 | Property | partial | DDL metadata is represented only for common cases |
| WithSchemaBindingProperty | properties.py:552 | Property | partial | DDL metadata is represented only for common cases |
| WithSystemVersioningProperty | properties.py:556 | Property | partial | DDL metadata is represented only for common cases |
| BitString | query.py:476 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| ByteString | query.py:485 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| Cube | query.py:634 | Expression | partial | Expr::Cube, group-by integration partial |
| GroupingSets | query.py:642 | Expression | partial | Expr::GroupingSets, group-by integration partial |
| HexString | query.py:480 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| JSONValueArray | query.py:2051 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Limit | query.py:650 | Expression | partial | SelectStatement::limit, fetch/offset variants partial |
| Offset | query.py:841 | Expression | partial | SelectStatement::offset |
| RawString | query.py:490 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| Rollup | query.py:638 | Expression | partial | Expr::Rollup, group-by integration partial |
| TableAlias | query.py:468 | Expression | partial | aliases exist on TableRef/TableSource, SQLGlot's richer alias node is partial |
| UnicodeString | query.py:494 | Expression, Condition | partial | represented by generic operator/predicate nodes where parsed |
| Ascii | string.py:11 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Base64DecodeBinary | string.py:261 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Base64DecodeString | string.py:265 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Base64Encode | string.py:269 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| BitLength | string.py:15 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ByteLength | string.py:19 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Chr | string.py:23 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CityHash64 | string.py:524 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CodePointsToBytes | string.py:273 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CodePointsToString | string.py:277 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Compress | string.py:482 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Concat | string.py:29 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Contains | string.py:38 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ConvertToCharset | string.py:281 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Decode | string.py:285 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DecompressBinary | string.py:508 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DecompressString | string.py:512 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Decrypt | string.py:486 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DecryptRaw | string.py:496 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Elt | string.py:42 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Encode | string.py:289 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Encrypt | string.py:516 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| EncryptRaw | string.py:520 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| EndsWith | string.py:47 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FarmFingerprint | string.py:529 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Format | string.py:52 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FromBase | string.py:293 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FromBase32 | string.py:297 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FromBase64 | string.py:301 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| HexDecodeString | string.py:309 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| IsAscii | string.py:61 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Levenshtein | string.py:74 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MD5Digest | string.py:539 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MD5NumberLower64 | string.py:545 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MD5NumberUpper64 | string.py:549 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MatchAgainst | string.py:89 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Normalize | string.py:93 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| NumberToStr | string.py:97 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Overlay | string.py:101 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Pad | string.py:105 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ParseBignumeric | string.py:577 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ParseNumeric | string.py:581 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ParseUrl | string.py:585 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegexpCount | string.py:411 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegexpExtract | string.py:420 | Expression, Func | partial | TypedFunction::RegexpExtract, dialect coverage partial |
| RegexpExtractAll | string.py:432 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegexpFullMatch | string.py:443 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| RegexpILike | string.py:447 | Expression, Binary, Func | partial | represented by generic operator/predicate nodes where parsed |
| RegexpInstr | string.py:451 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RegexpReplace | string.py:463 | Expression, Func | partial | TypedFunction::RegexpReplace, dialect coverage partial |
| RegexpSplit | string.py:475 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Repeat | string.py:109 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| RtrimmedLength | string.py:125 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SHA1Digest | string.py:557 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SHA2Digest | string.py:566 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SafeConvertBytesToString | string.py:317 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Search | string.py:129 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SearchIp | string.py:140 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Soundex | string.py:144 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SoundexP123 | string.py:148 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Space | string.py:152 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SplitPart | string.py:170 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StandardHash | string.py:570 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StartsWith | string.py:188 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StrPosition | string.py:193 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StrToMap | string.py:203 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| String | string.py:212 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Strtok | string.py:180 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Stuff | string.py:216 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| SubstringIndex | string.py:226 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToBase32 | string.py:321 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToBase64 | string.py:325 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToBinary | string.py:329 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToChar | string.py:333 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToCodePoints | string.py:342 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToDecfloat | string.py:346 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToDouble | string.py:353 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToFile | string.py:361 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToNumber | string.py:369 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Translate | string.py:237 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TryBase64DecodeBinary | string.py:381 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TryBase64DecodeString | string.py:385 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TryHexDecodeBinary | string.py:389 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TryHexDecodeString | string.py:393 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TryToDecfloat | string.py:397 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Unicode | string.py:250 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| AddMonths | temporal.py:76 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ConvertTimezone | temporal.py:411 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentDatetime | temporal.py:29 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentTime | temporal.py:33 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentTimestampLTZ | temporal.py:41 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| CurrentTimezone | temporal.py:45 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Date | temporal.py:288 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DateAdd | temporal.py:80 | Expression, Func, IntervalOp | partial | TypedFunction::DateAdd handles common forms |
| DateBin | temporal.py:84 | Expression, Func, IntervalOp | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DateDiff | temporal.py:88 | Expression, Func, TimeUnit | partial | TypedFunction::DateDiff handles common forms |
| DateFromParts | temporal.py:293 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DateFromUnixDate | temporal.py:298 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DateStrToDate | temporal.py:420 | Expression, Func | partial | temporal functions mostly normalize through TypedFunction |
| DateToDateStr | temporal.py:424 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DateToDi | temporal.py:428 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DateTrunc | temporal.py:165 | Expression, Func | partial | TypedFunction::DateTrunc handles common forms |
| Datetime | temporal.py:302 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DatetimeAdd | temporal.py:104 | Expression, Func, IntervalOp | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DatetimeDiff | temporal.py:108 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DatetimeSub | temporal.py:112 | Expression, Func, IntervalOp | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DatetimeTrunc | temporal.py:161 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DayOfMonth | temporal.py:209 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DayOfWeek | temporal.py:213 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DayOfWeekIso | temporal.py:217 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DayOfYear | temporal.py:221 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Dayname | temporal.py:225 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| DiToDate | temporal.py:432 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| FromISO8601Timestamp | temporal.py:436 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GapFill | temporal.py:306 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateDateArray | temporal.py:318 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GenerateTimestampArray | temporal.py:322 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| GetExtract | temporal.py:233 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Hour | temporal.py:237 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JustifyDays | temporal.py:326 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JustifyHours | temporal.py:330 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| JustifyInterval | temporal.py:334 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| LastDay | temporal.py:338 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Localtime | temporal.py:49 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Localtimestamp | temporal.py:53 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MakeInterval | temporal.py:343 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Minute | temporal.py:241 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Monthname | temporal.py:249 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| MonthsBetween | temporal.py:116 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| NextDay | temporal.py:355 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ParseDatetime | temporal.py:440 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ParseTime | temporal.py:444 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| PreviousDay | temporal.py:359 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Quarter | temporal.py:253 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Second | temporal.py:257 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StrToDate | temporal.py:448 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| StrToUnix | temporal.py:456 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Systimestamp | temporal.py:57 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Time | temporal.py:363 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeAdd | temporal.py:120 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeDiff | temporal.py:124 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeFromParts | temporal.py:367 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeSlice | temporal.py:194 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeStrToDate | temporal.py:460 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeStrToTime | temporal.py:464 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeStrToUnix | temporal.py:468 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeSub | temporal.py:128 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeToStr | temporal.py:472 | Expression, Func | partial | TypedFunction::TimeToStr handles common forms |
| TimeToTimeStr | temporal.py:476 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeToUnix | temporal.py:480 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimeTrunc | temporal.py:198 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Timestamp | temporal.py:380 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampAdd | temporal.py:132 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampDiff | temporal.py:136 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampFromParts | temporal.py:384 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampLtzFromParts | temporal.py:395 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampSub | temporal.py:141 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampTrunc | temporal.py:190 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TimestampTzFromParts | temporal.py:400 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| ToDays | temporal.py:261 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDiToDi | temporal.py:484 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDsAdd | temporal.py:145 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDsDiff | temporal.py:154 | Expression, Func, TimeUnit | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDsToDate | temporal.py:488 | Expression, Func | partial | TypedFunction::TsOrDsToDate handles common forms |
| TsOrDsToDateStr | temporal.py:492 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDsToDatetime | temporal.py:496 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDsToTime | temporal.py:500 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| TsOrDsToTimestamp | temporal.py:504 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixDate | temporal.py:508 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixMicros | temporal.py:512 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixMillis | temporal.py:516 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixSeconds | temporal.py:520 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixToStr | temporal.py:524 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixToTime | temporal.py:528 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UnixToTimeStr | temporal.py:551 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UtcDate | temporal.py:61 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UtcTime | temporal.py:65 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| UtcTimestamp | temporal.py:69 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Week | temporal.py:265 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| WeekOfYear | temporal.py:269 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| YearOfWeek | temporal.py:277 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| YearOfWeekIso | temporal.py:281 | Expression, Func | partial | generic Expr::Function fallback or selected TypedFunction variant |
| Avg | aggregate.py:87 | Expression, AggFunc | supported | Avg |
| DenseRank | aggregate.py:120 | Expression, AggFunc | supported | DenseRank |
| Max | aggregate.py:175 | Expression, AggFunc | supported | Max |
| Min | aggregate.py:184 | Expression, AggFunc | supported | Min |
| Ntile | aggregate.py:202 | Expression, AggFunc | supported | NTile |
| Rank | aggregate.py:244 | Expression, AggFunc | supported | Rank |
| RowNumber | aggregate.py:293 | Expression, Func | supported | RowNumber |
| Stddev | aggregate.py:301 | Expression, AggFunc | supported | Stddev |
| Sum | aggregate.py:313 | Expression, AggFunc | supported | Sum |
| Variance | aggregate.py:317 | Expression, AggFunc | supported | Variance |
| Array | array.py:20 | Expression, Func | supported | Expr::ArrayLiteral |
| ArrayConcat | array.py:54 | Expression, Func | supported | ArrayConcat |
| Flatten | array.py:186 | Expression, Func | supported | Flatten |
| GenerateSeries | array.py:217 | Expression, Func | supported | GenerateSeries |
| Unnest | array.py:259 | Expression, Func, UDTF | supported | TableSource::Unnest |
| Alias | core.py:1872 | Expression | supported | Expr::Alias |
| All | core.py:1589 | Expression, SubqueryPredicate | supported | Expr::AllOp |
| And | core.py:2267 | Expression, Connector, Func | supported | Expr::BinaryOp |
| Any | core.py:1593 | Expression, SubqueryPredicate | supported | Expr::AnyOp |
| ApproxDistinct | core.py:1975 | Expression, AggFunc | supported | ApproxDistinct |
| Between | core.py:2248 | Expression, Predicate | supported | Expr::Between |
| Boolean | core.py:1823 | Expression, Condition | supported | Expr::Boolean |
| Column | core.py:1673 | Expression, Condition | supported | Expr::Column |
| EQ | core.py:2101 | Expression, Binary, Predicate | supported | Expr::BinaryOp |
| Func | core.py:1616 | Condition | supported | Expr::Function |
| ILike | core.py:2141 | Expression, Binary, Predicate | supported | Expr::ILike |
| In | core.py:2256 | Expression, Predicate | supported | Expr::InList/InSubquery |
| Is | core.py:2149 | Expression, Binary, Predicate | supported | Expr::IsNull/IsBool |
| Like | core.py:2153 | Expression, Binary, Predicate | supported | Expr::Like |
| Literal | core.py:1712 | Expression, Condition | supported | Expr::Number/StringLiteral/Boolean/Null |
| Mod | core.py:2169 | Expression, Binary | supported | Mod |
| Null | core.py:1812 | Expression, Condition | supported | Expr::Null |
| Or | core.py:2271 | Expression, Connector, Func | supported | Expr::BinaryOp |
| Parameter | core.py:1796 | Expression, Condition | supported | Expr::Parameter |
| Paren | core.py:2213 | Unary | supported | Expr::Nested |
| Pow | core.py:2280 | Expression, Binary, Func | supported | Pow |
| Star | core.py:1784 | Expression | supported | Expr::Star/Wildcard |
| DataType | datatypes.py:179 | Expression | supported | DataType |
| Interval | datatypes.py:449 | _TimeUnit | supported | Expr::Interval |
| Create | ddl.py:39 | Expression, DDL | supported | Statement::CreateTable/CreateView |
| Drop | ddl.py:337 | Expression | supported | Statement::DropTable/DropView |
| Transaction | ddl.py:369 | Expression | supported | Transaction |
| Use | ddl.py:410 | Expression | supported | Statement::Use |
| Delete | dml.py:71 | Expression, DML | supported | Statement::Delete |
| Insert | dml.py:195 | Expression, DDL, DML | supported | Statement::Insert |
| Merge | dml.py:507 | Expression, DML | supported | Statement::Merge |
| Update | dml.py:294 | Expression, DML | supported | Statement::Update |
| Case | functions.py:96 | Expression, Func | supported | Expr::Case |
| Cast | functions.py:34 | Expression, Func | supported | Expr::Cast |
| Coalesce | functions.py:122 | Expression, Func | supported | Expr::Coalesce |
| Collate | functions.py:166 | Expression, Binary, Func | supported | Collate |
| Exists | functions.py:182 | Expression, Func, SubqueryPredicate | supported | Expr::Exists |
| Greatest | functions.py:137 | Expression, Func | supported | Greatest |
| If | functions.py:91 | Expression, Func | supported | Expr::If |
| Least | functions.py:142 | Expression, Func | supported | Least |
| Nullif | functions.py:147 | Expression, Func | supported | Expr::NullIf |
| TryCast | functions.py:72 | Cast | supported | Expr::TryCast |
| JSONFormat | json.py:141 | Expression, Func | supported | JSONFormat |
| ParseJSON | json.py:234 | Expression, Func | supported | ParseJSON |
| Abs | math.py:126 | Expression, Func | supported | Abs |
| Ceil | math.py:134 | Expression, Func | supported | Ceil |
| Floor | math.py:147 | Expression, Func | supported | Floor |
| Ln | math.py:159 | Expression, Func | supported | Ln |
| Log | math.py:163 | Expression, Func | supported | Log |
| Round | math.py:171 | Expression, Func | supported | Round |
| Sqrt | math.py:184 | Expression, Func | supported | Sqrt |
| From | query.py:544 | Expression | supported | FromClause/TableSource |
| Group | query.py:623 | Expression | supported | SelectStatement::group_by |
| Join | query.py:668 | Expression | supported | JoinClause |
| Lambda | query.py:646 | Expression | supported | Expr::Lambda |
| Lateral | query.py:796 | Expression, UDTF | supported | Lateral |
| Order | query.py:845 | Expression | supported | SelectStatement::order_by |
| Pivot | query.py:1747 | Expression | supported | TableSource::Pivot |
| Select | query.py:1128 | Expression, Query | supported | Statement::Select |
| SetOperation | query.py:1021 | Expression, Query | supported | SetOperation |
| Subquery | query.py:1667 | Expression, DerivedTable, Query | supported | Expr::Subquery/TableSource::Subquery |
| Table | query.py:943 | Expression, Selectable | supported | TableSource::Table |
| Tuple | query.py:886 | Expression | supported | Expr::Tuple |
| Union | query.py:1084 | SetOperation | supported | Statement::SetOperation |
| Window | query.py:1845 | Expression, Condition | supported | WindowSpec/Expr::TypedFunction.over |
| With | query.py:446 | Expression | supported | SelectStatement::ctes |
| ConcatWs | string.py:34 | Concat | supported | ConcatWs |
| Hex | string.py:305 | Expression, Func | supported | Hex |
| Initcap | string.py:57 | Expression, Func | supported | Initcap |
| Left | string.py:65 | Expression, Func | supported | Left |
| Length | string.py:69 | Expression, Func | supported | Length |
| Lower | string.py:85 | Expression, Func | supported | Lower |
| MD5 | string.py:535 | Expression, Func | supported | Md5 |
| Replace | string.py:113 | Expression, Func | supported | Replace |
| Reverse | string.py:117 | Expression, Func | supported | Reverse |
| Right | string.py:121 | Expression, Func | supported | Right |
| SHA | string.py:553 | Expression, Func | supported | Sha |
| SHA2 | string.py:561 | Expression, Func | supported | Sha2 |
| Split | string.py:160 | Expression, Func | supported | Split |
| Substring | string.py:221 | Expression, Func | supported | Substring |
| Trim | string.py:241 | Expression, Func | supported | Trim |
| Unhex | string.py:404 | Expression, Func | supported | Unhex |
| Upper | string.py:254 | Expression, Func | supported | Upper |
| CurrentDate | temporal.py:25 | Expression, Func | supported | CurrentDate |
| CurrentTimestamp | temporal.py:37 | Expression, Func | supported | CurrentTimestamp |
| DateSub | temporal.py:100 | Expression, Func, IntervalOp | supported | DateSub |
| Day | temporal.py:205 | Expression, Func | supported | Day |
| Extract | temporal.py:229 | Expression, Func | supported | Expr::Extract |
| Month | temporal.py:245 | Expression, Func | supported | Month |
| StrToTime | temporal.py:452 | Expression, Func | supported | StrToTime |
| Year | temporal.py:273 | Expression, Func | supported | Year |
| AIAgg | aggregate.py:8 | Expression, AggFunc | out-of-scope | vendor/AI aggregate long tail |
| AISummarizeAgg | aggregate.py:13 | Expression, AggFunc | out-of-scope | vendor/AI aggregate long tail |
| CollateProperty | properties.py:78 | Property | out-of-scope | DDL property long tail |
| EngineProperty | properties.py:128 | Property | out-of-scope | DDL property long tail |
| FileFormatProperty | properties.py:168 | Property | out-of-scope | DDL property long tail |
| IcebergProperty | properties.py:184 | Property | out-of-scope | DDL property long tail |

## Update Command

```bash
cargo run --bin xtask -- inventory-ast --sqlglot /path/to/sqlglot
```
