## [0.0.35] - 2026-10-07

### Other Changes
- Chore(deps): update dependencies to v1.49.0 (#1546) ([#1546](https://github.com/csskit/csskit/pull/1546))


### Chromashift
- chromashift: model RGB colour spaces as first class values (#1553) ([#1553](https://github.com/csskit/csskit/pull/1553))
- chromashift: Align serialisation to CSS Spec (#1555) ([#1555](https://github.com/csskit/csskit/pull/1555))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#1545) ([#1545](https://github.com/csskit/csskit/pull/1545))


### Csskit_vscode
- chore(deps): update dependency @types/vscode to v1.140.0 (#1548) ([#1548](https://github.com/csskit/csskit/pull/1548))

## [0.0.34] - 2026-09-30

### Bytescan
- chore(deps): update rust crate fearless_simd to v1 (#1530) ([#1530](https://github.com/csskit/csskit/pull/1530))


### Css_ast
- css_parse/css_ast: move the <an+b> parse driver into a trait (#1524) ([#1524](https://github.com/csskit/csskit/pull/1524))
- Regenerate css_ast/src/values from csswg drafts (#1506) ([#1506](https://github.com/csskit/csskit/pull/1506))


### Css_parse
- fuzz: new parser findings 2026-09-09 (#1515) ([#1515](https://github.com/csskit/csskit/pull/1515))


### Csskit
- csskit: add bin back to pakcage.json (#1539) ([#1539](https://github.com/csskit/csskit/pull/1539))
- Release v0.0.34 (#1525) ([#1525](https://github.com/csskit/csskit/pull/1525))


### Csskit_napi
- chore(deps): update dependency @napi-rs/cli to v3.10.5 (#1526) ([#1526](https://github.com/csskit/csskit/pull/1526))


### Csskit_vscode
- chore(deps): update dependency @types/node to v24.19.0 (#1536) ([#1536](https://github.com/csskit/csskit/pull/1536))
- chore(deps): update dependency @types/vscode to v1.138.0 (#1527) ([#1527](https://github.com/csskit/csskit/pull/1527))

## [0.0.33] - 2026-09-19

### Csskit
- Release v0.0.33 (#1523) ([#1523](https://github.com/csskit/csskit/pull/1523))


### Derive_atom_set
- derive_atom_set: Fix devdep version (#1522) ([#1522](https://github.com/csskit/csskit/pull/1522))

## [0.0.32] - 2026-09-19

### Atom_set
- atom_set: Split & separately publish `atom_set` (#1521) ([#1521](https://github.com/csskit/csskit/pull/1521))


### Css_ast
- csskit_spec_generator: Ensure longhand emission is ordered (#1478) ([#1478](https://github.com/csskit/csskit/pull/1478))
- Regenerate css_ast/src/values from csswg drafts (#1480) ([#1480](https://github.com/csskit/csskit/pull/1480))
- csskit_spec_generator/csskit_derives/csskit_ast: Rewrite shorthand/longhand meta (#1486) ([#1486](https://github.com/csskit/csskit/pull/1486))
- css_lexer: keep the `--` of a dashed ident in its atom (#1489) ([#1489](https://github.com/csskit/csskit/pull/1489))
- css_parse: compare SemanticEq nodes through their source text (#1490) ([#1490](https://github.com/csskit/csskit/pull/1490))
- csskit_derives/css_parse: rewrite derive(ToSpan) (#1492) ([#1492](https://github.com/csskit/csskit/pull/1492))
- csskit_derives: rewrite derive(Parse) (#1499) ([#1499](https://github.com/csskit/csskit/pull/1499))
- css_ast: Drop mathml extended elements (#1502) ([#1502](https://github.com/csskit/csskit/pull/1502))
- csskit_spec_generator/csskit_derives: more shorthands metadata improvements (#1504) ([#1504](https://github.com/csskit/csskit/pull/1504))
- css_parse: move AttributeOperator down from css_ast (#1514) ([#1514](https://github.com/csskit/csskit/pull/1514))


### Css_lexer
- css_lexer: DRY up token parsing with ParsedChars iterator (#1487) ([#1487](https://github.com/csskit/csskit/pull/1487))
- css_lexer: Mark KindSet::contains const (#1491) ([#1491](https://github.com/csskit/csskit/pull/1491))


### Csskit
- chore(deps): update dependencies (patch) (#1482) ([#1482](https://github.com/csskit/csskit/pull/1482))
- Release v0.0.32 (#1479) ([#1479](https://github.com/csskit/csskit/pull/1479))


### Csskit_derives
- csskit_derives: rewrite derive(ToCursors) (#1493) ([#1493](https://github.com/csskit/csskit/pull/1493))
- csskit_derives: rewrite derive(SemanticEq) (#1494) ([#1494](https://github.com/csskit/csskit/pull/1494))
- csskit_derives: rewrite derive(Visitable) (#1495) ([#1495](https://github.com/csskit/csskit/pull/1495))
- csskit_derives: derive(NodeWithMetadata) metadata body (#1496) ([#1496](https://github.com/csskit/csskit/pull/1496))
- csskit_derives: rewrite derive(Peek) (#1497) ([#1497](https://github.com/csskit/csskit/pull/1497))
- csskit_derives: drop synstructure (#1498) ([#1498](https://github.com/csskit/csskit/pull/1498))


### Csskit_napi
- chore(deps): update dependency @napi-rs/cli to v3.9.0 (#1508) ([#1508](https://github.com/csskit/csskit/pull/1508))


### Csskit_transform
- coverage: update css-minify-tests (#1505) ([#1505](https://github.com/csskit/csskit/pull/1505))


### Csskit_vscode
- chore(deps): update dependency @types/vscode to v1.134.0 (#1483) ([#1483](https://github.com/csskit/csskit/pull/1483))
- chore(deps): update dependency oxlint to v1.79.0 (#1485) ([#1485](https://github.com/csskit/csskit/pull/1485))
- chore(deps): update dependency mocha to v12 (#1488) ([#1488](https://github.com/csskit/csskit/pull/1488))
- chore(deps): update dependency @types/vscode to v1.136.0 (#1510) ([#1510](https://github.com/csskit/csskit/pull/1510))
- chore(deps): update dependency oxlint to v1.81.0 (#1513) ([#1513](https://github.com/csskit/csskit/pull/1513))
- chore(deps): update dependency oxlint to v1.82.0 (#1520) ([#1520](https://github.com/csskit/csskit/pull/1520))
- chore(deps): update dependency @types/vscode to v1.137.0 (#1516) ([#1516](https://github.com/csskit/csskit/pull/1516))

## [0.0.31] - 2026-08-28

### Other Changes
- Release: re-add `@release comments` (#1475) ([#1475](https://github.com/csskit/csskit/pull/1475))


### Bytescan
- bytescan/css_lexer: Factor out SIMD scanning into new library (#1466) ([#1466](https://github.com/csskit/csskit/pull/1466))


### Chromashift
- workspace: Globalise linting and clean up warnings (#1455) ([#1455](https://github.com/csskit/csskit/pull/1455))


### Css_ast
- css_parse: Consume multiple semicolons loose in Stylesheet (#1452) ([#1452](https://github.com/csskit/csskit/pull/1452))
- css_ast: Add more missing pseudos (#1453) ([#1453](https://github.com/csskit/csskit/pull/1453))
- css_ast: implement QueryableNode for unknown tag/at-rule nodes (#1457) ([#1457](https://github.com/csskit/csskit/pull/1457))
- csskit_spec_generator/css_ast: Add concept of shorthand resets (#1461) ([#1461](https://github.com/csskit/csskit/pull/1461))
- css_ast/csskit_spec_generator: Complete shorthand/longhand metadata (#1470) ([#1470](https://github.com/csskit/csskit/pull/1470))


### Css_parse
- css_parse/csskit_arena/source_tools: Move Vec/Box/String into csskit_arena (#1473) ([#1473](https://github.com/csskit/csskit/pull/1473))


### Csskit
- csskit: separate parse/check failures, add find --group (#1460) ([#1460](https://github.com/csskit/csskit/pull/1460))
- chore(deps): update dependencies (patch) (#1458) ([#1458](https://github.com/csskit/csskit/pull/1458))
- Release v0.0.31 (#1456) ([#1456](https://github.com/csskit/csskit/pull/1456))


### Csskit_arena
- csskit_area/derive_atom_set: Fold readme into lib docs (#1462) ([#1462](https://github.com/csskit/csskit/pull/1462))


### Csskit_napi
- chore(deps): update dependency @napi-rs/cli to v3.8.6 (#1459) ([#1459](https://github.com/csskit/csskit/pull/1459))
- csskit_napi: Add binary to cargo.toml (#1468) ([#1468](https://github.com/csskit/csskit/pull/1468))
- csskit_napi: Don't publish to registry (#1477) ([#1477](https://github.com/csskit/csskit/pull/1477))


### Csskit_proc_macro
- csskit_proc_macro: Add full feature flag to syn dep (#1454) ([#1454](https://github.com/csskit/csskit/pull/1454))


### Csskit_transform
- coverage: update css-minify-tests (#1469) ([#1469](https://github.com/csskit/csskit/pull/1469))
- csskit_transform: Introduce RemoveOveriddenDeclarations transform (#1471) ([#1471](https://github.com/csskit/csskit/pull/1471))
- csskit_transform: Introduce ReduceShorthandValues (#1472) ([#1472](https://github.com/csskit/csskit/pull/1472))
- release: re-add `@release comments` (#1474) ([#1474](https://github.com/csskit/csskit/pull/1474))
- coverage: update css-minify-tests (#1476) ([#1476](https://github.com/csskit/csskit/pull/1476))

## [0.0.30] - 2026-08-23

### Other Changes
- Fuzz: Minify corpus (#1444) ([#1444](https://github.com/csskit/csskit/pull/1444))
- Build: optimise for size, not speed (#1448) ([#1448](https://github.com/csskit/csskit/pull/1448))
- Build: lto=thin (#1449) ([#1449](https://github.com/csskit/csskit/pull/1449))


### Css_ast
- css_ast: Allow symbolic-counter-style in content syntax (#1419) ([#1419](https://github.com/csskit/csskit/pull/1419))
- css_ast: Boxup rare nodes to reduce allocation size (#1421) ([#1421](https://github.com/csskit/csskit/pull/1421))
- css_ast: Clean up & improve NodeWithMetadata derives, add value_kinds (#1422) ([#1422](https://github.com/csskit/csskit/pull/1422))
- csskit_transform: implement ReduceCharsetRule (#1427) ([#1427](https://github.com/csskit/csskit/pull/1427))
- css_ast: Add missing webkit pseudo elements (#1428) ([#1428](https://github.com/csskit/csskit/pull/1428))
- css_ast: Implement InitialLetterAlignStyleValue (#1429) ([#1429](https://github.com/csskit/csskit/pull/1429))
- css_ast: Allow prelude-less `@layer` (#1434) ([#1434](https://github.com/csskit/csskit/pull/1434))
- css_ast/css_parse/csskit_derives: Derive empty block, prelude, nested metadata kinds (#1435) ([#1435](https://github.com/csskit/csskit/pull/1435))
- css_ast/csskit_derives: Add Effective/Inert NodeKind flags (#1436) ([#1436](https://github.com/csskit/csskit/pull/1436))
-  chore(deps): update Rust deps & format caro files (#1437) ([#1437](https://github.com/csskit/csskit/pull/1437))
- Regenerate css_ast/src/values from csswg drafts (#1439) ([#1439](https://github.com/csskit/csskit/pull/1439))
- css_parse: Expose diagnostic details (#1440) ([#1440](https://github.com/csskit/csskit/pull/1440))
- source_tools: Extract out Cursor/Span/LineIndex etc into source_tools crate (#1441) ([#1441](https://github.com/csskit/csskit/pull/1441))
- css_ast: Support dynamically Visiting & Parsing (#1442) ([#1442](https://github.com/csskit/csskit/pull/1442))
- csskit_ast: Use ErasedNode in selector matching, add NodeKey for identity (#1443) ([#1443](https://github.com/csskit/csskit/pull/1443))


### Css_lexer
- css_lexer/css_parse: Ensure parsed whitespace survives minification (#1420) ([#1420](https://github.com/csskit/csskit/pull/1420))


### Csskit
- chore(deps): update dependencies (patch) (#1407) ([#1407](https://github.com/csskit/csskit/pull/1407))
- csskit_napi: Add native Node.js bindings (#1445) ([#1445](https://github.com/csskit/csskit/pull/1445))
- Release v0.0.30 (#1415) ([#1415](https://github.com/csskit/csskit/pull/1415))
- node: Stop doubling wasm builds! (#1447) ([#1447](https://github.com/csskit/csskit/pull/1447))


### Csskit_arena
- csskit_arena: Reuse reservations across arenas (#1430) ([#1430](https://github.com/csskit/csskit/pull/1430))
- csskit_arena: Clean up and consolidate allocation modes (#1431) ([#1431](https://github.com/csskit/csskit/pull/1431))
- csskit_arena: Add a detailed readme (#1433) ([#1433](https://github.com/csskit/csskit/pull/1433))


### Csskit_transform
- coverage: update css-minify-tests (#1423) ([#1423](https://github.com/csskit/csskit/pull/1423))
- csskit_transform: skip subtrees which don't match metadata (#1424) ([#1424](https://github.com/csskit/csskit/pull/1424))
- csskit_transform: implement RemoveInertNodes (#1438) ([#1438](https://github.com/csskit/csskit/pull/1438))


### Csskit_vscode
- chore(deps): update dependency mocha to v11.8.0 (#1410) ([#1410](https://github.com/csskit/csskit/pull/1410))
- chore(deps): update dependency oxlint to v1.76.0 (#1411) ([#1411](https://github.com/csskit/csskit/pull/1411))
- chore(deps): update dependency oxlint to v1.78.0 (#1413) ([#1413](https://github.com/csskit/csskit/pull/1413))


### Derive_atom_set
- derive_atom_set: Don't case fold on punctuation. (#1426) ([#1426](https://github.com/csskit/csskit/pull/1426))
- derive_atom_set: Expand readme with more detail (#1432) ([#1432](https://github.com/csskit/csskit/pull/1432))

## [0.0.29] - 2026-08-14

### Other Changes
- Csskit_spec_generator: Move out of crates/ into tools/ (#1331) ([#1331](https://github.com/csskit/csskit/pull/1331))
- Default debug builds to line-tables-only (#1333) ([#1333](https://github.com/csskit/csskit/pull/1333))
- Chore(deps): update dependencies (patch) (#1335) ([#1335](https://github.com/csskit/csskit/pull/1335))
- Fuzz: Use css_parse sepcified allocator (#1339) ([#1339](https://github.com/csskit/csskit/pull/1339))
- Csskit_spec_generator: Pass workspace_root to generate_property_atoms (#1368) ([#1368](https://github.com/csskit/csskit/pull/1368))
- Csskit_spec_generator: Read & emit commit sha when generating individual specs (#1383) ([#1383](https://github.com/csskit/csskit/pull/1383))


### Chromashift
- css_parse/css_ast: Use a #[node] tag to simplify csskit_source_finder, size tests (#1323) ([#1323](https://github.com/csskit/csskit/pull/1323))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#1311) ([#1311](https://github.com/csskit/csskit/pull/1311))
- chore(deps): update dependency rust to v1.97.1 (#1322) ([#1322](https://github.com/csskit/csskit/pull/1322))
- css_parse: Re-export `Arena` as Bumpalo, rename `bump` to `alloc` (#1326) ([#1326](https://github.com/csskit/csskit/pull/1326))
- csskit_arena: Introduce csskit_arena (#1327) ([#1327](https://github.com/csskit/csskit/pull/1327))
- Fix clippy errors (#1329) ([#1329](https://github.com/csskit/csskit/pull/1329))
- css_ast: repr(C) all nodes, introduce stable-type-layout. (#1334) ([#1334](https://github.com/csskit/csskit/pull/1334))
- css_parse/css_ast/csskit_wasm: Make csskit_arena default allocator (#1340) ([#1340](https://github.com/csskit/csskit/pull/1340))
- css_ast: Implement ray() function (#1342) ([#1342](https://github.com/csskit/csskit/pull/1342))
- css_ast: Implement palette-mix() function (#1343) ([#1343](https://github.com/csskit/csskit/pull/1343))
- css_ast: Implement calc-size() function (#1344) ([#1344](https://github.com/csskit/csskit/pull/1344))
- css_ast: Implement Autospace (#1345) ([#1345](https://github.com/csskit/csskit/pull/1345))
- css_ast: Implement TextEdge (#1346) ([#1346](https://github.com/csskit/csskit/pull/1346))
- css_ast: Implement ContentLevel (#1347) ([#1347](https://github.com/csskit/csskit/pull/1347))
- csskit_arena/css_parse/css_lexer/css_ast: Implement chunk based allocations, drop bumpalo (#1348) ([#1348](https://github.com/csskit/csskit/pull/1348))
- css_ast: Implement [Auto]Line(Width|Style|Color)(List|OrRepeat) (#1349) ([#1349](https://github.com/csskit/csskit/pull/1349))
- css_ast: Implement SingleAnimation (#1350) ([#1350](https://github.com/csskit/csskit/pull/1350))
- css_ast: Implement CSS 4 images functions (#1351) ([#1351](https://github.com/csskit/csskit/pull/1351))
- css_ast: Implement fade() function (#1352) ([#1352](https://github.com/csskit/csskit/pull/1352))
- css_ast: Implement mask-layer() (#1353) ([#1353](https://github.com/csskit/csskit/pull/1353))
- css_ast: Implement TimelineRangeCenterSubject (#1354) ([#1354](https://github.com/csskit/csskit/pull/1354))
- css_ast: Implement remaining todo media features (#1355) ([#1355](https://github.com/csskit/csskit/pull/1355))
- css_ast: Implement @scope rule (#1356) ([#1356](https://github.com/csskit/csskit/pull/1356))
- css_ast: Implement @color-profile rule (#1357) ([#1357](https://github.com/csskit/csskit/pull/1357))
- css_ast: Implement IntegerOrInfinite, uncomment RangeStyleValue (#1358) ([#1358](https://github.com/csskit/csskit/pull/1358))
- css_ast: Implement @font-feature-values (#1359) ([#1359](https://github.com/csskit/csskit/pull/1359))
- css_ast: Implement @font-palette-values (#1360) ([#1360](https://github.com/csskit/csskit/pull/1360))
- css_ast: Implement Syntax (#1361) ([#1361](https://github.com/csskit/csskit/pull/1361))
- css_ast: Implement tree counting functions, NumericValue<T> (#1362) ([#1362](https://github.com/csskit/csskit/pull/1362))
- css_ast: Drop Todo, clean up imports (#1364) ([#1364](https://github.com/csskit/csskit/pull/1364))
- csskit_ast: Unflag stable_type_layout from test. (#1367) ([#1367](https://github.com/csskit/csskit/pull/1367))
- Regenerate css_ast/src/values & css_feature_data from csswg drafts (#1369) ([#1369](https://github.com/csskit/csskit/pull/1369))
- css_ast: Generate Value<> slots for all keywords (#1370) ([#1370](https://github.com/csskit/csskit/pull/1370))
- css_ast: Fix errors in MathML tags (#1371) ([#1371](https://github.com/csskit/csskit/pull/1371))
- css_ast: Add new pseudo classes from various specs (#1372) ([#1372](https://github.com/csskit/csskit/pull/1372))
- css_ast/csskit_ast: Optimise NodeId code (#1373) ([#1373](https://github.com/csskit/csskit/pull/1373))
- css_ast: Fix panic in unexpected_pseudo_class diagnostic (#1374) ([#1374](https://github.com/csskit/csskit/pull/1374))
- css_ast: Delete dead macro code (#1376) ([#1376](https://github.com/csskit/csskit/pull/1376))
- css_parse/css_ast: Ensure ComponentValue(s) are visitable nodes (#1377) ([#1377](https://github.com/csskit/csskit/pull/1377))
- css_ast: Implement RotateStyleValue (#1378) ([#1378](https://github.com/csskit/csskit/pull/1378))
- css_ast: Implement nav-* style values (#1379) ([#1379](https://github.com/csskit/csskit/pull/1379))
- css_ast: Implement voice-pitch/voice-range, with semitones, fixing frequency (#1380) ([#1380](https://github.com/csskit/csskit/pull/1380))
- css_ast: Implement remaining background properties (#1381) ([#1381](https://github.com/csskit/csskit/pull/1381))
- css_ast: Implement offset style value (#1382) ([#1382](https://github.com/csskit/csskit/pull/1382))
- css_ast: IMplement font-variant-alternates style value (#1384) ([#1384](https://github.com/csskit/csskit/pull/1384))
- css_ast: Implement mask-border style value (#1385) ([#1385](https://github.com/csskit/csskit/pull/1385))
- css_ast: Implement border-image style value (#1386) ([#1386](https://github.com/csskit/csskit/pull/1386))
- csskit_proc_macro: Don't eagerly optimise alternates (#1387) ([#1387](https://github.com/csskit/csskit/pull/1387))
- css_ast: Implement FontVariantStyleValue (#1388) ([#1388](https://github.com/csskit/csskit/pull/1388))
- css_ast: Add revert-rule keyword (#1393) ([#1393](https://github.com/csskit/csskit/pull/1393))
- css_ast: Add missing pseudo elements from various specs (#1394) ([#1394](https://github.com/csskit/csskit/pull/1394))
- css_ast: Implement -webkit-animation style value (#1396) ([#1396](https://github.com/csskit/csskit/pull/1396))
- css_ast: ...actually implement -webkit-animation (#1397) ([#1397](https://github.com/csskit/csskit/pull/1397))
- css_ast: Group -webkit-animation properties (#1398) ([#1398](https://github.com/csskit/csskit/pull/1398))
- css_ast: Implement font shorthand (#1399) ([#1399](https://github.com/csskit/csskit/pull/1399))
- css_ast: Implement @font-face style values (#1403) ([#1403](https://github.com/csskit/csskit/pull/1403))


### Css_parse
- css_parse: Implement an Arena allocated String (#1324) ([#1324](https://github.com/csskit/csskit/pull/1324))
- css_parse/csskit/csskit_transform: Use css_parse::String where possible (#1325) ([#1325](https://github.com/csskit/csskit/pull/1325))
- css_parse: Clean up imports with preludes (#1365) ([#1365](https://github.com/csskit/csskit/pull/1365))


### Css_value_definition_parser
- csskit_source_finder/css_value_definition_parser: Autogeenerate list of sized types (#1363) ([#1363](https://github.com/csskit/csskit/pull/1363))
- csskit_source_finder: Ignore types inside test modules (#1400) ([#1400](https://github.com/csskit/csskit/pull/1400))


### Csskit
- chore(deps): update dependencies (patch) (#1312) ([#1312](https://github.com/csskit/csskit/pull/1312))
- csskit-wasm: Fix wasm-opt errors (#1330) ([#1330](https://github.com/csskit/csskit/pull/1330))
- csskit: Clean up imports with preludes (#1366) ([#1366](https://github.com/csskit/csskit/pull/1366))
- Release v0.0.29 (#1314) ([#1314](https://github.com/csskit/csskit/pull/1314))


### Csskit_arena
- csskit_arena: Add with_initial_capacity (#1375) ([#1375](https://github.com/csskit/csskit/pull/1375))


### Csskit_lsp
- csskit_lsp: Don't create new Arenas for each LineIndex (#1341) ([#1341](https://github.com/csskit/csskit/pull/1341))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.73.0 (#1318) ([#1318](https://github.com/csskit/csskit/pull/1318))
- chore(deps): update dependency @vscode/test-electron to v3.1.0 (#1313) ([#1313](https://github.com/csskit/csskit/pull/1313))
- chore(deps): update dependency oxlint to v1.75.0 (#1320) ([#1320](https://github.com/csskit/csskit/pull/1320))
- chore(deps): update dependency prettier to v3.9.6 (#1321) ([#1321](https://github.com/csskit/csskit/pull/1321))
- chore(deps): update dependencies (patch) (#1389) ([#1389](https://github.com/csskit/csskit/pull/1389))

## [0.0.28] - 2026-07-25

### Other Changes
- Chore(deps): update dependencies (patch) (#1300) ([#1300](https://github.com/csskit/csskit/pull/1300))


### Css_ast
- csskit_derives: Add peek(skip) attribute (#1266) ([#1266](https://github.com/csskit/csskit/pull/1266))
- Regenerate css_ast/src/values from csswg drafts (#1267) ([#1267](https://github.com/csskit/csskit/pull/1267))
- css_parse/csskit_derives: Add semantic_eq(skip) to derive(SemanticEq) (#1272) ([#1272](https://github.com/csskit/csskit/pull/1272))
- css_ast: Implement various shape functions for clip-path, etc (#1275) ([#1275](https://github.com/csskit/csskit/pull/1275))
- css_ast: Use blocks for Container & Style at rules (#1276) ([#1276](https://github.com/csskit/csskit/pull/1276))
- css_ast: Add more -moz-meter-* pseudo elements/classes (#1277) ([#1277](https://github.com/csskit/csskit/pull/1277))
- css_ast: Refactor and improve `@supports` parsing (#1278) ([#1278](https://github.com/csskit/csskit/pull/1278))
- css_ast: Implement fill & stroke (#1279) ([#1279](https://github.com/csskit/csskit/pull/1279))
- css_ast: Implement webkit-text-stroek (#1280) ([#1280](https://github.com/csskit/csskit/pull/1280))
- css_ast Implement grid functions, track, line, etc (#1281) ([#1281](https://github.com/csskit/csskit/pull/1281))
- css_ast/css_parse: Implement ToNormalisedValue trait (#1284) ([#1284](https://github.com/csskit/csskit/pull/1284))
- Regenerate css_ast/src/values from csswg drafts (#1285) ([#1285](https://github.com/csskit/csskit/pull/1285))
- Regenerate css_ast/src/values from csswg drafts (#1294) ([#1294](https://github.com/csskit/csskit/pull/1294))
- csskit_derives: Allow multiple fields to delegate metadata (#1295) ([#1295](https://github.com/csskit/csskit/pull/1295))
- css_parse: Reimplement computed value declaration peeking/parsing in DeclarationValue (#1298) ([#1298](https://github.com/csskit/csskit/pull/1298))
- css_parse: Refactor BumpBox into generic Arena allocating Box. (#1304) ([#1304](https://github.com/csskit/csskit/pull/1304))
- css_parse: Implement an Arena allocated Vec. (#1306) ([#1306](https://github.com/csskit/csskit/pull/1306))
- css_ast: Build out substitution function architecture (#1308) ([#1308](https://github.com/csskit/csskit/pull/1308))
- css_ast: Use Value/CalcableValue in most manual types (#1309) ([#1309](https://github.com/csskit/csskit/pull/1309))
- css_ast: Add value slots for more types (#1310) ([#1310](https://github.com/csskit/csskit/pull/1310))


### Css_feature_data
- Regenerate css_ast/src/values from csswg drafts (#1299) ([#1299](https://github.com/csskit/csskit/pull/1299))


### Css_lexer
- css_lexer/csskit: Implement LineIndex, precomputing line offsets (#1264) ([#1264](https://github.com/csskit/csskit/pull/1264))
- css_lexer: Use a boxed slice in LineIndex, dropping Bumpalo's Vec. (#1305) ([#1305](https://github.com/csskit/csskit/pull/1305))
- css_lexer: Implement fearless_simd paths for scanning (#1307) ([#1307](https://github.com/csskit/csskit/pull/1307))


### Css_parse
- fuzz: new parser findings 2026-07-13 (#1288) ([#1288](https://github.com/csskit/csskit/pull/1288))
- css_parse: Drop CursorSource trait (#1296) ([#1296](https://github.com/csskit/csskit/pull/1296))
- css_parse: Drop PreludeList, DeclarationRuleList (#1297) ([#1297](https://github.com/csskit/csskit/pull/1297))


### Csskit
- chore(deps): update dependencies (patch) (#1268) ([#1268](https://github.com/csskit/csskit/pull/1268))
- Release v0.0.28 (#1265) ([#1265](https://github.com/csskit/csskit/pull/1265))


### Csskit_derives
- csskit_derives: Ensure atom checks properly peek (#1282) ([#1282](https://github.com/csskit/csskit/pull/1282))


### Csskit_spec_generator
- csskit_spec_generator: Ensure property atoms includes the extra properties (#1283) ([#1283](https://github.com/csskit/csskit/pull/1283))


### Csskit_vscode
- chore(deps): update dependency @types/node to v24.13.2 (#1269) ([#1269](https://github.com/csskit/csskit/pull/1269))
- chore(deps): update dependency @types/vscode to v1.125.0 (#1270) ([#1270](https://github.com/csskit/csskit/pull/1270))
- chore(deps): update dependency typescript to v7 (#1273) ([#1273](https://github.com/csskit/csskit/pull/1273))
- chore(deps): update dependencies (patch) (#1286) ([#1286](https://github.com/csskit/csskit/pull/1286))
- chore(deps): update dependency oxlint to v1.69.0 (#1303) ([#1303](https://github.com/csskit/csskit/pull/1303))

## [0.0.27] - 2026-07-04

### Other Changes
- Fuzz: minimize corpus, and do so on nightly fuzz build (#1257) ([#1257](https://github.com/csskit/csskit/pull/1257))
- CI: Ensure benches build (#1259) ([#1259](https://github.com/csskit/csskit/pull/1259))


### Css_ast
- css_ast/css_parse: Improve peeking robustness. (#1238) ([#1238](https://github.com/csskit/csskit/pull/1238))
- Regenerate css_ast/src/values from csswg drafts (#1242) ([#1242](https://github.com/csskit/csskit/pull/1242))
- Regenerate css_ast/src/values from csswg drafts (#1254) ([#1254](https://github.com/csskit/csskit/pull/1254))
- fuzz: new parser findings 2026-06-19 (#1232) ([#1232](https://github.com/csskit/csskit/pull/1232))
- css_ast: Refactor assert_specificity to use closure pattern (#1261) ([#1261](https://github.com/csskit/csskit/pull/1261))
- css_ast: derive(ToSpan) on WebkitFunctionalPseudoClass (#1262) ([#1262](https://github.com/csskit/csskit/pull/1262))
- Implement "VisitFlow" (#1263) ([#1263](https://github.com/csskit/csskit/pull/1263))


### Css_parse
- fuzz: new parser findings 2026-07-01 (#1260) ([#1260](https://github.com/csskit/csskit/pull/1260))


### Csskit
- chore(deps): update dependencies (patch) (#1243) ([#1243](https://github.com/csskit/csskit/pull/1243))
- Release v0.0.27 (#1239) ([#1239](https://github.com/csskit/csskit/pull/1239))


### Csskit_derives
- chore(deps): update rust crate itertools to 0.15.0 (#1246) ([#1246](https://github.com/csskit/csskit/pull/1246))


### Csskit_vscode
- chore(deps): update dependencies to v1.68.0 (#1247) ([#1247](https://github.com/csskit/csskit/pull/1247))
- fix(deps): update dependencies (patch) (#1255) ([#1255](https://github.com/csskit/csskit/pull/1255))

## [0.0.26] - 2026-06-20

### Other Changes
- Chore(deps): update dependencies to v1.48.0 (#1227) ([#1227](https://github.com/csskit/csskit/pull/1227))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#1221) ([#1221](https://github.com/csskit/csskit/pull/1221))
- fuzz(css_parse): Mark more erroneous tokens in blocks as trivia. (#1225) ([#1225](https://github.com/csskit/csskit/pull/1225))


### Css_lexer
- fuzz(css_lex): avoid buffer overflows with SmallStr buffer (#1220) ([#1220](https://github.com/csskit/csskit/pull/1220))


### Css_parse
- fuzz(css_parse): consume `Bad` tokens as trivia in blocks. (#1228) ([#1228](https://github.com/csskit/csskit/pull/1228))


### Csskit
- chore(deps): update dependencies (patch) (#1222) ([#1222](https://github.com/csskit/csskit/pull/1222))
- csskit_wasm/csskit(node): Provide csskit library functions in csskit package (#1234) ([#1234](https://github.com/csskit/csskit/pull/1234))
- csskit(node): manage wasm-opt with mise (#1235) ([#1235](https://github.com/csskit/csskit/pull/1235))
- csskit(node): Move build script to Mise (#1237) ([#1237](https://github.com/csskit/csskit/pull/1237))
- Release v0.0.26 (#1224) ([#1224](https://github.com/csskit/csskit/pull/1224))

## [0.0.25] - 2026-06-13

### Other Changes
- Update cargo deps (#1189) ([#1189](https://github.com/csskit/csskit/pull/1189))
- Push allocator-api2 back to 2.21 due to bumpalo (#1191) ([#1191](https://github.com/csskit/csskit/pull/1191))
- Fuzz: Try setting up a fuzzing suite (#1201) ([#1201](https://github.com/csskit/csskit/pull/1201))


### Css_ast
- css_ast: Avoid hanging on `={-->}` input. (#1183) ([#1183](https://github.com/csskit/csskit/pull/1183))
- Regenerate css_ast/src/values from csswg drafts (#1184) ([#1184](https://github.com/csskit/csskit/pull/1184))
- css_ast: Use assert_parse! closure macro variant in tests. (#1196) ([#1196](https://github.com/csskit/csskit/pull/1196))
- cssast/csskit_derives: Add new FeatureMetadata system (#1197) ([#1197](https://github.com/csskit/csskit/pull/1197))
- csskit_derives/css_ast: make derive(IntoCursor) work with generics (#1200) ([#1200](https://github.com/csskit/csskit/pull/1200))
- css_ast: Implement a NonEmpty generic (#1202) ([#1202](https://github.com/csskit/csskit/pull/1202))


### Css_lexer
- fuzz: early return atom functions when given non-atom tokens (#1206) ([#1206](https://github.com/csskit/csskit/pull/1206))
- fuzz(css_lexer): ensure REPLACEMENT_CHARACTER is appended in small idents (#1213) ([#1213](https://github.com/csskit/csskit/pull/1213))


### Css_parse
- css_parse: Ensure buffer is refilled if skipping past a whole buffer (#1180) ([#1180](https://github.com/csskit/csskit/pull/1180))
- css_parse: Impl ToSpan for Comparison (#1190) ([#1190](https://github.com/csskit/csskit/pull/1190))
- Impl Display for Comparison (#1193) ([#1193](https://github.com/csskit/csskit/pull/1193))
- css_parse: Improve assert_parse! macros to use a block/closure (#1194) ([#1194](https://github.com/csskit/csskit/pull/1194))


### Csskit
- csskit: improve json output for subcommands (#1182) ([#1182](https://github.com/csskit/csskit/pull/1182))
- chore(deps): update dependencies (patch) (#1185) ([#1185](https://github.com/csskit/csskit/pull/1185))
- Release v0.0.25 (#1181) ([#1181](https://github.com/csskit/csskit/pull/1181))


### Csskit_derives
- csskit_derives: Ensure generics expect Peek when using it. (#1198) ([#1198](https://github.com/csskit/csskit/pull/1198))


### Csskit_source_finder
- csskit_source_finder: Don't find visitable nodes that just visit children (#1199) ([#1199](https://github.com/csskit/csskit/pull/1199))


### Csskit_vscode
- chore(deps): update dependency @vscode/test-electron to v3 (#1192) ([#1192](https://github.com/csskit/csskit/pull/1192))
- fix(deps): update dependency esbuild to v0.28.1 [security] (#1210) ([#1210](https://github.com/csskit/csskit/pull/1210))

## [0.0.24] - 2026-06-04

### Other Changes
- Update css-minify-tests (#1150) ([#1150](https://github.com/csskit/csskit/pull/1150))
- Chore(deps): update rust crate dashmap to v6.2.1 (#1151) ([#1151](https://github.com/csskit/csskit/pull/1151))
- Chore(deps): update rust crate insta to v1.47.2 (#1152) ([#1152](https://github.com/csskit/csskit/pull/1152))
- Coverage: refactor csskit-acceptance script to allow arbitrary command syntax (#1167) ([#1167](https://github.com/csskit/csskit/pull/1167))
- Coverage: add more csskit-acceptance snapshot tests (#1168) ([#1168](https://github.com/csskit/csskit/pull/1168))
- Coverage: try and be consistent with trailing newlines (#1170) ([#1170](https://github.com/csskit/csskit/pull/1170))


### Chromashift
- chromashift/css_ast: Completely rewrite color mixing, support `none` channels & N-colors (#1162) ([#1162](https://github.com/csskit/csskit/pull/1162))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#1142) ([#1142](https://github.com/csskit/csskit/pull/1142))
- css_ast: resolve `none` channels to 0 (#1154) ([#1154](https://github.com/csskit/csskit/pull/1154))
- css_ast: Rewrite <position> and <bg-position> (#1164) ([#1164](https://github.com/csskit/csskit/pull/1164))
- css_ast: Implement transform-origin parsing (#1169) ([#1169](https://github.com/csskit/csskit/pull/1169))
- css_ast: Implement rect() function (#1171) ([#1171](https://github.com/csskit/csskit/pull/1171))
- css_ast: implement extra values for pointer-events (#1172) ([#1172](https://github.com/csskit/csskit/pull/1172))
- css_ast: implement -ms-word-break (#1173) ([#1173](https://github.com/csskit/csskit/pull/1173))
- css_ast: Implement -ms-box-sizing (#1174) ([#1174](https://github.com/csskit/csskit/pull/1174))
- css_ast: Implement vendor-prefixed -*-filter style values (#1175) ([#1175](https://github.com/csskit/csskit/pull/1175))
- css_ast: Implement color-scheme (#1176) ([#1176](https://github.com/csskit/csskit/pull/1176))
- css_ast: Implement -ms-interpolation-mode (#1177) ([#1177](https://github.com/csskit/csskit/pull/1177))
- css_ast: allow comma less rect functions (#1178) ([#1178](https://github.com/csskit/csskit/pull/1178))


### Csskit
- chore(deps): update dependencies (patch) (#1143) ([#1143](https://github.com/csskit/csskit/pull/1143))
- csskit: refactor unify CLI extraction commands with Extract trait + shared OutputFormat (#1179) ([#1179](https://github.com/csskit/csskit/pull/1179))
- Release v0.0.24 (#1139) ([#1139](https://github.com/csskit/csskit/pull/1139))


### Csskit_proc_macro
- csskit_proc_macro: Generate name suffixes for colliding properties (#1163) ([#1163](https://github.com/csskit/csskit/pull/1163))


### Csskit_transform
- coverage: update snapshot data (#1153) ([#1153](https://github.com/csskit/csskit/pull/1153))
- coverage: update css-minify-tests (#1161) ([#1161](https://github.com/csskit/csskit/pull/1161))


### Csskit_vscode
- Release csskit vscode extension v1.0.8 (#1107) ([#1107](https://github.com/csskit/csskit/pull/1107))
- Release csskit vscode extension v1.0.9 (#1147) ([#1147](https://github.com/csskit/csskit/pull/1147))
- chore(deps): update dependency oxlint to v1.65.0 (#1146) ([#1146](https://github.com/csskit/csskit/pull/1146))
- chore(deps): update dependency oxlint to v1.66.0 (#1148) ([#1148](https://github.com/csskit/csskit/pull/1148))
- chore(deps): update dependencies (patch) (#1156) ([#1156](https://github.com/csskit/csskit/pull/1156))
- chore(deps): update dependency oxlint to v1.67.0 (#1159) ([#1159](https://github.com/csskit/csskit/pull/1159))
- fix(deps): update dependency vscode-languageclient to v10 (#1165) ([#1165](https://github.com/csskit/csskit/pull/1165))

## [0.0.23] - 2026-05-22

### Other Changes
- Coverage: update css-minify-tests (#1133) ([#1133](https://github.com/csskit/csskit/pull/1133))


### Css_ast
- fix(css_parse): preserve whitespace around +/- in calc() (#1114) ([#1114](https://github.com/csskit/csskit/pull/1114))
- coverage: drop bootstrap/foundation .min files (#1117) ([#1117](https://github.com/csskit/csskit/pull/1117))
- css_ast: Implement relative color syntax (#1119) ([#1119](https://github.com/csskit/csskit/pull/1119))
- Regenerate css_ast/src/values from csswg drafts (#1120) ([#1120](https://github.com/csskit/csskit/pull/1120))
- csskit_spec_generator/css_ast: Generate properties from SVG, PointerEvents specs (#1121) ([#1121](https://github.com/csskit/csskit/pull/1121))
- css_ast: Implement ContentStyleValue (#1122) ([#1122](https://github.com/csskit/csskit/pull/1122))
- Regenerate css_ast/src/values from csswg drafts (#1123) ([#1123](https://github.com/csskit/csskit/pull/1123))
- css_ast: add pinch-zoom to touch-action (#1128) ([#1128](https://github.com/csskit/csskit/pull/1128))
- css_ast/csskit: Finish off ToSpecificity, add `csskit specificity` command (#1129) ([#1129](https://github.com/csskit/csskit/pull/1129))
- css_ast: Box all Gradient types (#1130) ([#1130](https://github.com/csskit/csskit/pull/1130))
- css_ast: Implement Background property (#1131) ([#1131](https://github.com/csskit/csskit/pull/1131))
- css_ast: Implement SpreadShadow, improve Shadow (#1132) ([#1132](https://github.com/csskit/csskit/pull/1132))
- css_ast: Implement text-overflow (#1134) ([#1134](https://github.com/csskit/csskit/pull/1134))
- css_ast: Implement more legacy/vendor display values (#1135) ([#1135](https://github.com/csskit/csskit/pull/1135))
- css_ast: Implement light-dark for color values (#1136) ([#1136](https://github.com/csskit/csskit/pull/1136))


### Css_lexer
- css_parse: refactor CursorCompactWriteSink, preserve comments between idents (#1118) ([#1118](https://github.com/csskit/csskit/pull/1118))


### Csskit
- chore(deps): update dependencies (patch) (#1124) ([#1124](https://github.com/csskit/csskit/pull/1124))
- Release v0.0.23 (#1116) ([#1116](https://github.com/csskit/csskit/pull/1116))


### Csskit_spec_generator
- csskit_spec_generator: Replace non-breaking spaces with whitespace (#1137) ([#1137](https://github.com/csskit/csskit/pull/1137))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.64.0 (#1127) ([#1127](https://github.com/csskit/csskit/pull/1127))
- chore(deps): update dependency @types/vscode to v1.120.0 (#1125) ([#1125](https://github.com/csskit/csskit/pull/1125))

## [0.0.22] - 2026-05-15

### Other Changes
- Chore(deps): update rust crate indexmap to v2.13.1 (#1108) ([#1108](https://github.com/csskit/csskit/pull/1108))
- Chore(deps): update rust crate indexmap to v2.14.0 (#1109) ([#1109](https://github.com/csskit/csskit/pull/1109))


### Chromashift
- chromashift/csskit_transform: Round fractional colour channels to perceptually safe values (#1092) ([#1092](https://github.com/csskit/csskit/pull/1092))


### Css_ast
- css_ast: implement corner-*, whites-pace, text-emphasis(-style), voice-volume (#1084) ([#1084](https://github.com/csskit/csskit/pull/1084))
- css_ast: Implement FontVariant* properties and sub-types (#1086) ([#1086](https://github.com/csskit/csskit/pull/1086))
- csskit_proc_macro: use Either<> type for 2-type Alternatives (#1089) ([#1089](https://github.com/csskit/csskit/pull/1089))
- css_ast: implement BorderColor, BorderImageSlice, BorderImageWidth (#1090) ([#1090](https://github.com/csskit/csskit/pull/1090))
- css_ast: Implement lots more properties (#1091) ([#1091](https://github.com/csskit/csskit/pull/1091))
- css_ast: Uncomment BorderShape, OffsetPath, ShapeInside, ShapeOutside (#1093) ([#1093](https://github.com/csskit/csskit/pull/1093))
- css_parse/css_ast: Use PEEK_KINDSET alot more consistently (#1094) ([#1094](https://github.com/csskit/csskit/pull/1094))
- css_ast: Implement a bunch of vendor prefixed style values (#1096) ([#1096](https://github.com/csskit/csskit/pull/1096))
- css_ast: Add vendor prefix display values (#1097) ([#1097](https://github.com/csskit/csskit/pull/1097))
- css_ast: Implement z-index (#1098) ([#1098](https://github.com/csskit/csskit/pull/1098))
- css_ast: Add vendor prefixed value keywords (#1101) ([#1101](https://github.com/csskit/csskit/pull/1101))
- css_ast: Uncomment clip/clip-path/mask/mask-image/mask-mode/mask-border-slice (#1110) ([#1110](https://github.com/csskit/csskit/pull/1110))
- css_ast: Unlock more properties by stubbing types (#1111) ([#1111](https://github.com/csskit/csskit/pull/1111))


### Css_feature_data
- Regenerate css_ast/src/values from csswg drafts (#1102) ([#1102](https://github.com/csskit/csskit/pull/1102))


### Css_lexer
- css_parse: Add Either<Left, Right> (#1088) ([#1088](https://github.com/csskit/csskit/pull/1088))
- css_lexer: Add Span.overlaps (#1095) ([#1095](https://github.com/csskit/csskit/pull/1095))


### Css_value_definition_parser
- css_value_definition_parser: Optimize Auto Number/Percentage or LengthPercentage/Number cases (#1087) ([#1087](https://github.com/csskit/csskit/pull/1087))
- css_value_definition_parser: Ensure dash prefixed idents are properly emitted (#1099) ([#1099](https://github.com/csskit/csskit/pull/1099))


### Csskit
- chore(deps): update dependencies (patch) (#1103) ([#1103](https://github.com/csskit/csskit/pull/1103))
- csskit: Improve messaging for check commands with missing sheets (#1113) ([#1113](https://github.com/csskit/csskit/pull/1113))
- Release v0.0.22 (#1080) ([#1080](https://github.com/csskit/csskit/pull/1080))


### Csskit-linux-x64
- packages: remove accidentally comitted bin (#1079) ([#1079](https://github.com/csskit/csskit/pull/1079))


### Csskit_proc_macro
- csskit_proc_macro: Auto generate name for Combinator of Alternatives (#1085) ([#1085](https://github.com/csskit/csskit/pull/1085))
- csskit_proc_macro: Ensure dash prefixed idents are properly emitted (#1100) ([#1100](https://github.com/csskit/csskit/pull/1100))


### Csskit_vscode
- vscode: bump version (#1082) ([#1082](https://github.com/csskit/csskit/pull/1082))
- Release csskit vscode extension v1.0.7 (#1083) ([#1083](https://github.com/csskit/csskit/pull/1083))
- chore(deps): update dependency oxlint to v1.63.0 (#1106) ([#1106](https://github.com/csskit/csskit/pull/1106))

## [0.0.21] - 2026-05-08

### Other Changes
- Chore(deps): update rust crate similar to v3 (#951) ([#951](https://github.com/csskit/csskit/pull/951))
- Chore(deps): update rust crate clap to v4.6.1 (#1052) ([#1052](https://github.com/csskit/csskit/pull/1052))
- Chore(deps): update rust crate bumpalo to v3.20.2 (#1051) ([#1051](https://github.com/csskit/csskit/pull/1051))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#958) ([#958](https://github.com/csskit/csskit/pull/958))
- Regenerate css_ast/src/values from csswg drafts (#974) ([#974](https://github.com/csskit/csskit/pull/974))
- csskit_ast: Impl derive(metadata) for more types including generics (#986) ([#986](https://github.com/csskit/csskit/pull/986))
- csskit_ast: Improve docs for a lot of types (#988) ([#988](https://github.com/csskit/csskit/pull/988))
- css_ast: Assert atoms <127 are dimensions, reorganise atoms to avoid hitting asserts (#989) ([#989](https://github.com/csskit/csskit/pull/989))
- csskit_ast: Box up ColorFunction, reducing many struct sizes (#992) ([#992](https://github.com/csskit/csskit/pull/992))
- Regenerate css_ast/src/values from csswg drafts (#995) ([#995](https://github.com/csskit/csskit/pull/995))
- Regenerate css_ast/src/values from csswg drafts (#1002) ([#1002](https://github.com/csskit/csskit/pull/1002))
- css_ast: implement list-style property (#1010) ([#1010](https://github.com/csskit/csskit/pull/1010))
- css_ast/css_value_definition_parser: Implement `<kw>?` style syntax (#1011) ([#1011](https://github.com/csskit/csskit/pull/1011))
- css_ast: Implement counter-increment, counter-set (#1012) ([#1012](https://github.com/csskit/csskit/pull/1012))
- css_ast: Implement filter/backdrop-filter (#1013) ([#1013](https://github.com/csskit/csskit/pull/1013))
- css_ast: Implement -webkit prefixed declarations (#1014) ([#1014](https://github.com/csskit/csskit/pull/1014))
- css_ast: Uncomment/implement justify-*/align-* properties (#1017) ([#1017](https://github.com/csskit/csskit/pull/1017))
- css_ast: Uncomment a bunch of properties (#1020) ([#1020](https://github.com/csskit/csskit/pull/1020))
- css_ast: Implement out a whole heap of todo properties (#1023) ([#1023](https://github.com/csskit/csskit/pull/1023))
- csskit_proc_macro: Geenrate Optionals subtypes for small groups (#1024) ([#1024](https://github.com/csskit/csskit/pull/1024))
- css_ast: Add all missing size_of tests for StyleValues (#1025) ([#1025](https://github.com/csskit/csskit/pull/1025))
- css_ast: Add a whole heap of parser tests for the various StyleValues (#1026) ([#1026](https://github.com/csskit/csskit/pull/1026))
- css_ast: Ensure CustomIdent doesn't accept css-wide keywords (#1027) ([#1027](https://github.com/csskit/csskit/pull/1027))
- css_ast: Correct syntax for TimelineRangeName (#1028) ([#1028](https://github.com/csskit/csskit/pull/1028))
- css_ast: add none as legacy compat value for ruby-overhang (#1029) ([#1029](https://github.com/csskit/csskit/pull/1029))
- csskit_derives: Parse all alternatives, hoisting common vars (#1037) ([#1037](https://github.com/csskit/csskit/pull/1037))
- css_ast: make OpacityValue unbounded at parse time (#1040) ([#1040](https://github.com/csskit/csskit/pull/1040))
- Regenerate css_ast/src/values from csswg drafts (#1030) ([#1030](https://github.com/csskit/csskit/pull/1030))
- css_ast: add negative border-spacing red test (#1039) ([#1039](https://github.com/csskit/csskit/pull/1039))
- css_ast: Add tests for FlexWrapStyleValue (#1041) ([#1041](https://github.com/csskit/csskit/pull/1041))
- Regenerate css_ast/src/values from csswg drafts (#1046) ([#1046](https://github.com/csskit/csskit/pull/1046))
- css_parse: Allow nested CommaSeparated items without failures (#1044) ([#1044](https://github.com/csskit/csskit/pull/1044))
- css_ast/css_transform: Remove unitless-zero time values (#1053) ([#1053](https://github.com/csskit/csskit/pull/1053))
- css_ast: Add some more tests for FlowIntoStyleValue (#1055) ([#1055](https://github.com/csskit/csskit/pull/1055))
- csskit_proc_macro: always lowercase idents (#1056) ([#1056](https://github.com/csskit/csskit/pull/1056))
- css_ast: Drop erroneous Atoms from bad filter-effects generation (#1057) ([#1057](https://github.com/csskit/csskit/pull/1057))
- css_ast: Uncomment a bunch of properties (#1063) ([#1063](https://github.com/csskit/csskit/pull/1063))
- csskit_derives: Fix one_must_occur discriminator to OR atoms (#1064) ([#1064](https://github.com/csskit/csskit/pull/1064))
- csskit_proc_macro: Wrap Zero-Or-More vecs in Option<> (#1071) ([#1071](https://github.com/csskit/csskit/pull/1071))
- csskit_ast/csskit_proc_macro/cssit_highlight: Force update all snapshots (#1072) ([#1072](https://github.com/csskit/csskit/pull/1072))


### Css_lexer
- css_lexer: Remove debug_assert! around atom bits (#993) ([#993](https://github.com/csskit/csskit/pull/993))
- css_lexer: Don't atomize for non-dimensions (#994) ([#994](https://github.com/csskit/csskit/pull/994))
- css_lexer: Add KindSet::remove (#1076) ([#1076](https://github.com/csskit/csskit/pull/1076))
- css_parse: Add assert_peek_false helper (#1075) ([#1075](https://github.com/csskit/csskit/pull/1075))


### Css_parse
- css_parse: Add peek_n_including_whitespace (#1077) ([#1077](https://github.com/csskit/csskit/pull/1077))


### Css_value_definition_parser
- css_value_definition_parser: Ensure maybe_unsized returns false for optimised types (#987) ([#987](https://github.com/csskit/csskit/pull/987))
- csskit_proc_macro/css_value_definition_parser: Optimize Alternatives by distributing types to discriminants (#1016) ([#1016](https://github.com/csskit/csskit/pull/1016))
- css_value_definition_parser: Improve optimisation of Alternatives via distributing optionals (#1018) ([#1018](https://github.com/csskit/csskit/pull/1018))
- css_value_definition_parser: Improve optimisations for Alternatives with optionals (#1019) ([#1019](https://github.com/csskit/csskit/pull/1019))
- css_value_definition_parser: Avoid distributing repeated multipliers (#1022) ([#1022](https://github.com/csskit/csskit/pull/1022))
- css_value_definition_parser: Pre-flatten nested Alternatives (#1031) ([#1031](https://github.com/csskit/csskit/pull/1031))


### Csskit
- chore(deps): update dependencies (patch) (#959) ([#959](https://github.com/csskit/csskit/pull/959))
- Release v0.0.21 (#952) ([#952](https://github.com/csskit/csskit/pull/952))


### Csskit_derives
- csskit_derives: Add support for named struct variants (#1035) ([#1035](https://github.com/csskit/csskit/pull/1035))
- csskit_derives: guard atom extraction in parse, when encountering non-atom values (#1036) ([#1036](https://github.com/csskit/csskit/pull/1036))
- csskit_derives: massively refactor library (#1038) ([#1038](https://github.com/csskit/csskit/pull/1038))
- csskit_derives: Support AllMustOccur/OneMustOccur in peek. (#1042) ([#1042](https://github.com/csskit/csskit/pull/1042))
- csskit_derives: Improve parsing for OneMustOccur fallthrough (#1043) ([#1043](https://github.com/csskit/csskit/pull/1043))
- csskit_derives: Handle AllMustOccur peeking in struct cases. (#1045) ([#1045](https://github.com/csskit/csskit/pull/1045))
- csskit_derives: Add new SharedPrefixPlan for parsing common variants with later discriminants (#1054) ([#1054](https://github.com/csskit/csskit/pull/1054))
- csskit_derives: Refactor all generation to use "Plans" concept (#1058) ([#1058](https://github.com/csskit/csskit/pull/1058))
- csskit_derives: Fix AllMustOccur occurance check to skip the optional fields (#1059) ([#1059](https://github.com/csskit/csskit/pull/1059))
- csskit_derives: Hoist shared type fields during parsing where possible (#1060) ([#1060](https://github.com/csskit/csskit/pull/1060))
- csskit_derives: Fix AllMustOccur all-optional non-atom dispatch (#1066) ([#1066](https://github.com/csskit/csskit/pull/1066))
- csskit_derives: Fix overlapping lead-atoms in sibling AllMustOccur variants (#1067) ([#1067](https://github.com/csskit/csskit/pull/1067))
- csskit_derives: Avoid getting atom from Idents only. (#1070) ([#1070](https://github.com/csskit/csskit/pull/1070))
- csskit_derives: Optimise Peek derive to use PEEK_KINDSET when possible (#1073) ([#1073](https://github.com/csskit/csskit/pull/1073))
- csskit_derives: Ensure peek checks enum variants after Option (#1074) ([#1074](https://github.com/csskit/csskit/pull/1074))


### Csskit_proc_macro
- csskit_proc_macro: Don't factor Puncts into variant names (#1021) ([#1021](https://github.com/csskit/csskit/pull/1021))
- csskit_proc_macro: Inline Options-with-keywords as enum struct variants (#1032) ([#1032](https://github.com/csskit/csskit/pull/1032))
- csskit_proc_macro: Generate more concise names for varaitns (#1033) ([#1033](https://github.com/csskit/csskit/pull/1033))
- csskit_proc_macro: drop erroneous pub from codegen (#1034) ([#1034](https://github.com/csskit/csskit/pull/1034))
- csskit_proc_macro: Allow names to be truncated even when unique values are equal (#1061) ([#1061](https://github.com/csskit/csskit/pull/1061))
- csskit_proc_macro: Flatten nested options into separate fields for enums (#1062) ([#1062](https://github.com/csskit/csskit/pull/1062))
- csskit_proc_macro: Try to improve type names even more (#1065) ([#1065](https://github.com/csskit/csskit/pull/1065))


### Csskit_vscode
- vscode/zed/nvim: Add docs & publish to various stores. Fixes #926 (#954) ([#926](https://github.com/csskit/csskit/pull/926))
- Release csskit vscode extension v1.0.1 (#955) ([#955](https://github.com/csskit/csskit/pull/955))
- chore(deps): update dependency oxlint to v1.55.0 (#967) ([#967](https://github.com/csskit/csskit/pull/967))
- chore(deps): update dependency oxlint to v1.57.0 (#969) ([#969](https://github.com/csskit/csskit/pull/969))
- chore(deps): update dependencies (patch) (#975) ([#975](https://github.com/csskit/csskit/pull/975))
- chore(deps): update dependency @types/vscode to v1.115.0 (#976) ([#976](https://github.com/csskit/csskit/pull/976))
- chore(deps): update dependency oxlint to v1.59.0 (#978) ([#978](https://github.com/csskit/csskit/pull/978))
- csskit_vscode: Update extension to auto-download the latest release (#982) ([#982](https://github.com/csskit/csskit/pull/982))
- Release csskit vscode extension v1.0.2 (#966) ([#966](https://github.com/csskit/csskit/pull/966))
- csskit_vscode: add logo to extension (#983) ([#983](https://github.com/csskit/csskit/pull/983))
- Release csskit vscode extension v1.0.3 (#984) ([#984](https://github.com/csskit/csskit/pull/984))
- chore(deps): update dependencies (patch) (#996) ([#996](https://github.com/csskit/csskit/pull/996))
- chore(deps): update dependency @types/vscode to v1.116.0 (#997) ([#997](https://github.com/csskit/csskit/pull/997))
- chore(deps): update dependency oxlint to v1.60.0 (#999) ([#999](https://github.com/csskit/csskit/pull/999))
- Release csskit vscode extension v1.0.4 (#1000) ([#1000](https://github.com/csskit/csskit/pull/1000))
- csskit_vscode: bump vscode engines version (#1001) ([#1001](https://github.com/csskit/csskit/pull/1001))
- chore(deps): update dependency oxlint to v1.61.0 (#1005) ([#1005](https://github.com/csskit/csskit/pull/1005))
- chore(deps): update dependency oxlint to v1.62.0 (#1049) ([#1049](https://github.com/csskit/csskit/pull/1049))
- chore(deps): update dependency @types/vscode to v1.118.0 (#1048) ([#1048](https://github.com/csskit/csskit/pull/1048))
- Release csskit vscode extension v1.0.5 (#1007) ([#1007](https://github.com/csskit/csskit/pull/1007))


### Csskit_wasm
- website: switch from commonjs to esm, drop wasm-pack (#957) ([#957](https://github.com/csskit/csskit/pull/957))


### Csskit_zed
- csskit_zed: add license field to extension.toml (#971) ([#971](https://github.com/csskit/csskit/pull/971))
- csskit_zed: Rewrite extension to download available binary for the latest version (#981) ([#981](https://github.com/csskit/csskit/pull/981))

## [0.0.20] - 2026-04-01

### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#943) ([#943](https://github.com/csskit/csskit/pull/943))


### Csskit
- chore(deps): update dependencies (patch) (#944) ([#944](https://github.com/csskit/csskit/pull/944))
- Release v0.0.20 (#948) ([#948](https://github.com/csskit/csskit/pull/948))


### Csskit_lsp
- csskit_lsp: Skip serializing None values for result/error in LSP messages. (#950) ([#950](https://github.com/csskit/csskit/pull/950))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.54.0 (#947) ([#947](https://github.com/csskit/csskit/pull/947))

## [0.0.19] - 2026-03-24

### Other Changes
- Csskit-acceptance: Never skip tests (#916) ([#916](https://github.com/csskit/csskit/pull/916))
- Chore(deps): update dependencies (patch) (#919) ([#919](https://github.com/csskit/csskit/pull/919))
- Chore(deps): update dependencies (patch) (#929) ([#929](https://github.com/csskit/csskit/pull/929))
- Chore(deps): update dependencies (patch) (#935) ([#935](https://github.com/csskit/csskit/pull/935))


### Chromashift
- chromashift: Add `in_gamut_of(ColorSpace)` to help guard minifying OOG. (#915) ([#915](https://github.com/csskit/csskit/pull/915))
- chromashift/csskit_transform/css_ast: Implement colour mixing & minification (#924) ([#924](https://github.com/csskit/csskit/pull/924))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#911) ([#911](https://github.com/csskit/csskit/pull/911))
- Regenerate css_ast/src/values from csswg drafts (#918) ([#918](https://github.com/csskit/csskit/pull/918))


### Csskit
- chore(deps): update dependencies (patch) (#912) ([#912](https://github.com/csskit/csskit/pull/912))
- Release v0.0.19 (#914) ([#914](https://github.com/csskit/csskit/pull/914))


### Csskit_transform
- Update css-minify-tests (#923) ([#923](https://github.com/csskit/csskit/pull/923))


### Csskit_vscode
- chore(deps): update dependency @types/node to v24.11.0 (#913) ([#913](https://github.com/csskit/csskit/pull/913))
- chore(deps): update dependency @types/node to v24.12.0 (#921) ([#921](https://github.com/csskit/csskit/pull/921))
- chore(deps): update dependencies to v1.50.0 (minor) (#920) ([#920](https://github.com/csskit/csskit/pull/920))
- chore(deps): update dependency @types/vscode to v1.110.0 (#930) ([#930](https://github.com/csskit/csskit/pull/930))
- chore(deps): update dependency oxlint to v1.51.0 (#938) ([#938](https://github.com/csskit/csskit/pull/938))
- chore(deps): update dependency typescript to v6 (#940) ([#940](https://github.com/csskit/csskit/pull/940))

## [0.0.18] - 2026-03-01

### Other Changes
- Chore(deps): update rust crate toml to v1 (#875) ([#875](https://github.com/csskit/csskit/pull/875))
- Update css-tokenizer-tests (#887) ([#887](https://github.com/csskit/csskit/pull/887))
- Chore(deps): update dependencies (patch) (#900) ([#900](https://github.com/csskit/csskit/pull/900))


### Chromashift
- chromashift: Update to support wide-gamut color spaces, out of gamut colors (#877) ([#877](https://github.com/csskit/csskit/pull/877))
- chromashift: Implement css-color-4 raytrace gamut mapping (#879) ([#879](https://github.com/csskit/csskit/pull/879))
- chromashift: Increase precision of display-p3 to sRGB conversion (#897) ([#897](https://github.com/csskit/csskit/pull/897))


### Css_ast
- css_ast: Implement gap rule list (#878) ([#878](https://github.com/csskit/csskit/pull/878))
- Regenerate css_ast/src/values from csswg drafts (#881) ([#881](https://github.com/csskit/csskit/pull/881))
- css_ast: Implement ColorMix (#889) ([#889](https://github.com/csskit/csskit/pull/889))
- css_ast: Boxup more types (#890) ([#890](https://github.com/csskit/csskit/pull/890))
- Implement ColorFunctionColor::to_chromashift() and extract shortest_color helper (#898) ([#898](https://github.com/csskit/csskit/pull/898))
- Regenerate css_ast/src/values from csswg drafts (#899) ([#899](https://github.com/csskit/csskit/pull/899))
- css_ast: Remove StringOrUrl type (#909) ([#909](https://github.com/csskit/csskit/pull/909))


### Css_lexer
- css_lexer: Introduce UnicodeRange token kind. (#891) ([#891](https://github.com/csskit/csskit/pull/891))
- css_lexer: Add "Bad" tokens. (#892) ([#892](https://github.com/csskit/csskit/pull/892))
- css_lexer: Re-encode escapes in strings if they're invalid for unescaping (#894) ([#894](https://github.com/csskit/csskit/pull/894))
- css_lexer: Re-encode escapes in idents if they're invalid for unescaping (#896) ([#896](https://github.com/csskit/csskit/pull/896))
- css_lexer: Rewrite to use Bytes, memchr (#907) ([#907](https://github.com/csskit/csskit/pull/907))


### Css_parse
- css_parse: Add BumpBox type and blanket trait impls for Box types (#888) ([#888](https://github.com/csskit/csskit/pull/888))
- css_parse: flush trailing semicolons in CursorCompactWriteSink on drop (#908) ([#908](https://github.com/csskit/csskit/pull/908))


### Csskit
- chore(deps): update dependencies (patch) (#882) ([#882](https://github.com/csskit/csskit/pull/882))
- Release v0.0.18 (#876) ([#876](https://github.com/csskit/csskit/pull/876))


### Csskit_transform
- csskit_transform: introduce css-minify-tests (#886) ([#886](https://github.com/csskit/csskit/pull/886))
- csskit_transform: snapshot css-minify-tests failures instead of asserting count (#893) ([#893](https://github.com/csskit/csskit/pull/893))
- csskit_transform: reduce Urls to Strings where possible (#910) ([#910](https://github.com/csskit/csskit/pull/910))


### Csskit_vscode
- chore(deps): update dependencies (minor) (#884) ([#884](https://github.com/csskit/csskit/pull/884))
- chore(deps): update dependencies to v1.49.0 (minor) (#901) ([#901](https://github.com/csskit/csskit/pull/901))

## [0.0.17] - 2026-02-11

### Css_ast
- css_ast: Uncomment FlexStyleValue (#860) ([#860](https://github.com/csskit/csskit/pull/860))
- Regenerate css_ast/src/values from csswg drafts (#851) ([#851](https://github.com/csskit/csskit/pull/851))
- css_ast: Add NormalOr<> generic. (#861) ([#861](https://github.com/csskit/csskit/pull/861))
- css_ast: Enable FontFeatureSettingsStyleValue, FontVariationSettingsStyleValue (#862) ([#862](https://github.com/csskit/csskit/pull/862))
- css_ast: Fixup container queries (#874) ([#874](https://github.com/csskit/csskit/pull/874))
- css_ast: Support custom scroll state queries (#873) ([#873](https://github.com/csskit/csskit/pull/873))
- Regenerate css_ast/src/values from csswg drafts (#863) ([#863](https://github.com/csskit/csskit/pull/863))


### Csskit
- chore(deps): update dependencies (patch) (#852) ([#852](https://github.com/csskit/csskit/pull/852))
- Release v0.0.17 (#858) ([#858](https://github.com/csskit/csskit/pull/858))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.42.0 (#855) ([#855](https://github.com/csskit/csskit/pull/855))
- chore(deps): update dependencies (patch) (#864) ([#864](https://github.com/csskit/csskit/pull/864))
- chore(deps): update dependency oxlint to v1.43.0 (#867) ([#867](https://github.com/csskit/csskit/pull/867))
- chore(deps): update dependency @types/vscode to v1.109.0 (#865) ([#865](https://github.com/csskit/csskit/pull/865))

## [0.0.16] - 2026-01-29

### Css_lexer
- css_lexer: Compact String & URL tokens (#843) ([#843](https://github.com/csskit/csskit/pull/843))
- csskit/css_lexer/css_parse: Add super secret hidden `expand` command (#846) ([#846](https://github.com/csskit/csskit/pull/846))


### Csskit
- Release v0.0.16 (#844) ([#844](https://github.com/csskit/csskit/pull/844))


### Csskit_wasm
- website/csskit-wasm: Support the format feature in the playground (#841) ([#841](https://github.com/csskit/csskit/pull/841))

## [0.0.15] - 2026-01-28

### Css_ast
- css_ast/csskit_derives: Add derive(NodeWithMetadata) (#824) ([#824](https://github.com/csskit/csskit/pull/824))
- css_ast/csskit_derive/csskit_proc_macro: Add "constrainable" generics. (#825) ([#825](https://github.com/csskit/csskit/pull/825))
- css_ast/css_lexer: Preserve sign in an+b, otherwise compact (#828) ([#828](https://github.com/csskit/csskit/pull/828))
- css_ast: Don't preserve sign for integer An+B selectors. (#829) ([#829](https://github.com/csskit/csskit/pull/829))
- Regenerate css_ast/src/values from csswg drafts (#830) ([#830](https://github.com/csskit/csskit/pull/830))


### Css_lexer
- csskit_transform/css_lexer: Check compacted lengths to determine if replacement is shorter. (#827) ([#827](https://github.com/csskit/csskit/pull/827))
- css_lexer: De-duplicate escape sequence consumption (#835) ([#835](https://github.com/csskit/csskit/pull/835))


### Csskit
- chore(deps): update dependencies (patch) (#831) ([#831](https://github.com/csskit/csskit/pull/831))
- Release v0.0.15 (#826) ([#826](https://github.com/csskit/csskit/pull/826))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.40.0 (#834) ([#834](https://github.com/csskit/csskit/pull/834))

## [0.0.14] - 2026-01-22

### Other Changes
- Coverage: Add `:unknown` coverage testing to all popular CSS libraries (#796) ([#796](https://github.com/csskit/csskit/pull/796))


### Chromashift
- chromashift: Fix lossy alpha conversions (#784) ([#784](https://github.com/csskit/csskit/pull/784))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#789) ([#789](https://github.com/csskit/csskit/pull/789))
- css_ast: Fixup MediaQueryList & ContainerConditionList to use CommaSeparated (#795) ([#795](https://github.com/csskit/csskit/pull/795))
- css_ast: Add support for parsing `@import` rules (#797) ([#797](https://github.com/csskit/csskit/pull/797))
- css_ast: enables `<display>` (#799) ([#799](https://github.com/csskit/csskit/pull/799))
- css_ast/csskit_proc_macro: Provide system for easily renaming auto-generated types (#800) ([#800](https://github.com/csskit/csskit/pull/800))
- Regenerate css_ast/src/values from csswg drafts (#801) ([#801](https://github.com/csskit/csskit/pull/801))
- Regenerate css_ast/src/values from csswg drafts (#808) ([#808](https://github.com/csskit/csskit/pull/808))
- csskit_transform: Reduce more color types (#818) ([#818](https://github.com/csskit/csskit/pull/818))
- csskit_transform: Add ReduceTimeUnits transform (#819) ([#819](https://github.com/csskit/csskit/pull/819))
- css_ast/css_transform: Only reduce lengths to unitless zero when applicable (#821) ([#821](https://github.com/csskit/csskit/pull/821))
- csskit_spec_generator/css_ast: Generate many more values (#823) ([#823](https://github.com/csskit/csskit/pull/823))


### Css_lexer
- css_lexer/css_parse: Add SourceCursor compacting, ensure CursorCompactWriteSink compacts cursors before write (#782) ([#782](https://github.com/csskit/csskit/pull/782))


### Css_parse
- csskit/csskit_transform/css_parse: Improve transform architecture. Minify lengths (#815) ([#815](https://github.com/csskit/csskit/pull/815))


### Csskit
- chore(deps): update dependencies (patch) (#790) ([#790](https://github.com/csskit/csskit/pull/790))
- Release v0.0.14 (#783) ([#783](https://github.com/csskit/csskit/pull/783))


### Csskit_ast
- csskit_ast: Add benchmarks for linting with example cks files (#788) ([#788](https://github.com/csskit/csskit/pull/788))


### Csskit_transform
- csskit_transform: Add Reduce colors (#816) ([#816](https://github.com/csskit/csskit/pull/816))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.36.0 (#791) ([#791](https://github.com/csskit/csskit/pull/791))
- chore(deps): update dependency @types/vscode to v1.108.1 (#803) ([#803](https://github.com/csskit/csskit/pull/803))
- chore(deps): update dependency oxlint to v1.38.0 (#805) ([#805](https://github.com/csskit/csskit/pull/805))
- chore(deps): update dependencies (patch) (#802) ([#802](https://github.com/csskit/csskit/pull/802))
- fix(deps): update dependencies (patch) (#809) ([#809](https://github.com/csskit/csskit/pull/809))
- chore(deps): update dependency oxlint to v1.39.0 (#811) ([#811](https://github.com/csskit/csskit/pull/811))
- chore(deps): update dependency prettier to v3.8.0 (#813) ([#813](https://github.com/csskit/csskit/pull/813))


### Csskit_wasm
- website: fix pages deploy (#807) ([#807](https://github.com/csskit/csskit/pull/807))
- csskit_wasm: Use minifier transform architecture (#817) ([#817](https://github.com/csskit/csskit/pull/817))

## [0.0.13] - 2026-01-03

### Csskit
- csskit_ast: Fix up feature flag issues (#780) ([#780](https://github.com/csskit/csskit/pull/780))
- Release v0.0.13 (#781) ([#781](https://github.com/csskit/csskit/pull/781))

## [0.0.12] - 2026-01-02

### Csskit
- csskit: Resolve platform binary using a directory walk, fixing `npx`. (#771) ([#771](https://github.com/csskit/csskit/pull/771))
- csskit: Tidy up bash script (#772) ([#772](https://github.com/csskit/csskit/pull/772))
- Release v0.0.12 (#773) ([#773](https://github.com/csskit/csskit/pull/773))

## [0.0.11] - 2025-12-30

### Chromashift
- csskit_highlight/csskit/chromashift: Support owo-colors (#766) ([#766](https://github.com/csskit/csskit/pull/766))


### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#760) ([#760](https://github.com/csskit/csskit/pull/760))


### Csskit
- fix(deps): update dependencies (patch) (#761) ([#761](https://github.com/csskit/csskit/pull/761))
- csskit_ast/csskit: Implement Rule sheets, `csskit check` command (#769) ([#769](https://github.com/csskit/csskit/pull/769))
- Release v0.0.11 (#763) ([#763](https://github.com/csskit/csskit/pull/763))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.35.0 (#762) ([#762](https://github.com/csskit/csskit/pull/762))

## [0.0.10] - 2025-12-28

### Css_ast
- csskit_ast: Ensure build script is isolated (#758) ([#758](https://github.com/csskit/csskit/pull/758))


### Csskit
- Release v0.0.10 (#759) ([#759](https://github.com/csskit/csskit/pull/759))

## [0.0.9] - 2025-12-28

### Css_ast
- css_ast: Re-work CssAtomSet to allow for vendor prefix bitflags (#716) ([#716](https://github.com/csskit/csskit/pull/716))
- css_ast: Add NodeKinds to CssMetadata (#719) ([#719](https://github.com/csskit/csskit/pull/719))
- css_parse/css_ast: Improve NodeWithMetadata to allow extracting self meta (#720) ([#720](https://github.com/csskit/csskit/pull/720))
- css_ast: Set DeclarationKind Shorthand/Longhand for known properties (#721) ([#721](https://github.com/csskit/csskit/pull/721))
- css_ast: Queryable now requires NodeWithMetadata<CssMetada>, derives default (#722) ([#722](https://github.com/csskit/csskit/pull/722))
- css_parse: Consume whitespace as trivia around non-descendant combinators (#723) ([#723](https://github.com/csskit/csskit/pull/723))
- csskit_ast: Rework matcher, using Metadata to propagate up SelectorRequirements (#725) ([#725](https://github.com/csskit/csskit/pull/725))
- css_ast/csskit_ast: Implement efficient attribute selectors via PropertyKind (#726) ([#726](https://github.com/csskit/csskit/pull/726))
- Regenerate css_ast/src/values from csswg drafts (#727) ([#727](https://github.com/csskit/csskit/pull/727))
- css_ast: Parse error on nested :has() (#739) ([#739](https://github.com/csskit/csskit/pull/739))
- css_ast: Rework CssMetadata to remove RuleKind (#740) ([#740](https://github.com/csskit/csskit/pull/740))
- css_ast: Ensure visit_declaration provides a QueryableNode (#741) ([#741](https://github.com/csskit/csskit/pull/741))
- css_ast: Implement @counter-style, @namespace (#742) ([#742](https://github.com/csskit/csskit/pull/742))
- css_ast/csskit_highlight/csskit_ast: Simplify code with visit_queryable_node (#743) ([#743](https://github.com/csskit/csskit/pull/743))
- csskit_ast: Significantly rework the selector engine. (#745) ([#745](https://github.com/csskit/csskit/pull/745))
- csskit_ast: Implement :size() pseudo (#748) ([#748](https://github.com/csskit/csskit/pull/748))
- css_lexer: Add DynAtomRegistry (#751) ([#751](https://github.com/csskit/csskit/pull/751))
- css_ast/csskit_spec_generator: Expand longhands in generator, not at runtime (#752) ([#752](https://github.com/csskit/csskit/pull/752))
- css_ast: fix css_ast featue gated imports (#757) ([#757](https://github.com/csskit/csskit/pull/757))


### Css_parse
- csskit_ast: Optimise QuerySelectorComponent matching with single-pass… (#733) ([#733](https://github.com/csskit/csskit/pull/733))


### Csskit
- csskit_ast: Rewrite QuerySelector to use css_parse (#724) ([#724](https://github.com/csskit/csskit/pull/724))
- chore(deps): update dependencies (patch) (#728) ([#728](https://github.com/csskit/csskit/pull/728))
- csskit: Add a `tree` command which shows selector tree (#750) ([#750](https://github.com/csskit/csskit/pull/750))
- csskit: improve `find` command (#756) ([#756](https://github.com/csskit/csskit/pull/756))
- Release v0.0.9 (#717) ([#717](https://github.com/csskit/csskit/pull/717))


### Csskit_ast
- csskit_ast: Remove :block psuedo class (#718) ([#718](https://github.com/csskit/csskit/pull/718))
- csskit_ast: Split matcher.rs into several files (#732) ([#732](https://github.com/csskit/csskit/pull/732))
- csskit_ast: Simplify PropertyGroup and VendorPrefixes lookups during parse/meta creation (#734) ([#734](https://github.com/csskit/csskit/pull/734))
- csskit_ast: Unify pseudo-class matching for nodes and declarations. (#735) ([#735](https://github.com/csskit/csskit/pull/735))
- csskit_ast: Improve :not & sibling selector matching (#736) ([#736](https://github.com/csskit/csskit/pull/736))
- csskit_ast: Optimise selector & attribute matching (#737) ([#737](https://github.com/csskit/csskit/pull/737))
- csskit_ast: Further improve :not() matching. (#738) ([#738](https://github.com/csskit/csskit/pull/738))
- csskit_ast: Implement `:has()` (#746) ([#746](https://github.com/csskit/csskit/pull/746))
- csskit_ast: Move CsskitAtomSet to own file (#749) ([#749](https://github.com/csskit/csskit/pull/749))
- csskit_ast: Ensure invalid selectors never match (#753) ([#753](https://github.com/csskit/csskit/pull/753))
- csskit_ast: build.rs to generate NodeId atoms in csskit_atom_set (#754) ([#754](https://github.com/csskit/csskit/pull/754))
- csskit_ast: Tidy up types, adding derives, removing dead code (#755) ([#755](https://github.com/csskit/csskit/pull/755))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.34.0 (#730) ([#730](https://github.com/csskit/csskit/pull/730))

## [0.0.8] - 2025-12-20

### Css_ast
- Regenerate css_ast/src/values from csswg drafts (#686) ([#686](https://github.com/csskit/csskit/pull/686))
- css_ast: Implement @starting-style (#693) ([#693](https://github.com/csskit/csskit/pull/693))
- css_parse: Ensure Parser restores Skip/Stop/State flags (#698) ([#698](https://github.com/csskit/csskit/pull/698))
- css_parse: Ensure trailing/leading descendant combinators aren't parsed (#699) ([#699](https://github.com/csskit/csskit/pull/699))
- css_ast/css_parse: Properly handle parse failures for unknown Rules vs Declarations (#700) ([#700](https://github.com/csskit/csskit/pull/700))
- Regenerate css_ast/src/values from csswg drafts (#701) ([#701](https://github.com/csskit/csskit/pull/701))
- css_ast: Implement exit_* methods alongside visit_* methods (#706) ([#706](https://github.com/csskit/csskit/pull/706))
- csskit_source_finder: Capture more node types (e.g. manual Visitable impls) (#707) ([#707](https://github.com/csskit/csskit/pull/707))
- css_ast: add exit_* calls to container nodes (#708) ([#708](https://github.com/csskit/csskit/pull/708))
- css_ast: derive(ToSpan) on more nodes (#709) ([#709](https://github.com/csskit/csskit/pull/709))
- css_ast: Implement Visitable for media query features (#710) ([#710](https://github.com/csskit/csskit/pull/710))
- css_ast: Make Supports & Container features visitable (#711) ([#711](https://github.com/csskit/csskit/pull/711))
- css_ast: Add QueryableNode trait for NodeIds (#712) ([#712](https://github.com/csskit/csskit/pull/712))
- css_ast: Make container/supports nodes queryable (#713) ([#713](https://github.com/csskit/csskit/pull/713))
- csskit: add `csskit find`, csskit_ast  (#715) ([#715](https://github.com/csskit/csskit/pull/715))


### Css_parse
- css_parse: elide more whitespace in CursorCompactWriteSink (#694) ([#694](https://github.com/csskit/csskit/pull/694))


### Csskit
- Release v0.0.8 (#689) ([#689](https://github.com/csskit/csskit/pull/689))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.31.0 (#688) ([#688](https://github.com/csskit/csskit/pull/688))
- chore(deps): update dependency prettier to v3.7.4 (#691) ([#691](https://github.com/csskit/csskit/pull/691))
- chore(deps): update dependency oxlint to v1.32.0 (#704) ([#704](https://github.com/csskit/csskit/pull/704))
- chore(deps): update dependency @types/vscode to v1.107.0 (#703) ([#703](https://github.com/csskit/csskit/pull/703))
- chore(deps): update dependencies (patch) (#702) ([#702](https://github.com/csskit/csskit/pull/702))


### Csskit_wasm
- wasm: use cursor compact write sink for minification (#692) ([#692](https://github.com/csskit/csskit/pull/692))

## [0.0.7] - 2025-12-06

### Other Changes
- Generate-values: clean up old generate values code (#620) ([#620](https://github.com/csskit/csskit/pull/620))
- Chore(deps): update dependencies (patch) (#630) ([#630](https://github.com/csskit/csskit/pull/630))
- Chore(deps): update rust crate syn to v2.0.110 (#659) ([#659](https://github.com/csskit/csskit/pull/659))
- Chore(deps): update dependencies (patch) (#677) ([#677](https://github.com/csskit/csskit/pull/677))


### Css_ast
- css_ast: Add more atoms, <EventTriggerEvent>, <KeypressFunction> (#622) ([#622](https://github.com/csskit/csskit/pull/622))
- Regenerate css_ast/src/values from csswg drafts (#624) ([#624](https://github.com/csskit/csskit/pull/624))
- Regenerate css_ast/src/values from csswg drafts (#629) ([#629](https://github.com/csskit/csskit/pull/629))
- css_ast/csskit_proc_macro: Support `,` literals better. (#638) ([#638](https://github.com/csskit/csskit/pull/638))
- Introduce CssMetadata (#639) ([#639](https://github.com/csskit/csskit/pull/639))
- Regenerate css_ast/src/values from csswg drafts (#640) ([#640](https://github.com/csskit/csskit/pull/640))
- Implement IntoCursors/Copy for more single Cursor enums (#641) ([#641](https://github.com/csskit/csskit/pull/641))
- Implement SemanticEq type for all nodes (#642) ([#642](https://github.com/csskit/csskit/pull/642))
- Migrate codegen hashmaps to toml files (#643) ([#643](https://github.com/csskit/csskit/pull/643))
- Add longhand/shorthands enumeration into Codegen (#644) ([#644](https://github.com/csskit/csskit/pull/644))
- css_ast: Add `num_sides` method to BoxSides. (#645) ([#645](https://github.com/csskit/csskit/pull/645))
- css_ast: Provide methods for getting StyleValue metadata by declaration ID. (#646) ([#646](https://github.com/csskit/csskit/pull/646))
- Ensure longhands/shorthand_group values are more accurate and stable. (#648) ([#648](https://github.com/csskit/csskit/pull/648))
- Regenerate css_ast/src/values from csswg drafts (#656) ([#656](https://github.com/csskit/csskit/pull/656))
- Regenerate css_ast/src/values from csswg drafts (#663) ([#663](https://github.com/csskit/csskit/pull/663))
- Regenerate css_ast/src/values from csswg drafts (#680) ([#680](https://github.com/csskit/csskit/pull/680))


### Css_feature_data
- Regenerate css_ast/src/values from csswg drafts (#649) ([#649](https://github.com/csskit/csskit/pull/649))


### Css_parse
- css_parse: impl Peek for Optionals (#626) ([#626](https://github.com/csskit/csskit/pull/626))


### Csskit
- npm: release architecture dependant packages. (#623) ([#623](https://github.com/csskit/csskit/pull/623))
- csskit: ensure npm script is robust to symlinks (#637) ([#637](https://github.com/csskit/csskit/pull/637))
- Release v0.0.7 (#621) ([#621](https://github.com/csskit/csskit/pull/621))


### Csskit_spec_generator
- csskit_spec_generator: Fork generate-values/mod.ts into Rust package (#619) ([#619](https://github.com/csskit/csskit/pull/619))
- csskit_spec_generator: reinstate opt-outs for Parse impls (#627) ([#627](https://github.com/csskit/csskit/pull/627))
- csskit_spec_generator: generate feature data in generate-all (#628) ([#628](https://github.com/csskit/csskit/pull/628))
- csskit_spec_generator: disable benchmarks (#635) ([#635](https://github.com/csskit/csskit/pull/635))
- csskit_spec_generator: Generate more descriptive PR bodies for weekly PR (#679) ([#679](https://github.com/csskit/csskit/pull/679))


### Csskit_vscode
- chore(deps): update dependency oxlint to v1.24.0 (#632) ([#632](https://github.com/csskit/csskit/pull/632))
- chore(deps): update dependency @types/node to v24.9.1 (#631) ([#631](https://github.com/csskit/csskit/pull/631))
- chore(deps): update dependencies (patch) (#650) ([#650](https://github.com/csskit/csskit/pull/650))
- chore(deps): update dependency oxlint to v1.25.0 (#654) ([#654](https://github.com/csskit/csskit/pull/654))
- chore(deps): update dependency @types/node to v24.10.0 (#652) ([#652](https://github.com/csskit/csskit/pull/652))
- chore(deps): update dependencies (patch) (#657) ([#657](https://github.com/csskit/csskit/pull/657))
- chore(deps): update dependency oxlint to v1.26.0 (#658) ([#658](https://github.com/csskit/csskit/pull/658))
- fix(deps): update dependencies (patch) (#664) ([#664](https://github.com/csskit/csskit/pull/664))
- chore(deps): update dependency oxlint to v1.27.0 (#667) ([#667](https://github.com/csskit/csskit/pull/667))
- chore(deps): update dependency @types/vscode to v1.106.0 (#665) ([#665](https://github.com/csskit/csskit/pull/665))
- chore(deps): update dependency oxlint to v1.29.0 (#676) ([#676](https://github.com/csskit/csskit/pull/676))
- fix(deps): update dependencies (patch) (#673) ([#673](https://github.com/csskit/csskit/pull/673))
- chore(deps): update dependency oxlint to v1.30.0 (#678) ([#678](https://github.com/csskit/csskit/pull/678))

## [0.0.6] - 2025-10-26

### Other Changes
- Introduce new submodules for test coverage (#602) ([#602](https://github.com/csskit/csskit/pull/602))


### Chromashift
- Add a bunch of readmes and Cargo.toml descriptions for all released crates. (#613) ([#613](https://github.com/csskit/csskit/pull/613))


### Css_ast
- benchmarks: fix benchmark code (#607) ([#607](https://github.com/csskit/csskit/pull/607))


### Css_value_definition_parser
- csskit_proc_macro: move requires_allocator_lifetime & generated_data_type css_value_definition_parser (#609) ([#609](https://github.com/csskit/csskit/pull/609))
- css_value_definition_parser: support quoted literal puncts, fixed groups (#611) ([#611](https://github.com/csskit/csskit/pull/611))


### Csskit
- release: tweak canary workflow (#610) ([#610](https://github.com/csskit/csskit/pull/610))
- Release v0.0.6 (#618) ([#618](https://github.com/csskit/csskit/pull/618))

## [0.0.5] - 2025-10-25

### Other Changes
- Update cargo deps (#493) ([#493](https://github.com/csskit/csskit/pull/493))
- Remove rust-toolchain.toml (#491) ([#491](https://github.com/csskit/csskit/pull/491))
- Refine & simplify readme (#558) ([#558](https://github.com/csskit/csskit/pull/558))
- Version 0.0.4 (#565) ([#565](https://github.com/csskit/csskit/pull/565))
- Csskit-acceptance: Get acceptance tests working again. (#566) ([#566](https://github.com/csskit/csskit/pull/566))
- Fix link to csskit.rs in README. (#582) ([#582](https://github.com/csskit/csskit/pull/582))
- Fixup cargo lock & toml (#592) ([#592](https://github.com/csskit/csskit/pull/592))
- Inject package versions on each release (#594) ([#594](https://github.com/csskit/csskit/pull/594))
- Fixup release-automation (#598) ([#598](https://github.com/csskit/csskit/pull/598))
- Fixup release automation (#599) ([#599](https://github.com/csskit/csskit/pull/599))
- Set initial tag in cliff.toml (#600) ([#600](https://github.com/csskit/csskit/pull/600))


### Chromashift
- fix failing actions (#508) ([#508](https://github.com/csskit/csskit/pull/508))
- chromashift: Add ability to extract alpha, TryInto named color (#567) ([#567](https://github.com/csskit/csskit/pull/567))
- chromashift: dont clamp xyz values (#584) ([#584](https://github.com/csskit/csskit/pull/584))
- introduce package level changelogs (#593) ([#593](https://github.com/csskit/csskit/pull/593))
- build a system for canary+release PRs (#595) ([#595](https://github.com/csskit/csskit/pull/595))


### Css_ast
- drop Build trait (#455) ([#455](https://github.com/csskit/csskit/pull/455))
- Add #[in_range] as a generic attribute for both derive(Parse) and derive(Peek). (#456) ([#456](https://github.com/csskit/csskit/pull/456))
- css_ast: derive(Parse) on more types (#457) ([#457](https://github.com/csskit/csskit/pull/457))
- Replace miette diagnostics with custom Sized struct. (#470) ([#470](https://github.com/csskit/csskit/pull/470))
- css_ast/css_parse/css_lexer: Add allocation tests (#472) ([#472](https://github.com/csskit/csskit/pull/472))
- switch serde json snapshots to ron snapshots (#473) ([#473](https://github.com/csskit/csskit/pull/473))
- drop tag/content strings from serde output (#474) ([#474](https://github.com/csskit/csskit/pull/474))
- css_ast: tidy up mod/impl generation (#475) ([#475](https://github.com/csskit/csskit/pull/475))
- css_ast: Use prelude pattern in types/functions (#476) ([#476](https://github.com/csskit/csskit/pull/476))
- drop derives & ToCursors/ToSpan from _feature! macros (#477) ([#477](https://github.com/csskit/csskit/pull/477))
- css_ast: add prelude for rules/mod.rs (#478) ([#478](https://github.com/csskit/csskit/pull/478))
- css_ast: add prelude for units/*.rs (#479) ([#479](https://github.com/csskit/csskit/pull/479))
- css_ast: remove content/kebab-casing from more serde types (#480) ([#480](https://github.com/csskit/csskit/pull/480))
- Re-introduce string interning across codebase
- css_ast/css_lexer: iterate over snapshots directory to produce snapshot data (#483) ([#483](https://github.com/csskit/csskit/pull/483))
- update Rust 1.90.0 (#484) ([#484](https://github.com/csskit/csskit/pull/484))
- coverage: add more css files to test (#485) ([#485](https://github.com/csskit/csskit/pull/485))
- csskit_derives: Add support for deriving on generics (#496) ([#496](https://github.com/csskit/csskit/pull/496))
- css_ast: derive(Parse) on more types (#497) ([#497](https://github.com/csskit/csskit/pull/497))
- css_ast: Clean up a few nodes by using derive(Parse/Peek) (#498) ([#498](https://github.com/csskit/csskit/pull/498))
- css_parse: add trivia output (#499) ([#499](https://github.com/csskit/csskit/pull/499))
- css_parse: implement ordered cursor sink output (#500) ([#500](https://github.com/csskit/csskit/pull/500))
- css_parse: Avoid having to manually flush Cursor Sinks by emitting Eof. (#501) ([#501](https://github.com/csskit/csskit/pull/501))
- CI: test in nightly, make more build modes required (#503) ([#503](https://github.com/csskit/csskit/pull/503))
- CI: try to get release builds working again (#504) ([#504](https://github.com/csskit/csskit/pull/504))
- Regenerate css_ast/src/values from csswg drafts (#517) ([#517](https://github.com/csskit/csskit/pull/517))
- remove the redundant prefix from Diagnostic codes (#526) ([#526](https://github.com/csskit/csskit/pull/526))
- Regenerate css_ast/src/values from csswg drafts (#532) ([#532](https://github.com/csskit/csskit/pull/532))
- Regenerate css_ast/src/values from csswg drafts (#537) ([#537](https://github.com/csskit/csskit/pull/537))
- coverage: instate badly fetched files (#546) ([#546](https://github.com/csskit/csskit/pull/546))
- css_ast: tidy up build files using prettyplease (#560) ([#560](https://github.com/csskit/csskit/pull/560))
- css_ast: Gate Visit behind a feature flag (#562) ([#562](https://github.com/csskit/csskit/pull/562))
- csskit_source_finder: Simplify build.rs files by passing DeriveInput instead of str. (#564) ([#564](https://github.com/csskit/csskit/pull/564))
- Implement Iterator<Item = Cursor> for css_lexer, make Parser generic over this type (#563) ([#563](https://github.com/csskit/csskit/pull/563))
- css_ast: Fix nightly build failures (#570) ([#570](https://github.com/csskit/csskit/pull/570))
- css_ast: Ensure Length is always visitable via LengthPercentage (#568) ([#568](https://github.com/csskit/csskit/pull/568))
- generate-values: sort types lexicographically (#588) ([#588](https://github.com/csskit/csskit/pull/588))
- csskit_proc_macro: drop hardcoded types, rely on ast imports more (#589) ([#589](https://github.com/csskit/csskit/pull/589))
- Leverage cargo-release for unified releases (#591) ([#591](https://github.com/csskit/csskit/pull/591))


### Css_feature_data
- Regenerate css_ast/src/values from csswg drafts (#458) ([#458](https://github.com/csskit/csskit/pull/458))
- Regenerate css_ast/src/values from csswg drafts (#573) ([#573](https://github.com/csskit/csskit/pull/573))


### Css_lexer
- css_lexer: switch to using bumpalo::Vec over bumpalo::String (#502) ([#502](https://github.com/csskit/csskit/pull/502))
- css_lexer: Allow custom allocators, drop bumpalo as a core dependency. (#519) ([#519](https://github.com/csskit/csskit/pull/519))
- css_lexer: delegate Debug trait to underlying str (#521) ([#521](https://github.com/csskit/csskit/pull/521))
- css_lexer: Optimize some hot paths (#549) ([#549](https://github.com/csskit/csskit/pull/549))
- css_lexer: Gate tests that require serde on serde. (#559) ([#559](https://github.com/csskit/csskit/pull/559))


### Css_parse
- css_parse: align(64) diagnostic. (#471) ([#471](https://github.com/csskit/csskit/pull/471))
- css_parse: drop peek_next/peek_including_white_space (#492) ([#492](https://github.com/csskit/csskit/pull/492))
- css_parse: add Cursor buffer (#494) ([#494](https://github.com/csskit/csskit/pull/494))
- css_parse: switch from p.peek::<T> to <T>::peek(p, p.next_n(1)) (#495) ([#495](https://github.com/csskit/csskit/pull/495))
- css_parse: enable the bumpalo feature for css_lexer (#520) ([#520](https://github.com/csskit/csskit/pull/520))
- css_parse: move CursorToSourceCursorSink to its own file (#572) ([#572](https://github.com/csskit/csskit/pull/572))


### Css_value_definition_parser
- csskit_proc_macro: Split def.rs into it's own library. (#590) ([#590](https://github.com/csskit/csskit/pull/590))


### Csskit
- csskit add dbg lex command (#548) ([#548](https://github.com/csskit/csskit/pull/548))


### Csskit_highlight
- csskit_highlight: require chromashift feature in css_ast (#513) ([#513](https://github.com/csskit/csskit/pull/513))
- csskit_highlight: Fix AnsiHighlightCursorStream not separating tokens when needed (#585) ([#585](https://github.com/csskit/csskit/pull/585))


### Csskit_proc_macro
- csskit_proc_macro: remove dead code (#454) ([#454](https://github.com/csskit/csskit/pull/454))


### Csskit_source_finder
- csskit_source_finder: snapshot test the found results (#561) ([#561](https://github.com/csskit/csskit/pull/561))


### Csskit_vscode
- chore(deps): update dependencies (patch) (#459) ([#459](https://github.com/csskit/csskit/pull/459))
- chore(deps): update dependency @types/node to v24.4.0 (#460) ([#460](https://github.com/csskit/csskit/pull/460))
- chore(deps): update dependency oxlint to v1.15.0 (#464) ([#464](https://github.com/csskit/csskit/pull/464))
- chore(deps): update dependency @types/vscode to v1.104.0 (#461) ([#461](https://github.com/csskit/csskit/pull/461))
- chore(deps): update dependency oxlint to v1.16.0 (#488) ([#488](https://github.com/csskit/csskit/pull/488))
- chore(deps): update dependency @types/node to v24.5.2 (#487) ([#487](https://github.com/csskit/csskit/pull/487))
- fix(deps): update dependencies (patch) (#486) ([#486](https://github.com/csskit/csskit/pull/486))
- chore(deps): update dependency @types/node to v24.6.2 (#535) ([#535](https://github.com/csskit/csskit/pull/535))
- fix(deps): update dependencies (patch) (#533) ([#533](https://github.com/csskit/csskit/pull/533))
- chore(deps): update dependency @types/node to v24.7.2 (#539) ([#539](https://github.com/csskit/csskit/pull/539))
- chore(deps): update dependency @types/vscode to v1.105.0 (#540) ([#540](https://github.com/csskit/csskit/pull/540))
- chore(deps): update dependency oxlint to v1.18.0 (#543) ([#543](https://github.com/csskit/csskit/pull/543))
- update dependencies (patch) (#544) ([#544](https://github.com/csskit/csskit/pull/544))
- chore(deps): update dependency oxlint to v1.22.0 (#579) ([#579](https://github.com/csskit/csskit/pull/579))
- chore(deps): update dependency @types/node to v24.8.1 (#575) ([#575](https://github.com/csskit/csskit/pull/575))
- chore(deps): update dependencies (patch) (#574) ([#574](https://github.com/csskit/csskit/pull/574))
- chore(deps): update dependency oxlint to v1.23.0 (#580) ([#580](https://github.com/csskit/csskit/pull/580))


### Derive_atom_set
- derive_atom_set: optimise key lookup performance (#482) ([#482](https://github.com/csskit/csskit/pull/482))
- derive_atom_set: use simpler from_le_bytes rather than bit shift loop (#545) ([#545](https://github.com/csskit/csskit/pull/545))

## [0.0.2] - 2025-09-14

### Other Changes
- Add root license
- Update snapshots
- Update more snapshot
- Update snapshots
- Update coverage snapshots
- Update snapshots
- Update snapshots
- Update the snaps
- Update snaps
- Remove empty error files
- Uncomment run_paser...
- Update snaps
- Update the snaps
- Cleanup last bumpalo/oxc_allocator migration
- Update snapshots!
- Mark some more files as parsable
- Update snapshots!
- Tick
- Update snapshots!
- Update deps
- Foundation now parses
- Add primer to test files
- Gzip snapshots
- Update snapshots!
- Update snapshots!
- Tick some boxes
- Update dependencies (#19) ([#19](https://github.com/csskit/csskit/pull/19))
- Update dependencies (#20) ([#20](https://github.com/csskit/csskit/pull/20))
- Update dependencies (#21) ([#21](https://github.com/csskit/csskit/pull/21))
- Update dependencies (#22) ([#22](https://github.com/csskit/csskit/pull/22))
- Update Rust crate proc-macro2 to v1.0.85 (#23) ([#23](https://github.com/csskit/csskit/pull/23))
- Update Rust crate clap to v4.5.6 (#24) ([#24](https://github.com/csskit/csskit/pull/24))
- Update Rust crate clap to v4.5.7 (#26) ([#26](https://github.com/csskit/csskit/pull/26))
- Update dependencies (#27) ([#27](https://github.com/csskit/csskit/pull/27))
- Update dependencies (#28) ([#28](https://github.com/csskit/csskit/pull/28))
- Update dependencies (#29) ([#29](https://github.com/csskit/csskit/pull/29))
- Update dependencies (#31) ([#31](https://github.com/csskit/csskit/pull/31))
- Update dependencies (#32) ([#32](https://github.com/csskit/csskit/pull/32))
- Update dependencies (#38) ([#38](https://github.com/csskit/csskit/pull/38))
- Update dependencies (#40) ([#40](https://github.com/csskit/csskit/pull/40))
- Update dependencies (#41) ([#41](https://github.com/csskit/csskit/pull/41))
- Update dependencies (#42) ([#42](https://github.com/csskit/csskit/pull/42))
- Update dependencies (#43) ([#43](https://github.com/csskit/csskit/pull/43))
- Update dependencies (#44) ([#44](https://github.com/csskit/csskit/pull/44))
- Update Rust crate syn to v2.0.79 (#45) ([#45](https://github.com/csskit/csskit/pull/45))
- Update Rust crate clap to v4.5.19 (#47) ([#47](https://github.com/csskit/csskit/pull/47))
- Update dependencies (#48) ([#48](https://github.com/csskit/csskit/pull/48))
- Update dependencies (#49) ([#49](https://github.com/csskit/csskit/pull/49))
- Update dependencies (#51) ([#51](https://github.com/csskit/csskit/pull/51))
- Update deps
- Update dependencies (#54) ([#54](https://github.com/csskit/csskit/pull/54))
- Update dependencies (#55) ([#55](https://github.com/csskit/csskit/pull/55))
- Update dependencies (#56) ([#56](https://github.com/csskit/csskit/pull/56))
- Update dependencies (#57) ([#57](https://github.com/csskit/csskit/pull/57))
- Upgrade rust-toolchain to 1.83
- (tasks) upgrade primer
- Update submodules
- Update dependencies
- (cargo) update lock
- Update dependencies (#58) ([#58](https://github.com/csskit/csskit/pull/58))
- Create CODE_OF_CONDUCT.md
- Create SECURITY.md
- Create PRIVACY.md
- Create CODEOWNERS
- Chore(deps): update dependencies (#79) ([#79](https://github.com/csskit/csskit/pull/79))
- (generate-values): fixup script
- Chore(deps): update dependencies (#80) ([#80](https://github.com/csskit/csskit/pull/80))
- Chore(deps): update dependencies (#81) ([#81](https://github.com/csskit/csskit/pull/81))
- Chore(deps): update dependencies (#82) ([#82](https://github.com/csskit/csskit/pull/82))
- Chore(deps): update dependencies (#83) ([#83](https://github.com/csskit/csskit/pull/83))
- Chore(deps): update dependencies (#85) ([#85](https://github.com/csskit/csskit/pull/85))
- Chore(deps): update dependencies (#86) ([#86](https://github.com/csskit/csskit/pull/86))
- Chore(deps): update dependencies (#87) ([#87](https://github.com/csskit/csskit/pull/87))
- Chore(deps): update dependencies (#88) ([#88](https://github.com/csskit/csskit/pull/88))
- Chore(deps): update dependencies (#89) ([#89](https://github.com/csskit/csskit/pull/89))
- Add note about rename to README.md (#93) ([#93](https://github.com/csskit/csskit/pull/93))
- New logo, new site design (#95) ([#95](https://github.com/csskit/csskit/pull/95))
- Chore(deps): update rust crate crossbeam-channel to v0.5.15 [security] (#100) ([#100](https://github.com/csskit/csskit/pull/100))
- Chore(deps): update rust crate clap to v4.5.38 (#102) ([#102](https://github.com/csskit/csskit/pull/102))
- Chore(deps): update dependencies (#103) ([#103](https://github.com/csskit/csskit/pull/103))
- Chore(deps): update dependencies (#107) ([#107](https://github.com/csskit/csskit/pull/107))
- Csskit_zed is in packages (#109) ([#109](https://github.com/csskit/csskit/pull/109))
- Update deps (#116) ([#116](https://github.com/csskit/csskit/pull/116))
- Sync rust's toolchain with mises' (#117) ([#117](https://github.com/csskit/csskit/pull/117))
- Update deps (#215) ([#215](https://github.com/csskit/csskit/pull/215))
- Generate-values: add grammars as comments to commented properties (#263) ([#263](https://github.com/csskit/csskit/pull/263))
- Generate-values: refine heuristics for enum/struct determination (#272) ([#272](https://github.com/csskit/csskit/pull/272))
- Chore(deps): update dependencies (patch) (#324) ([#324](https://github.com/csskit/csskit/pull/324))
- Chore(deps): update rust crate phf to 0.13.0 (#328) ([#328](https://github.com/csskit/csskit/pull/328))
- Chore(deps): update rust crate tracing-subscriber to v0.3.20 [security] (#346) ([#346](https://github.com/csskit/csskit/pull/346))
- Chore(deps): update dependencies (patch) (#381) ([#381](https://github.com/csskit/csskit/pull/381))
- Chore(deps): update rust crate string_cache to 0.9.0 (#387) ([#387](https://github.com/csskit/csskit/pull/387))


### Chromashift
- chromashift: Introduce chromeashift (#413) ([#413](https://github.com/csskit/csskit/pull/413))
- chromashift: Ensure generic Color enum can into all, and all can Into<anstyle> (#415) ([#415](https://github.com/csskit/csskit/pull/415))
- chromashift: Rename hsb to hsv, add Display trait, plus more From<X>s (#416) ([#416](https://github.com/csskit/csskit/pull/416))
- Add a bunch of Readmes (#422) ([#422](https://github.com/csskit/csskit/pull/422))
- csskit: Minor improvements to colors command: (#423) ([#423](https://github.com/csskit/csskit/pull/423))
- css_lexer: mark a bunch of functions const (#429) ([#429](https://github.com/csskit/csskit/pull/429))


### Css_ast
- Extensively document and clean up css_lexer/css_parse (#78) ([#78](https://github.com/csskit/csskit/pull/78))
- (ast/parser) move comma from compoundselector to selectorlist
- chore: rename hdx to csskit (#92) ([#92](https://github.com/csskit/csskit/pull/92))
- migrate & centralise types away from values (#97) ([#97](https://github.com/csskit/csskit/pull/97))
- Run generate-values, update versions (#110) ([#110](https://github.com/csskit/csskit/pull/110))
- Implementing `<font-weight-absolute>` (#62) ([#62](https://github.com/csskit/csskit/pull/62))
- Implement more `align` types (#112) ([#112](https://github.com/csskit/csskit/pull/112))
- add easing_function support (#114) ([#114](https://github.com/csskit/csskit/pull/114))
- Implements `glyph-orientation-vertical` along with IntLiteral and DimensionLiteral value defs (#115) ([#115](https://github.com/csskit/csskit/pull/115))
- Implements ordered combinators and uncomments some align properties (#113) ([#113](https://github.com/csskit/csskit/pull/113))
- (css_parse, css_lexer) Deny warnings (#119) ([#119](https://github.com/csskit/csskit/pull/119))
- more deny(warnings) (#122) ([#122](https://github.com/csskit/csskit/pull/122))
- More deny warnings (#123) ([#123](https://github.com/csskit/csskit/pull/123))
- Support + Define values with <type>+ syntax (#125) ([#125](https://github.com/csskit/csskit/pull/125))
- drop empty scroll_animations mod (#126) ([#126](https://github.com/csskit/csskit/pull/126))
- Further scope deny warnings in css ast (#127) ([#127](https://github.com/csskit/csskit/pull/127))
- tidy up many warnings in css_ast types (#128) ([#128](https://github.com/csskit/csskit/pull/128))
- Cargo fmt (#129) ([#129](https://github.com/csskit/csskit/pull/129))
- Bookmark label scaffold out content list type (#130) ([#130](https://github.com/csskit/csskit/pull/130))
- Add scroll-margin property (#131) ([#131](https://github.com/csskit/csskit/pull/131))
- updat ui - appearance: base-select (#133) ([#133](https://github.com/csskit/csskit/pull/133))
- clean up double comment (#134) ([#134](https://github.com/csskit/csskit/pull/134))
- Update regions (#132) ([#132](https://github.com/csskit/csskit/pull/132))
- Implements some font style values (#135) ([#135](https://github.com/csskit/csskit/pull/135))
- Correctly handle parsing optional bounded types (#136) ([#136](https://github.com/csskit/csskit/pull/136))
- Reintroduce derive(Parse/Peek/ToCursors) (#138) ([#138](https://github.com/csskit/csskit/pull/138))
- implement a whole bunch of values (#141) ([#141](https://github.com/csskit/csskit/pull/141))
- enable text-decoration-trim (#143) ([#143](https://github.com/csskit/csskit/pull/143))
- enable cue-before, cue-after (#144) ([#144](https://github.com/csskit/csskit/pull/144))
- enable text-emphasis-skip (#145) ([#145](https://github.com/csskit/csskit/pull/145))
- (css_ast): add tests for zoom (#146) ([#146](https://github.com/csskit/csskit/pull/146))
- (css_ast): Enable aspect-ratio (#147) ([#147](https://github.com/csskit/csskit/pull/147))
- (css_ast): Implements `<shadow>` (#161) ([#161](https://github.com/csskit/csskit/pull/161))
- Write blanket impls for more ToCursors types, and Derive ToCursors where we can
- css_ast: derive ToCursors for many more types (#164) ([#164](https://github.com/csskit/csskit/pull/164))
- css_ast/css_parse: derive ToCursors for many more types (#165) ([#165](https://github.com/csskit/csskit/pull/165))
- Prefer From<T> for Cursor (over <&T>), From<&T> for Span (over <T>) (#166) ([#166](https://github.com/csskit/csskit/pull/166))
- implement IntoCursor derive (#167) ([#167](https://github.com/csskit/csskit/pull/167))
- (css_ast): Implements `transform` (#162) ([#162](https://github.com/csskit/csskit/pull/162))
- (css_ast): enable transform property (#169) ([#169](https://github.com/csskit/csskit/pull/169))
- (csskit_derives): clean up derive code (#170) ([#170](https://github.com/csskit/csskit/pull/170))
- (css_ast): refactors `::units::time` to peek/parse slightly better (#172) ([#172](https://github.com/csskit/csskit/pull/172))
- (css_ast): enables all transition properties (#171) ([#171](https://github.com/csskit/csskit/pull/171))
- css_ast: make todo types use the Todo struct (#173) ([#173](https://github.com/csskit/csskit/pull/173))
- css_ast: derive IntoSpan for all AST Nodes (#174) ([#174](https://github.com/csskit/csskit/pull/174))
- (css_ast): enables `<content-list>` (#175) ([#175](https://github.com/csskit/csskit/pull/175))
- Implement -moz- pseudo elements, and some blanket impls for Peek/Parse/ToCursors (#176) ([#176](https://github.com/csskit/csskit/pull/176))
- (css_ast): enables `<grid-line>` (#180) ([#180](https://github.com/csskit/csskit/pull/180))
- (css_ast): derive `Parse` for a few more Vec impls (#177) ([#177](https://github.com/csskit/csskit/pull/177))
- Modify all Into<Span>/From<T> for Span to `ToSpan` trait (#182) ([#182](https://github.com/csskit/csskit/pull/182))
- uncomment and clean up Position tests  (#183) ([#183](https://github.com/csskit/csskit/pull/183))
- enable cargo clippy in CI (#186) ([#186](https://github.com/csskit/csskit/pull/186))
- (css_ast): removes warnings from single_transition (#189) ([#189](https://github.com/csskit/csskit/pull/189))
- (css_ast): fix `<grid-line>` optionals implementation (#188) ([#188](https://github.com/csskit/csskit/pull/188))
- add comma separated struct (#185) ([#185](https://github.com/csskit/csskit/pull/185))
- Introduces Optionals![A, B, ...] and uses them in a bunch of places (#181) ([#181](https://github.com/csskit/csskit/pull/181))
- css_ast: zoom is not a set of optionals (#193) ([#193](https://github.com/csskit/csskit/pull/193))
- Generate more data in style values, such as caniuse, baseline, browser versions, and comments. (#191) ([#191](https://github.com/csskit/csskit/pull/191))
- clean up keyword_set!/*_feature!/pseudo_class!/pseudo_element! uses (#198) ([#198](https://github.com/csskit/csskit/pull/198))
- Create Visitable derive macro, derive it everywhere. (#199) ([#199](https://github.com/csskit/csskit/pull/199))
- csskit_proc_macro/css_ast: use heck for string transforms (#200) ([#200](https://github.com/csskit/csskit/pull/200))
- css_ast: tidy up build script (#203) ([#203](https://github.com/csskit/csskit/pull/203))
- css_ast: avoid Vec in Nth type (#201) ([#201](https://github.com/csskit/csskit/pull/201))
- css_parse/css_ast: add vis + enum type for function_set! (#204) ([#204](https://github.com/csskit/csskit/pull/204))
- css_parse/css_ast: add vis + enum type for atkeyword_set! (#205) ([#205](https://github.com/csskit/csskit/pull/205))
- css_ast: use atkeyword_set, function_set a little more (#206) ([#206](https://github.com/csskit/csskit/pull/206))
- css_parse/css_ast: make function/atkeyword newtype over their respective token macros (#207) ([#207](https://github.com/csskit/csskit/pull/207))
- css_parse/css_ast/csskit_proc_macro: make keyword_set newtype over T![Ident] token macro (#208) ([#208](https://github.com/csskit/csskit/pull/208))
- css_ast: support url()/src() functions in images (#209) ([#209](https://github.com/csskit/csskit/pull/209))
- refactor many traits into generic structs (#210) ([#210](https://github.com/csskit/csskit/pull/210))
- csskit_derives/css_ast: allow state/stop to be set in derive(parse) (#211) ([#211](https://github.com/csskit/csskit/pull/211))
- css_parse/css_ast: move parsing steps for Declarations into parse (#212) ([#212](https://github.com/csskit/csskit/pull/212))
- remove a couple of dbg!s (#225) ([#225](https://github.com/csskit/csskit/pull/225))
- Regenerate css_ast/src/values from csswg drafts (#242) ([#242](https://github.com/csskit/csskit/pull/242))
- css_parse/css_ast: move rule parsing steps into css_parse (#213) ([#213](https://github.com/csskit/csskit/pull/213))
- Regenerate css_ast/src/values from csswg drafts (#244) ([#244](https://github.com/csskit/csskit/pull/244))
- css_ast: move justify content and align items out of flex (#245) ([#245](https://github.com/csskit/csskit/pull/245))
- css_ast: Implement text-combine-upright style value. (#247) ([#247](https://github.com/csskit/csskit/pull/247))
- css_ast: Fixup Resolution unit (#249) ([#249](https://github.com/csskit/csskit/pull/249))
-  css_parse: Implement generic Function<FT, T> struct.  (#250) ([#250](https://github.com/csskit/csskit/pull/250))
- css_ast: Implement CursorStyleValue (#253) ([#253](https://github.com/csskit/csskit/pull/253))
- remove dbg! in stylesheet.rs (#254) ([#254](https://github.com/csskit/csskit/pull/254))
- optimise Alternatives of auto|<length(-percentage)> to <length(-percentage)-or-auto> (#255) ([#255](https://github.com/csskit/csskit/pull/255))
- css_ast: Implement ScrollSnapTypeStyleValue, ScrollPaddingBlockStyleValue, ScrollPaddingInlineStyleValue (#258) ([#258](https://github.com/csskit/csskit/pull/258))
- Implement TimeOrAuto, optimize for it in Defs, and uncomment AnimationDurationStyleValue. (#259) ([#259](https://github.com/csskit/csskit/pull/259))
- Regenerate css_ast/src/values from csswg drafts (#260) ([#260](https://github.com/csskit/csskit/pull/260))
- css_ast: fix popularity to 3dp (#264) ([#264](https://github.com/csskit/csskit/pull/264))
- css_ast: Implement BoxShadowStyleValue, TextShadowStyleValue, TransformBoxStyleValue, OutlineStyleValue (#265) ([#265](https://github.com/csskit/csskit/pull/265))
- css_ast/csskit_proc_macro: Implement border-clip & friends (#266) ([#266](https://github.com/csskit/csskit/pull/266))
- css ast implement corner shape value (#267) ([#267](https://github.com/csskit/csskit/pull/267))
- Regenerate css_ast/src/values from csswg drafts (#274) ([#274](https://github.com/csskit/csskit/pull/274))
- css_ast: Implement AnimationNameStyleValue (#276) ([#276](https://github.com/csskit/csskit/pull/276))
- Regenerate css_ast/src/values from csswg drafts (#278) ([#278](https://github.com/csskit/csskit/pull/278))
- implement Peek for Nth (#284) ([#284](https://github.com/csskit/csskit/pull/284))
- implement :heading (#285) ([#285](https://github.com/csskit/csskit/pull/285))
- css_ast: Implement BorderImageRepeatStyleValue (#289) ([#289](https://github.com/csskit/csskit/pull/289))
- css_ast: Implement BoxShadowPositionStyleValue (#290) ([#290](https://github.com/csskit/csskit/pull/290))
- css_ast: Implement OverscrollBehaviorStyleValue (#291) ([#291](https://github.com/csskit/csskit/pull/291))
- css_ast: Implement ScrollSnapAlignStyleValue (#292) ([#292](https://github.com/csskit/csskit/pull/292))
- css_ast: Implement LinkParametersStyleValue (#293) ([#293](https://github.com/csskit/csskit/pull/293))
-  css_ast: Add contain keywords for width/height/etc  (#294) ([#294](https://github.com/csskit/csskit/pull/294))
- Regenerate css_ast/src/values from csswg drafts (#295) ([#295](https://github.com/csskit/csskit/pull/295))
- css_ast: Refactor all functions into functions folder (#303) ([#303](https://github.com/csskit/csskit/pull/303))
- Implement FontFamilyStyleValue (and FamilyName / GenericFamily types) (#304) ([#304](https://github.com/csskit/csskit/pull/304))
- Implement BackgroundImageStyleValue, fixing up BgImage (#305) ([#305](https://github.com/csskit/csskit/pull/305))
- Implement BackgroundSizeStyleValue (and BgSize) (#306) ([#306](https://github.com/csskit/csskit/pull/306))
- Implement combined types NumberPercentage, NumberLength, NumberLengthOrAuto. (#307) ([#307](https://github.com/csskit/csskit/pull/307))
- css_ast: Implement BorderImageOutsetStyleValue, ScaleStyleValue (#308) ([#308](https://github.com/csskit/csskit/pull/308))
- css_ast: drop Default from Visit/VisitMut (#309) ([#309](https://github.com/csskit/csskit/pull/309))
- css_ast/css_parse: Add new methods to DeclarationValue. (#316) ([#316](https://github.com/csskit/csskit/pull/316))
- css_ast: Add transforms-2 functions. (#317) ([#317](https://github.com/csskit/csskit/pull/317))
- Make all StyleValues visitable. (#318) ([#318](https://github.com/csskit/csskit/pull/318))
- Do not generate function types within generate.rs, defer to hand coded types (#319) ([#319](https://github.com/csskit/csskit/pull/319))
- Simplify Function<'a, FT, T> to Function<FT, T> (#320) ([#320](https://github.com/csskit/csskit/pull/320))
- css_ast: Make a whole bunch of types Visitable (#321) ([#321](https://github.com/csskit/csskit/pull/321))
- Regenerate css_ast/src/values from csswg drafts (#323) ([#323](https://github.com/csskit/csskit/pull/323))
- csskit: Get basic fmt, minify commands working. (#335) ([#335](https://github.com/csskit/csskit/pull/335))
- move tasks dir (#336) ([#336](https://github.com/csskit/csskit/pull/336))
- Avoid css_lexer dep in downstream traits, clean up generic structs. (#337) ([#337](https://github.com/csskit/csskit/pull/337))
- css ast drop css lexer as a dependency (#339) ([#339](https://github.com/csskit/csskit/pull/339))
- css_ast: Introduce AutoOr<T>, NoneOr<T>, AutoNoneOr<T> types. (#340) ([#340](https://github.com/csskit/csskit/pull/340))
- css_ast: Tighten up transform functions (#342) ([#342](https://github.com/csskit/csskit/pull/342))
- css_ast: Add assert_visits! test & some tests (#345) ([#345](https://github.com/csskit/csskit/pull/345))
- Regenerate css_ast/src/values from csswg drafts (#348) ([#348](https://github.com/csskit/csskit/pull/348))
- Drop derive()s from proc macro generation, add to mod.ts instead. (#349) ([#349](https://github.com/csskit/csskit/pull/349))
- rename #[value()] to #[syntax()] (#350) ([#350](https://github.com/csskit/csskit/pull/350))
-  css_ast: Use #[syntax] for AnchorName (#351) ([#351](https://github.com/csskit/csskit/pull/351))
- css_ast: Use #[syntax] for BgSize (#352) ([#352](https://github.com/csskit/csskit/pull/352))
- css_ast: Use #[syntax] for FamilyName (#353) ([#353](https://github.com/csskit/csskit/pull/353))
- css_ast: Use #[syntax] for FontWeightAbsolute (#354) ([#354](https://github.com/csskit/csskit/pull/354))
- css_ast: Use #[syntax] for GenericFamily (#355) ([#355](https://github.com/csskit/csskit/pull/355))
- css_ast: Improve dynamic-range-limit (#360) ([#360](https://github.com/csskit/csskit/pull/360))
- css_ast: Refactor & simplify CornerShapeValue code (#363) ([#363](https://github.com/csskit/csskit/pull/363))
- css_ast: Implement TryFrom for f32 for Auto/None/AutoOrNone<T> types (#364) ([#364](https://github.com/csskit/csskit/pull/364))
- Introduce ToNumberValue to aid in number checking code (#365) ([#365](https://github.com/csskit/csskit/pull/365))
- css_ast: Genericise MIN for Visitable/VisitableMut on CommaSeparated<T, MIN> (#371) ([#371](https://github.com/csskit/csskit/pull/371))
- css_ast: derive(Parse) on more types (#372) ([#372](https://github.com/csskit/csskit/pull/372))
- Remove Parse impl generation from generate.rs. Rely on derive(Parse) as much as possible. (#376) ([#376](https://github.com/csskit/csskit/pull/376))
- css_ast: Export apply_visit_methods (#377) ([#377](https://github.com/csskit/csskit/pull/377))
- Implement <auto-line-width-list> (#379) ([#379](https://github.com/csskit/csskit/pull/379))
- Regenerate css_ast/src/values from csswg drafts (#380) ([#380](https://github.com/csskit/csskit/pull/380))
- css_ast: Add new CSS pseudo classes & elements. (#389) ([#389](https://github.com/csskit/csskit/pull/389))
- css_feature_data: Add new css feature data! (#390) ([#390](https://github.com/csskit/csskit/pull/390))
- css_lexer/css_parse: Introduce AssociatedWhitespaceRules (#393) ([#393](https://github.com/csskit/csskit/pull/393))
- css_ast: Make a bunch of fields public. (#400) ([#400](https://github.com/csskit/csskit/pull/400))
- csskit_proc_macro: Ensure AllMustOccur combinators at root annotate struct (#402) ([#402](https://github.com/csskit/csskit/pull/402))
- css_ast: Clean up various types, using derive(Parse). (#403) ([#403](https://github.com/csskit/csskit/pull/403))
- css_ast: Simplify ColorFunction by separating out into structs that can derive(Parse) (#405) ([#405](https://github.com/csskit/csskit/pull/405))
- css_ast: Shrink color function size by using CommaOrSlash struct (#407) ([#407](https://github.com/csskit/csskit/pull/407))
- css_ast: Implement normalisation of some values. (#408) ([#408](https://github.com/csskit/csskit/pull/408))
- css_ast: Add chromashift support. (#414) ([#414](https://github.com/csskit/csskit/pull/414))
- css_ast: Fix Oklab ToChromashift values (#417) ([#417](https://github.com/csskit/csskit/pull/417))
- Regenerate css_ast/src/values from csswg drafts (#427) ([#427](https://github.com/csskit/csskit/pull/427))
- Tidy up diagnostics, split them into respective crates (#448) ([#448](https://github.com/csskit/csskit/pull/448))
- css_parse: drop parse! macro (#450) ([#450](https://github.com/csskit/csskit/pull/450))


### Css_feature_data
- css_feature_data: Ensure browserlist tests are feature gated (#406) ([#406](https://github.com/csskit/csskit/pull/406))
- Regenerate css_ast/src/values from csswg drafts (#430) ([#430](https://github.com/csskit/csskit/pull/430))


### Css_lexer
- css_lexer: Fix ToSpan on Vec<'a, T: ToSpan> (#310) ([#310](https://github.com/csskit/csskit/pull/310))
- css_lexer: Fix quote_style() (#329) ([#329](https://github.com/csskit/csskit/pull/329))
- css_lexer: Allow String tokens to change quote style. (#330) ([#330](https://github.com/csskit/csskit/pull/330))
- css_lexer: Ensure Span::DUMMY can be used without being added to (#341) ([#341](https://github.com/csskit/csskit/pull/341))
- css_parse/css_lexer: Fix requiring whitespace between dimension and `-` (#388) ([#388](https://github.com/csskit/csskit/pull/388))
- css_lexer: Refactor Token const to use const fn new_delim/new_delim_kind (#391) ([#391](https://github.com/csskit/csskit/pull/391))
- css_lexer: Stop storing length in flags. (#392) ([#392](https://github.com/csskit/csskit/pull/392))
- move SourceCursor into css_lexer & refine (#398) ([#398](https://github.com/csskit/csskit/pull/398))
- css_lexer: Eagerly parse hex values & encode them in Token data (#399) ([#399](https://github.com/csskit/csskit/pull/399))
- css_lexer: Fix hex_value parsing (#409) ([#409](https://github.com/csskit/csskit/pull/409))
- css_lexer: clippy fix (#420) ([#420](https://github.com/csskit/csskit/pull/420))
- release: 0.0.2 (#451) ([#451](https://github.com/csskit/csskit/pull/451))


### Css_parse
- css_parse: fix doc warnings (#120) ([#120](https://github.com/csskit/csskit/pull/120))
- Enhances CI to also run doc builds (#168) ([#168](https://github.com/csskit/csskit/pull/168))
- (css_ast): run cargo fmt (#178) ([#178](https://github.com/csskit/csskit/pull/178))
- Implement asserting based on ast's (#184) ([#184](https://github.com/csskit/csskit/pull/184))
- csskit_proc_macro: tidy up def.rs & break out into generate.rs (#197) ([#197](https://github.com/csskit/csskit/pull/197))
- css_parse: implement singular function_set, Into::T![Function] (#251) ([#251](https://github.com/csskit/csskit/pull/251))
- csskit_proc_macro: Generate empty CommaSeparated with new_in, not Default (#252) ([#252](https://github.com/csskit/csskit/pull/252))
- css_parse: impl CursorSink for Vec (#311) ([#311](https://github.com/csskit/csskit/pull/311))
- css_parse: Add SourceCursor, SourceCursorSink, impl SourceCursorSink for fmt::Write (#312) ([#312](https://github.com/csskit/csskit/pull/312))
- css_parse: Rename CursorFmtSink to CursorWriteSink (#313) ([#313](https://github.com/csskit/csskit/pull/313))
- css_parse: Introduce CursorPrettyWriteSink (#314) ([#314](https://github.com/csskit/csskit/pull/314))
- css_parse: implement CursorOverlaySink (#315) ([#315](https://github.com/csskit/csskit/pull/315))
- css_parse: Add parse! macro (#331) ([#331](https://github.com/csskit/csskit/pull/331))
- css_parse: Normalize quote styles in CursorPrettyWriteSink (#332) ([#332](https://github.com/csskit/csskit/pull/332))
- css_parse: Add CursorCompactWriteSink (#333) ([#333](https://github.com/csskit/csskit/pull/333))
- css_parse: Use Span::DUMMY for Optionals (#343) ([#343](https://github.com/csskit/csskit/pull/343))
- css_parse: Refactor assert_parse_span! (#344) ([#344](https://github.com/csskit/csskit/pull/344))
- css_parse: Give CommaSeparated the ability to express minimums (#369) ([#369](https://github.com/csskit/csskit/pull/369))
- css_parse: Allow CursorPrettyWriteSink/CursorCompactWriteSink to push to other sinks. (#396) ([#396](https://github.com/csskit/csskit/pull/396))
- csskit_highlight/csskit: Introduce ansi colors (#397) ([#397](https://github.com/csskit/csskit/pull/397))
- css_parse: drop debugs from parse_entirely (#418) ([#418](https://github.com/csskit/csskit/pull/418))


### Csskit
- update deps (#229) ([#229](https://github.com/csskit/csskit/pull/229))
- csskit: Tidy up main.rs (#394) ([#394](https://github.com/csskit/csskit/pull/394))
- csskit: Allow stdin as a file for fmt/min (#395) ([#395](https://github.com/csskit/csskit/pull/395))
- csskit: Modularise commands (#404) ([#404](https://github.com/csskit/csskit/pull/404))
- csskit: Tidy up commands more. (#412) ([#412](https://github.com/csskit/csskit/pull/412))
- csskit: add new colo(u)rs command (#419) ([#419](https://github.com/csskit/csskit/pull/419))
- csskit: colors command - Show line and column that a colour came from. (#421) ([#421](https://github.com/csskit/csskit/pull/421))


### Csskit_derives
- publish to crates.io also (#237) ([#237](https://github.com/csskit/csskit/pull/237))
- csskit_proc_macro: Visit sub-types of StyleValues (#322) ([#322](https://github.com/csskit/csskit/pull/322))
- csskit_proc_macro: Use csskit_derives::Peek instead of custom implementations (#347) ([#347](https://github.com/csskit/csskit/pull/347))
- csskit_derives: Add a whole bunch of tests for csskit_derives (#356) ([#356](https://github.com/csskit/csskit/pull/356))
- csskit_proc_macro/csskit_derives: Add #[parse(in_range=X..Y?)] syntax (#358) ([#358](https://github.com/csskit/csskit/pull/358))
- csskit_proc_macro/csskit_derives: Implement AllMustOccur simple cases (#359) ([#359](https://github.com/csskit/csskit/pull/359))
- csskit_derives: Add support for parsing as keyword (#361) ([#361](https://github.com/csskit/csskit/pull/361))
- csskit_proc_macro: Use #[parse(keywords = )] annotation in #[syntax] when derive(Parse) (#362) ([#362](https://github.com/csskit/csskit/pull/362))
- css_parse/csskit_proc_macro: Bring Optionals parsing into derive(Parse) (#367) ([#367](https://github.com/csskit/csskit/pull/367))
- csskit_derives: Ensure parsing one_must_occur doesn't try to parse optionals (#368) ([#368](https://github.com/csskit/csskit/pull/368))
- csskit_derives: Use syn "full" feature. (#410) ([#410](https://github.com/csskit/csskit/pull/410))


### Csskit_highlight
- csskit_highlight: disable bench (#196) ([#196](https://github.com/csskit/csskit/pull/196))
- csskit_highlight: Add a whole bunch of tokens & styling. (#401) ([#401](https://github.com/csskit/csskit/pull/401))


### Csskit_lsp
- cargo update (#194) ([#194](https://github.com/csskit/csskit/pull/194))


### Csskit_proc_macro
- Update to 2024 Rust Edition (#118) ([#118](https://github.com/csskit/csskit/pull/118))
- (css_proc_macro): Do not fallthrough when unreachable (#124) ([#124](https://github.com/csskit/csskit/pull/124))
- Refactor def.rs for better codegen (#142) ([#142](https://github.com/csskit/csskit/pull/142))
-  csskit_proc_macro: Elide DefGroupStyle::None where possible in def parsing. (#243) ([#243](https://github.com/csskit/csskit/pull/243))
- csskit_proc_macro: Ensure DefRange of Range is inclusive. (#246) ([#246](https://github.com/csskit/csskit/pull/246))
- csskit_proc_macro: Refactor & simplify Def::Multiplier (#248) ([#248](https://github.com/csskit/csskit/pull/248))
- csskit_proc_macro: Remove DefGroupStyle::Range/OneMustOccur, use Def::Multiplier instead (#256) ([#256](https://github.com/csskit/csskit/pull/256))
- csskit_proc_macro: split out optimization pass of Def parsing, optimize more (#257) ([#257](https://github.com/csskit/csskit/pull/257))
- csskit_proc_macro: Generate structs for multipliers of keywords (#268) ([#268](https://github.com/csskit/csskit/pull/268))
- csskit_proc_macro: Generate sub-types where applicable (#273) ([#273](https://github.com/csskit/csskit/pull/273))
- csskit_proc_macro: Refine generating subtypes (#275) ([#275](https://github.com/csskit/csskit/pull/275))
- csskit proc macro properly gather required idents (#277) ([#277](https://github.com/csskit/csskit/pull/277))
- csskit_proc_macro: implement bounded multiplier of keywords (#286) ([#286](https://github.com/csskit/csskit/pull/286))
- csskit_proc_macro: Refine generation for bounded multiplier of keywords (#287) ([#287](https://github.com/csskit/csskit/pull/287))
- csskit_proc_macro: Only add #[visit] attrs if derive(Visitable), avoid Parse impl if derive(Parse) (#357) ([#357](https://github.com/csskit/csskit/pull/357))
- csskit_proc_macro: use #[parse] attrs for literal int/dimension types when derive(Parse) (#366) ([#366](https://github.com/csskit/csskit/pull/366))
- csskit_proc_macro: Leverage CommaSeparated<_, MIN> in #[syntax] generation (#370) ([#370](https://github.com/csskit/csskit/pull/370))
- csskit_proc_macro: Improve support for AllMustOccur with checks (#373) ([#373](https://github.com/csskit/csskit/pull/373))
- csskit_proc_macro: ensure attrs expand to inner variants on enum (#375) ([#375](https://github.com/csskit/csskit/pull/375))
- csskit_proc_macro: derive(Parse) for generated subtypes (#374) ([#374](https://github.com/csskit/csskit/pull/374))
- csskit_proc_macro: move & split tests to test/test_generate, test/test_def (#378) ([#378](https://github.com/csskit/csskit/pull/378))


### Csskit_vscode
- fix(deps): update dependencies (#104) ([#104](https://github.com/csskit/csskit/pull/104))
- chore(deps): update dependencies (#105) ([#105](https://github.com/csskit/csskit/pull/105))
- chore(deps): update dependency oxlint to v1 (#106) ([#106](https://github.com/csskit/csskit/pull/106))
- update csskit_vscode deps (#238) ([#238](https://github.com/csskit/csskit/pull/238))
- Update dependencies (#262) ([#262](https://github.com/csskit/csskit/pull/262))
- update deps (#271) ([#271](https://github.com/csskit/csskit/pull/271))
- chore(deps): update dependencies (patch) (#281) ([#281](https://github.com/csskit/csskit/pull/281))
- chore(deps): update dependency oxlint to v1.12.0 (#300) ([#300](https://github.com/csskit/csskit/pull/300))
- chore(deps): update dependency @types/vscode to v1.103.0 (#298) ([#298](https://github.com/csskit/csskit/pull/298))
- chore(deps): update dependency @types/node to v24.3.0 (#297) ([#297](https://github.com/csskit/csskit/pull/297))
- fix(deps): update dependencies (patch) (#296) ([#296](https://github.com/csskit/csskit/pull/296))
- chore(deps): update dependency oxlint to v1.13.0 (#384) ([#384](https://github.com/csskit/csskit/pull/384))
- chore(deps): update dependency oxlint to v1.14.0 (#386) ([#386](https://github.com/csskit/csskit/pull/386))
- fix(deps): update dependencies (patch) (#425) ([#425](https://github.com/csskit/csskit/pull/425))


### Csskit_zed
- fix repository org name (#98) ([#98](https://github.com/csskit/csskit/pull/98))
- fix(deps): update dependencies (#101) ([#101](https://github.com/csskit/csskit/pull/101))
- fix(deps): update rust crate zed_extension_api to 0.6 (#111) ([#111](https://github.com/csskit/csskit/pull/111))


### Hdx
- initial commit
- add license
- fix npm package
- Rewrite Lexer & Parser (#1) ([#1](https://github.com/csskit/csskit/pull/1))
- get 960gs working again
- another round of improvements
- tidy up a bunch
- drop oxc_allocator, use Bumpalo with serde, upgrade deps
- hack weekend
- use criterion compare (#4) ([#4](https://github.com/csskit/csskit/pull/4))
- Another big rewrite...
- (writer) remove hdx_writer
- (bin) apply changes from parser/ast
- (derive) remove hdx_derive
- clean up some dead code
- Add Semantic Token Highlighting (#64) ([#64](https://github.com/csskit/csskit/pull/64))
- Add hidden dbg parse command (#65) ([#65](https://github.com/csskit/csskit/pull/65))
- (lsp) scaffold out basic lsp server (#69) ([#69](https://github.com/csskit/csskit/pull/69))
- Impove highlighting (#72) ([#72](https://github.com/csskit/csskit/pull/72))
- (lsp) rewrite LSP service using traits


### Hdx_ast
- add bordershorthand & linewidth
- implement list-style shorthand
- implement Quotes property
- fixup broken @page rule parse/write
- implement charset rule
- add border shorthands
- implement white-space and friends
- Add more border property values
- add page break values
- rename some structs to be closer to CSS OM
- remove redunant file
- tweak seralisation
- tidy up a bunch
- a bunch more fixes that I can't be bothered to separate into individual commits
- no more boxup
- get a bunch more properties working
- fix size test
- first bit of media queries
- work more on minification
- fixup parser tests
- partialeq
- make a much nicer test harness and also add supports rules
- drop empty style rules
- fix font-size: 0
- fix line-height: 0
- support all descre media queries, add some webkit prefixes, and other stuff
- better handling of non standard properties
- clean up impl Value, use derive instead
- add transition delay/duration
- add opacity
- Fix test-size-adjust
- make a cleaner impl for parsing with state, using traits
- rewrite selector parsing
- smol tests
- add font-style
- add vendor prefixed font-smooth
- ensure font-style inherits
- add text-rendering nonstandard
- Fix up combinator selector parsing
- font-variant properties
- quotes
- borders
- fixup display: list-item
- serialize properties better
- fixup erroneous serde flags
- fix 960 parsing
- fix aspec-ratio typo
- drop NonStandard enum prefix
- ranged media queries
- media rules can nest
- fix serde serializations
- add -o- and -ms- vendor prefix selectors
- add non-standard media features
- add webkit-calendar-date-picker-indicator pseudo element
- fixes to get boostrap working
- add a bunch of rule todos, and implement Keyframes
- more media features
- yet more vendor pseudos
- serialize supportscondition properly
- add nesting combinator
- dont swallow whitespace adjacent to nesting combinator
- ensure style rules output sub-rules
- ensure nesting combinator preserves prior descendant combinator
- add some more nesting tests for good measure
- rewrite media query parsing
- get font-face kind of working
- add supports for parsing IE Media Query hacks
- tidy up (rustfmt and fix warnings)
- fix primer parse issues
- tidy up values & types. Get some more values parsing
- add visitor pattern, add first transformer
- Ci passing (#2) ([#2](https://github.com/csskit/csskit/pull/2))
- add benchmarking (#3) ([#3](https://github.com/csskit/csskit/pull/3))
- drop FromToken (#5) ([#5](https://github.com/csskit/csskit/pull/5))
- Rewrite lexer (#50) ([#50](https://github.com/csskit/csskit/pull/50))
- remove commented out code
- (values) support for will-change
- (values) AnimationName should be a struct
- (values) support for animation-iteration-count
- (ast/units) fixup Time unit parsing
- (ast/values) use length-percentage for FloatOffset
- (ast/types) fix parsing for position
- (ast/types) fix parsing for gradient
- (ast/types) fix parsing for absolutecolorfunction
- (ast/types) add systemcolor keywords
- (ast/values) implement some animation properties
- cargo fmt
- (ast/values) implement sizing properties
- (ast/values) implement some overflow properties
- (ast/values) properly test implemented align values
- (proc_macro) optimize for bounded range syntax such <overflow>{1,2}
- (ast/values) implement padding/margin
- (ast/values) position tests
- (ast/values) implement a bunch of borders properties
- (ast/values) implement some anchor position properties
- (ast/values) implement some background properties
- (ast/values) implement color Opacity value
- (ast/values) implement some transition properties
- (ast/values) implement some Content properties
- (ast/values) implement some ScrollSnap properties
- (ast/values) implement float properties
- cargo fmt
- (ast/values) implement logical properties
- (ast/values) implement flex-basis property
- (ast/selector) rewrite nth selector parsing
- (ast/properties) fixup uknown property parsing
- (ast/media) fixup media query parsing
- (ast/media) extract condition parsing into trait
- (ast/supports) fixup supports parsing, add selector()
- (ast/stylerule) fixup stylerule parsing
- cargo fmt
- (ast/stylesheet) fixup error recovery on unknown rules
- remove dbg!s
- (ast/syntax) fix simpleblock parsing
- (ast/syntax) fix nested simpleblock parsing
- (ast/syntax) fix baddeclaration parsing
- rename Token!/Delim!/Dimension! to T!
- rename parser to p
- remove unused discard! macro
- cargo fix
- (ast) reflect changes from parser+lexer
- (ast) update snapshots
- Disable pprof in windows (#59) ([#59](https://github.com/csskit/csskit/pull/59))
- (ast) add support for @layer
- (ast) add support for @document (and @-moz-document)
- (ast) add color_hdr support (#61) ([#61](https://github.com/csskit/csskit/pull/61))
- (ast) fix typo
- (ast) implement @property rule parsing
- (ast) rename all rule nodes to be suffixed "Rule"
- (ast/parser) add support for @container queries
- (ast) fix a couple of errors in @container serialization
- update snapshots
- re-enable a bunch of snapshot tests
- Adds support for fixed ranges, like <color>{2} (#63) ([#63](https://github.com/csskit/csskit/pull/63))
- Resolve some windows ci failures (#60) ([#60](https://github.com/csskit/csskit/pull/60))
- (ast) rename "Custom" to "CustomDimension"
- clean up some warnings


### Hdx_atom
- peek at windows ci failures
- (atom) drop debug message from build
- (atom) drop debug message from build
- (atom) drop debug message from build


### Hdx_derive
- (lexer) a lot more refinement+breaking changes
- (derive) drop unused derives


### Hdx_lexer
- drop old symlink
- rename Span up_to to until
- rebuild snapshots
- (lexer) fixup dimension numbers for small values
- cleanup lexer benchmark
- (lexer) fix eq_ignore_ascii_case
- (lexer) fix eq_ignore_ascii_case


### Hdx_lsp
- fix: we cannot pprof in windows (#74) ([#74](https://github.com/csskit/csskit/pull/74))
- (lsp) use string ropes for incremental edits, scoped per-file threads for parsing
- (lsp) fix tests
- (lsp) tidy up dead code
- (lsp) get tracing working
- (lsp) enable incremental text-sync


### Hdx_parser
- remove dbg! calls
- fix bugged contentsvalue
- ensure !important is output for properties including custom ones
- remove debugs
- clean up oxc_allocator in parser
- (parser) avoid positive peeks for unknown dimension units
- (parser) reflect changes to lexer
- guard last_cursor behind debug assertions


### Hdx_proc_macro
- rebaseline proc_macro_tests
- add missing snapshot
- (proc_macro) get alternatives with multi-comma variants generating
- (proc_macro) fixup #[initial] generation around smallvecs
- (proc_macro) update snapshots
- (proc_macro) relfect changes from parser/ast into proc_macro
- (proc_macro) update snapshots


### Hdx_syntax
- (syntax) add some more consts


### Hdx_transform
- (transform) comment out transforms for now
- (transform) tweak benchmark script to pass


### Hdx_vscode
- Update dependencies (#70) ([#70](https://github.com/csskit/csskit/pull/70))
- Update dependency mocha to v11 (#71) ([#71](https://github.com/csskit/csskit/pull/71))
- update package-lock files
- feat: mvp zed extension going (#75) ([#75](https://github.com/csskit/csskit/pull/75))
- (vscode) massively simplify extension
- chore(deps): update dependencies (#84) ([#84](https://github.com/csskit/csskit/pull/84))
- fix(deps): update dependencies (#91) ([#91](https://github.com/csskit/csskit/pull/91))


### Hdx_wasm
- (wasm) apply changes from parser/ast


### Hdx_writer
- restructure writer css values to match as file structure
- implement writer for LineWidth


### Hdx_zed
- fix(deps): update dependencies (#77) ([#77](https://github.com/csskit/csskit/pull/77))

