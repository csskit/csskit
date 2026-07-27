fn main() {
	// Only run the napi codegen/link setup when building the native addon; the default (rlib/core)
	// build must not pull Node symbols so the workspace links without a Node runtime present.
	if std::env::var_os("CARGO_FEATURE_NAPI").is_some() {
		napi_build::setup();
	}
}
