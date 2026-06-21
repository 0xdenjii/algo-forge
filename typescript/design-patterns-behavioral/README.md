# TypeScript - Behavioral Design Patterns

Each file is self-contained, with a header comment and a runnable demo. Run one with
`npx tsx <file>.ts` or `node --experimental-transform-types <file>.ts` (Node 22+).

| File | Pattern | Description |
|---|---|---|
| `0150_evaluate_reverse_polish_notation.ts` | Interpreter / Command | LC 150: evaluate RPN with an operator registry and a stack |
| `0173_binary_search_tree_iterator.ts` | Iterator | LC 173: in-order BST iterator with O(h) stack, also `Iterable` |
| `0251_flatten_2d_vector.ts` | Iterator | LC 251: iterate a jagged 2D array without copying it |
| `0284_peeking_iterator.ts` | Iterator / Decorator | LC 284: add `peek()` to any iterator; run-length demo |
| `0341_flatten_nested_list_iterator.ts` | Iterator | LC 341: lazy nested-list flattening, plus a generator version |
| `1472_design_browser_history.ts` | Memento | LC 1472: back/forward history in O(1) per operation |
| `chain_http_middleware.ts` | Chain of Responsibility | Koa-style async middleware composition with short-circuiting |
| `chain_of_responsibility_atm_dispenser.ts` | Chain of Responsibility | ATM note handlers with limited inventory and all-or-nothing commit |
| `chain_of_responsibility_support_tickets.ts` | Chain of Responsibility | Functional escalation chain: bot, tier1, engineer, manager |
| `command_job_queue_with_retry.ts` | Command | Async job queue with a concurrency limit and exponential-backoff retry |
| `command_smart_home_remote.ts` | Command | Remote slots, macro commands and an undo history |
| `command_text_editor_undo_redo.ts` | Command | Insert/delete commands with undo/redo stacks |
| `elevator_system_state_strategy.ts` | State + Strategy | Multi-elevator LLD: SCAN movement and nearest-car dispatch |
| `interpreter_boolean_rules.ts` | Interpreter | Parse and evaluate feature-flag rules with and/or/not and comparisons |
| `iterator_lazy_pipeline.ts` | Iterator | Lazy `Seq` with map/filter/take/chunk/zip over infinite sources |
| `mediator_air_traffic_control.ts` | Mediator | Tower assigns runways, with emergency-priority queueing |
| `mediator_chat_room.ts` | Mediator | Chat room routing broadcasts, DMs, mutes and join/leave notices |
| `memento_game_checkpoint.ts` | Memento | Immutable, opaque save slots for a game hero |
| `null_object_logger.ts` | Null Object | Do-nothing logger and discount defaults that remove null checks |
| `observer_reactive_signals.ts` | Observer | Signals, cached computeds and effects with dynamic dependency tracking |
| `observer_stock_ticker.ts` | Observer | Price ticker that notifies only on change, with unsubscribe |
| `observer_typed_event_emitter.ts` | Observer | Compile-time type-safe EventEmitter (on/once/off/emit) |
| `pubsub_topic_broker.ts` | Publish/Subscribe | Trie-based topic broker with `*` and `#` wildcards |
| `specification_product_filter.ts` | Specification | Composable and/or/not product filters with readable labels |
| `state_tcp_connection.ts` | State | GoF TCP connection; illegal operations throw per state |
| `state_traffic_light.ts` | State | Table-driven traffic light with timers, pedestrian button and fault mode |
| `state_vending_machine.ts` | State | Vending machine with Idle/HasMoney/SoldOut state classes |
| `strategy_payment_processing.ts` | Strategy | Card, PayPal and crypto payment strategies, swapped at runtime |
| `strategy_rate_limiter.ts` | Strategy | Token bucket, sliding log and fixed window limiters behind one interface |
| `strategy_shipping_cost.ts` | Strategy | Function strategies in a registry; picks the cheapest carrier |
| `template_method_board_game.ts` | Template Method | One game loop shared by Nim (optimal play) and Tic-Tac-Toe |
| `template_method_data_exporter.ts` | Template Method | CSV, JSON Lines and Markdown exporters sharing one export algorithm |
| `visitor_discriminated_union_shapes.ts` | Visitor (TS idiom) | Exhaustive `match` over a tagged union of shapes |
| `visitor_expression_tree.ts` | Visitor | Double dispatch over an AST: evaluate, print, differentiate |
