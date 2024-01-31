# semtex

## What is it?
semtex is a set of tools for locally indexing and semantically searching
your browser history. It comprises:
- a rust-based backend to index text into a vector index ([usearch](https://github.com/unum-cloud/usearch))
- a browser extension which tracks local browsing activity;
- and a desktop application to search and managed your data.

## How does it work?
semtex's browser extension inspects the content of each page you visit (entirely offline) and determines whether it appears to be article-like text. It uses Mozilla's [Readability](https://github.com/mozilla/readability) to transform websites into a readable form and sends these (via localhost) to the desktop app.

The desktop application receives these text snippets and stores them in a local `sqlite` database before transforming them into a vector embedding representation (using [MiniLM-L12-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2)). These are then written to a local vector index.

To search your browing history, open up the desktop app and type a free-form semantic search query. This will similarly be converted to a text embedding and then used to search the local vector index and sqlite database.

