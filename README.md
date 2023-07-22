# look: a simple and fast way to search your notes

look, a Rust-based CLI, uses the Term Frequency-Inverse Document Frequency (TF-IDF) algorithm to search for terms across your text documents in a given directory. By generating an inverted index, it helps you retrieve information quickly and efficiently.

## Installation

To install look, you first need to have Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone the look repository to your local machine using the following command in your terminal:

```bash
git clone https://github.com/acrucetta/look.git
```

Navigate to the `look` directory:

```bash
cd look
```

Finally, build the project:

```bash
cargo build --release
```

You should now have the `look` executable in the `target/release` directory.

## Setup

To configure look, the utility automatically generates a configuration file `.env` in a `look-cli` subdirectory within your system's configuration directory. This file contains two key entries:

1. `INDEX_PATH`: The location where the `index.json` index file will be stored.
2. `PERSONAL_DATA`: The directory look will search and index.

By default, look will create these paths under the `look-cli` directory. If you wish to specify different directories, you can edit the `.env` file and replace the paths next to `INDEX_PATH` and `PERSONAL_DATA`.

## Usage

look offers two main commands: `for` and `reindex`.

### The 'for' Command

The 'for' command facilitates searching for a specific term within the indexed documents. Use it as follows:

```bash
look for "your_query"
```

Replace `"your_query"` with the term you're searching for.

```
/Users/your_user/example.txt [0.11]
259:         - doing more mobile testing
266:         - designing, building, and testing new features from ideation to deployment.
360:         - manually test or not test at all; build unit tests to check for the performance
362:         - use react; jasmine; test runner
```

The output is ranked by the most common terms. The number in [brackets] represents the TF-IDF score for the term in that document. The higher the score, the more relevant the document is to the search term.

The output was inspired by ripgrep. I wanted to make it easy to see the context of the search term in the document. The line number is followed by the line itself.

### The 'reindex' Command

You can use the 'reindex' command to re-index a directory. This is particularly useful when you have added new files or updated existing ones. Use it as follows:

```bash
look reindex
```

## Search architecture details

### Data ingestion module

- Read and process various file formats (e.g., plain text, markdown, PDF, Word, HTML)
- Perform text processing techniques, such as stemming or lemmatization, using an NLP library
- Indexer module

### Tokenize documents using more advanced text processing techniques

- Store the index on disk using a database or an inverted index
- Implement incremental indexing to update the index efficiently when files are added or modified
- Calculate and store ranking information, such as term frequency-inverse document frequency (TF-IDF)
- Query processing module

### Parse and tokenize search queries

- Retrieve search results from the index, considering the ranking information for relevance
- Handle exceptions and edge cases during the search process
- CLI query interface

### Provide a user-friendly interface to input search queries

- Display search results, sorted by relevance
- Offer helpful feedback and error messages
