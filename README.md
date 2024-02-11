# Proof-of-concept for using TypeScript type checker from Rust

https://github.com/oxc-project/oxc/issues/2218

Proof-of-concept implementation of Rust <--> TSServer communication for `no-floating-promises` ESLint rule.
Type checker is only needed as the last step to check the type of `CallExpression`.

The way the POC works, is it copies typecheck helper implementation for `isPromiseLike` from ESLint, and exposes that as a command in `tsserver` style protocol.
To actually implement the rule, we would traverse the Rust AST until we reach expression we need to check.
And then pass the location and type of the AST node to `isPromiseLike` command to do the type check on the JS side.
This node mapping can probably be optimized to just child index access on the JS side.

## How to run this?

```bash
# Install dependencies
pnpm i

# Build Rust side
cargo build

# Build JS side
pnpm build

# Run the demo
cargo run
```

The output should look something like this:
```
Working Dir: /source/js-tools/tsserver-client
{"seq":0,"type":"response","command":"status","request_seq":1,"success":true,"body":{"version":"0.1.0"}}
Sample file: /source/js-tools/tsserver-client/./src_js/sample.ts
{"seq":0,"type":"response","command":"getNode","request_seq":3,"success":true,"body":{"kind":"CallExpression","text":"func()","type":"number"}}
{"seq":0,"type":"response","command":"getNode","request_seq":4,"success":true,"body":{"kind":"CallExpression","text":"funcAsync()","type":"Promise<void>","symbol":"Promise"}}
{"seq":0,"type":"response","command":"noFloatingPromises::isPromiseLike","request_seq":5,"success":true,"body":{"result":false}}
{"seq":0,"type":"response","command":"noFloatingPromises::isPromiseLike","request_seq":6,"success":true,"body":{"result":true}}
Server exited with exit code: 0
```
