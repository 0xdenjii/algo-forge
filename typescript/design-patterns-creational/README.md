# Creational Design Patterns (TypeScript)

Each file is self-contained with a header (problem, approach, complexity) and a runnable demo (`npx tsx <file>`).

## Singleton / Multiton
- `singleton_lazy.ts` - Lazy singleton ConfigManager with private constructor.
- `singleton_eager.ts` - Eagerly created singleton Logger.
- `singleton_module_object.ts` - Idiomatic module-scoped singleton via frozen object literal.
- `singleton_async_init.ts` - Async singleton caching the init Promise (no double connection, retry on failure).
- `singleton_per_subclass.ts` - Inherited `instance()` giving one instance per subclass.
- `multiton_registry.ts` - One CurrencyFormatter per locale+currency key.

## Factory / Factory Method
- `simple_factory_shapes.ts` - Simple factory over a discriminated union with exhaustive check.
- `factory_method_logistics.ts` - Classic Factory Method: Road/Sea logistics create Truck/Ship.
- `factory_method_document_app.ts` - Application framework with overridable createDocument hook.
- `registry_factory_plugins.ts` - Open/closed parser factory backed by a runtime registry.
- `factory_notification_channels.ts` - LLD exercise: Email/SMS/Push notifiers with validation.
- `factory_generic_constructor.ts` - Type-safe generic `create(Ctor, ...args)` and counting factory.
- `static_factory_methods.ts` - Named constructors (fromHex/fromRgb) with validation and interning.
- `factory_ast_nodes_interning.ts` - Node factory with literal interning inside an expression parser/evaluator.

## Abstract Factory
- `abstract_factory_gui.ts` - Cross-platform Button/Checkbox families (Win/Mac).
- `abstract_factory_furniture.ts` - Victorian/Modern furniture sets with pricing.
- `abstract_factory_db_drivers.ts` - Dialect-consistent Connection + QueryBuilder (Postgres/MySQL).
- `abstract_factory_theme_kits.ts` - Light/Dark UI component kits built from palettes.
- `abstract_factory_with_builder_cars.ts` - Regional parts factory combined with a car builder.

## Builder
- `builder_fluent_http_request.ts` - Fluent, validated, frozen HTTP request builder.
- `builder_director_pizza.ts` - Builder plus Director encoding standard recipes.
- `builder_step_typed.ts` - Compile-time enforced step builder for emails.
- `builder_sql_query.ts` - Parameterised, injection-safe SELECT builder.
- `builder_immutable_with.ts` - Immutable value object with copy-on-write `withX` methods.
- `builder_html_tree.ts` - Nested HTML tree builder with escaping.
- `builder_test_data.ts` - Test Data Builder with defaults and derived totals.
- `builder_url.ts` - URL builder with per-component encoding and default-port elision.

## Prototype
- `prototype_clone_shapes.ts` - Polymorphic deep clone of shape hierarchy.
- `prototype_registry.ts` - Enemy spawner from template registry using structuredClone.
- `prototype_object_create.ts` - Native JS prototypal creation via Object.create.
- `prototype_document_templates.ts` - Class-preserving clone of invoices with Date/Map fields.

## Object Pool / Lazy / DI
- `object_pool_connections.ts` - Bounded async connection pool with FIFO waiters.
- `object_pool_particles.ts` - Preallocated particle pool with free-list (GC-friendly).
- `lazy_initialization_holder.ts` - Generic `Lazy<T>` with memoization and retry on failure.
- `dependency_injection_container.ts` - Token-based DI container with lifetimes and cycle detection.
