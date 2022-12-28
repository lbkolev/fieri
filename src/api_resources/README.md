# <p align="center">Unofficial Rust client for the OpenAI API</p>

## Holds all necessary resources for direct interaction with the endpoints.

## Implemented endpoints:

### `models`
- [x] [list](https://beta.openai.com/docs/api-reference/models/list)
- [x] [retrieve](https://beta.openai.com/docs/api-reference/models/retrieve)

### `completions`
- [x] [create](https://beta.openai.com/docs/api-reference/completions/create)
- [ ] [create with stream](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stream)

### `edits`
- [x] [create](https://beta.openai.com/docs/api-reference/edits/create)

### `images`
- [x] [create image](https://beta.openai.com/docs/api-reference/images/create)
- [x] [create image edit](https://beta.openai.com/docs/api-reference/images/create-edit)
- [x] [create image variation](https://beta.openai.com/docs/api-reference/images/create-variation)

### `embeddings`
- [x] [create](https://beta.openai.com/docs/api-reference/embeddings/create)

### `files`
- [x] [list](https://beta.openai.com/docs/api-reference/files/list)
- [x] [upload](https://beta.openai.com/docs/api-reference/files/upload)
- [x] [delete](https://beta.openai.com/docs/api-reference/files/delete)
- [x] [retrieve](https://beta.openai.com/docs/api-reference/files/retrieve)
- [ ] [retrieve content](https://beta.openai.com/docs/api-reference/files/retrieve-content)

### `fine-tunes`
- [x] [create](https://beta.openai.com/docs/api-reference/fine-tunes/create)
- [x] [list](https://beta.openai.com/docs/api-reference/fine-tunes/list)
- [x] [retrieve](https://beta.openai.com/docs/api-reference/fine-tunes/retrieve)
- [x] [cancel](https://beta.openai.com/docs/api-reference/fine-tunes/cancel)
- [x] [list events](https://beta.openai.com/docs/api-reference/fine-tunes/events)
- [ ] [list events with stream](https://beta.openai.com/docs/api-reference/fine-tunes/events#fine-tunes/events-stream)
- [x] [delete](https://beta.openai.com/docs/api-reference/fine-tunes/delete-model)

### `moderations`
- [x] [create](https://beta.openai.com/docs/api-reference/moderations/create)

### `/engines` - *DEPRECATED*
