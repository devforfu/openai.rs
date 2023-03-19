# OpenAI API Client Generator


Un-official toolkit to generating OpenAI API client using [OpenAPI specification](https://github.com/openai/openai-openapi/blob/master/openapi.yaml) and slightly modified 
[Rust OpenAPI Templates](https://github.com/OpenAPITools/openapi-generator/tree/master/modules/openapi-generator/src/main/resources/rust).

## Steps to build a client

1. Retrieve the most recent OpenAPI specification.
```bash
pushd openapi_definition
cargo run
popd
```
2. Generate the client using `openapi-generator`.
```bash
openapi-generator-cli generate -g rust \
  -i openapi_definition/config/openapi_sanitized.yaml \
  -o rust_client \
  -t rust_templates \
  -c config.yaml
```
3. Check if it works.
```bash
pushd openai
OPENAI_API_KEY=<your_key> cargo run
popd
```
A possible output (truncated):
```
Model { id: "babbage", object: "model", created: 1649358449, owned_by: "openai" }
Model { id: "davinci", object: "model", created: 1649359874, owned_by: "openai" }
Model { id: "babbage-code-search-code", object: "model", created: 1651172509, owned_by: "openai-dev" }
...
```
