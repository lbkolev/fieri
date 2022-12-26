# <p align="center">Unofficial Rust client for the OpenAI API</p>

## Holds all necessary resources for direct interaction with the endpoints.

- [x]   `/models`
    - [x] [list](https://beta.openai.com/docs/api-reference/models/list)
    - [x] [retrieve](https://beta.openai.com/docs/api-reference/models/retrieve)
- [x]   `/completions`
    - [x] [create](https://beta.openai.com/docs/api-reference/completions/create)
    - [ ] [create with stream](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stream)
- [x]   `/edits`
    - [x] [create](https://beta.openai.com/docs/api-reference/edits/create)
- [ ]   `/images`
    - [x] [create image](https://beta.openai.com/docs/api-reference/images/create)
    - [ ] [create image edit](https://beta.openai.com/docs/api-reference/images/create-edit)
    - [ ] [create image variation](https://beta.openai.com/docs/api-reference/images/create-variation)
- [x]   `/embeddings`
    - [x] [create](https://beta.openai.com/docs/api-reference/embeddings/create)
- [ ]   `/files`
    - [x] [list](https://beta.openai.com/docs/api-reference/files/list)
    - [x] [upload](https://beta.openai.com/docs/api-reference/files/upload)
    - [x] [delete](https://beta.openai.com/docs/api-reference/files/delete)
    - [x] [retrieve](https://beta.openai.com/docs/api-reference/files/retrieve)
    - [ ] [retrieve content](https://beta.openai.com/docs/api-reference/files/retrieve-content)
- [ ]   `/fine-tunes`
    - [ ] [create](https://beta.openai.com/docs/api-reference/fine-tunes/create)
    - [ ] [list](https://beta.openai.com/docs/api-reference/fine-tunes/list)
    - [ ] [retrieve](https://beta.openai.com/docs/api-reference/fine-tunes/retrieve)
    - [ ] [cancel](https://beta.openai.com/docs/api-reference/fine-tunes/cancel)
    - [ ] [list events](https://beta.openai.com/docs/api-reference/fine-tunes/events)
    - [ ] [list events with stream](https://beta.openai.com/docs/api-reference/fine-tunes/events#fine-tunes/events-stream)
    - [x] [delete](https://beta.openai.com/docs/api-reference/fine-tunes/delete-model)
- [x]   `/moderations`
    - [x] [create](https://beta.openai.com/docs/api-reference/moderations/create)
- [ ]   `/engines` - *DEPRECATED*
