# Structural Design Patterns (TypeScript)

These are standalone TypeScript files. Each one runs with `npx tsx <file>.ts` (or `ts-node`), and each has a demo at the bottom.

## Adapter
- `adapter_legacy_printer.ts`: an object adapter that wraps a legacy line printer behind a `Printer` interface.
- `adapter_payment_gateway.ts`: puts two incompatible payment SDKs behind one async `PaymentProcessor`.
- `adapter_temperature_sensor.ts`: shows an object adapter and a class adapter for a Fahrenheit sensor feeding a Celsius thermostat.
- `adapter_callback_to_promise.ts`: a type-safe `promisify` that adapts Node-style callback APIs to Promises.
- `0251_flatten_2d_vector.ts`: LeetCode 251. Adapts a 2D array to a flat iterator, including `Symbol.iterator`.

## Bridge
- `bridge_shapes_renderers.ts`: shapes × renderers (SVG and ASCII), so you need M+N classes instead of M×N.
- `bridge_remote_devices.ts`: basic and advanced remotes that drive TV and radio devices independently.
- `bridge_notification_channels.ts`: alert and digest notifications sent over email, SMS (with the 160-char limit) or Slack.

## Composite
- `composite_file_system.ts`: a file/directory tree with total size, find-by-predicate and a tree printout.
- `composite_organization_chart.ts`: an org chart that computes cost, headcount and depth for managers and their reports.
- `composite_ui_components.ts`: labels and buttons inside VBox/HBox containers, with measuring and text rendering.
- `1628_design_expression_tree.ts`: LeetCode 1628. Builds an expression tree from postfix tokens and evaluates it.
- `0341_flatten_nested_list_iterator.ts`: LeetCode 341. A stack-based iterator over a nested-integer composite.
- `0339_nested_list_weight_sum.ts`: LeetCode 339 and 364. Depth-weighted and inverse-depth-weighted sums.
- `0385_mini_parser.ts`: LeetCode 385. Parses a nested-list string into a composite.

## Decorator
- `decorator_coffee_shop.ts`: the classic Starbuzz example, with condiment decorators and prices kept in cents.
- `decorator_data_stream.ts`: stackable run-length compression and XOR/base64 encryption around a data source.
- `decorator_function_wrappers.ts`: typed higher-order wrappers for memoize, timing and logging, plus `compose`.
- `decorator_retry_http_client.ts`: stackable auth, timeout and retry-with-backoff layers for an HTTP client.
- `0284_peeking_iterator.ts`: LeetCode 284. Adds `peek()` to any iterator.

## Facade
- `facade_home_theater.ts`: one call to start a movie and one to end it, across five subsystems.
- `facade_compiler_pipeline.ts`: lexer, recursive-descent parser, codegen and stack VM behind `Compiler.run`.
- `facade_order_checkout.ts`: the inventory, pricing, payment and shipping steps of a checkout, with rollback when payment fails.

## Flyweight
- `flyweight_text_glyphs.ts`: shared glyph objects for a text editor. A 12k-char document uses 12 flyweights.
- `flyweight_forest_trees.ts`: shared tree types plus per-tree data in typed arrays for 100k trees.
- `flyweight_string_interning.ts`: interns `Money` values using WeakRef and FinalizationRegistry, so equal values are the same object (`===` holds).

## Proxy
- `proxy_virtual_image.ts`: a virtual proxy that loads images lazily and shares a load that is already in progress.
- `proxy_protection_access.ts`: a role-based protection proxy that keeps an audit trail.
- `proxy_caching_lru.ts`: a caching proxy with LRU eviction and a TTL. The LRU part is LeetCode 146.
- `proxy_es_validation_observable.ts`: a native ES `Proxy` that validates against a schema and notifies on nested changes.
- `proxy_rate_limiter.ts`: a rate-limiting proxy using one token bucket per client.

## Related structural idioms
- `private_class_data.ts`: an account with write-once data in a frozen `#private` object and a balance changed only through guarded methods.
- `mixins_composition.ts`: typed mixins (Timestamped, SoftDeletable, Emitter) combined per entity.
