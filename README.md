# Aptos SDK

![license](https://img.shields.io/github/license/nkypy/aptos-client?style=flat-square)

working in progress

|        | sync | async |
| ------ | ---- | ----- |
| native | O    | ?     |
| wasm32 | X    | ?     |

## Usage

```yaml
# Cargo.toml

aptos-client = { git = "https://github.com/nkypy/aptos-client" }
```
```bash
# for wasm32
brew install emscripten
export CC=emcc
export AR=emar
```

## Functions

- [x] account_resource
- [x] table_item
- [x] create_single_signer_bcs_transaction
- [x] submit_bcs_transaction
- [x] wait_for_transaction
- [x] account_balance
- [x] collection
- [x] create_collection
- [x] token
- [x] create_token
- [x] token_balance
- [x] token_data
- [x] offer_token
- [x] claim_token
- [x] list_account_token_data
