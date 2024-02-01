# semtex
semtex is semtantic search for your desktop. Fully offline and private. 

## Install
WARNING: semtex is alpha-quality software (on a good day). Any data stored may get lost or be corrupted.

- Download and run the [desktop application](https://github.com/scalar-dev/semtex/releases/download/v0.1.2/semtex_0.1.0_amd64.AppImage) (currently Linux only).

- Install the [browser extension](https://github.com/scalar-dev/semtex/releases/download/v0.1.2/semtex_browser_extension-0.1.0.zip). Currently, this is only supported on browsers which allow you to install unsigned extensions (e.g. Firefox Nightly).

## What is it?
- a rust-based backend to index text into a local vector index ([usearch](https://github.com/unum-cloud/usearch)) and database ([sqlite](http://sqlite.org/))
- a browser extension which tracks local browsing activity;
- a desktop application to search and manage your data.

## How does it work?
semtex's browser extension inspects the content of each page you visit and determines whether it appears to be article-like text. It uses Mozilla's [Readability](https://github.com/mozilla/readability) library to transform websites into plain text and sends these (via localhost) to the desktop application.

The desktop application receives these text snippets and:
 - stores them in a local `sqlite` database;
 - transforms them into a vector embedding representation (using [MiniLM-L12-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2));
 - writes the embeddings to a local vector database (`usearch`).

 The embedding model used has been chosen to work well without GPU acceleration.

To search your browing history, open up the desktop app and type a free-form semantic search query. This will similarly be converted to a text embedding and then used to search the local vector index and sqlite database.

## Credits
semtex is an open source project built by [Scalar](https://scalar.dev). We are experts in building LLM-powered data processing and search systems. Get in touch [hello@scalar.dev](mailto:hello@scalar.dev).