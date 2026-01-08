#[test]
fn platform_custom_time_callback() {
  extern "C" fn mock_time_callback() -> f64 {
    1767865796917.0 // 2026/01/08 18:49:56
  }

  let platform =
    v8::new_default_platform_with_time_callback(0, false, mock_time_callback);

  v8::V8::set_flags_from_string("--harmony-temporal");
  v8::V8::initialize_platform(platform.into());
  v8::V8::initialize();
  let isolate = &mut v8::Isolate::new(Default::default());
  v8::scope!(let scope, isolate);

  let context = v8::Context::new(scope, Default::default());
  let scope = &mut v8::ContextScope::new(scope, context);
  let source = r#"
    function assertEquals(a, b) {
      if (a === b) return;
      throw a + " does not equal " + b;
    }

    const now = Temporal.Now.instant().epochMilliseconds;

    assertEquals(now, 1767865796917);
  "#;
  let source = v8::String::new(scope, source).unwrap();
  let script = v8::Script::compile(scope, source, None).unwrap();
  script.run(scope).unwrap();
}
