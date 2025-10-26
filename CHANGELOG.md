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

