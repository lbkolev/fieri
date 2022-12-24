# <p align="center">Unofficial Rust client for the OpenAI API</p>

<p align="center">
    <a href="https://github.com/lbkolev/openai-rs/blob/master/LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://github.com/lbkolev/openai-rs/actions?query=workflow%3ACI+branch%3Amaster">
        <img src="https://github.com/lbkolev/openai-rs/actions/workflows/ci.yml/badge.svg">
    </a>
</p>

### Soon™️

### Endpoints:

- [x]   `/models`
    - [x] [list](https://beta.openai.com/docs/api-reference/models/list)
    - [x] [retrieve](https://beta.openai.com/docs/api-reference/models/retrieve)
- [x]   `/completions`
    - [x] [create](https://beta.openai.com/docs/api-reference/completions/create)
    - [ ] [create with stream](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stream)
- [x]   `/edits`
    - [x] [create](https://beta.openai.com/docs/api-reference/edits/create)
- [ ]   `/images`
    - [ ] [create image](https://beta.openai.com/docs/api-reference/images/create)
    - [ ] [create image edit](https://beta.openai.com/docs/api-reference/images/create-edit)
    - [ ] [create image variation](https://beta.openai.com/docs/api-reference/images/create-variation)
- [x]   `/embeddings`
    - [ ] [create](https://beta.openai.com/docs/api-reference/embeddings/create)
- [ ]   `/files`
    - [x] [list](https://beta.openai.com/docs/api-reference/files/list)
    - [ ] [upload](https://beta.openai.com/docs/api-reference/files/upload)
    - [ ] [delete](https://beta.openai.com/docs/api-reference/files/delete)
    - [ ] [retrieve](https://beta.openai.com/docs/api-reference/files/retrieve)
    - [ ] [retrieve content](https://beta.openai.com/docs/api-reference/files/retrieve-content)
- [ ]   `/fine-tunes`
    - [ ] [create](https://beta.openai.com/docs/api-reference/fine-tunes/create)
    - [ ] [list](https://beta.openai.com/docs/api-reference/fine-tunes/list)
    - [ ] [retrieve](https://beta.openai.com/docs/api-reference/fine-tunes/retrieve)
    - [ ] [cancel](https://beta.openai.com/docs/api-reference/fine-tunes/cancel)
    - [ ] [list events](https://beta.openai.com/docs/api-reference/fine-tunes/events)
    - [ ] [delete](https://beta.openai.com/docs/api-reference/fine-tunes/delete-model)
- [x]   `/moderations`
    - [x] [create](https://beta.openai.com/docs/api-reference/moderations/create)
- [-]   `/engines` - *DEPRECATED*
