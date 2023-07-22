# looker: a simple and fast way to search your notes

Looker, a Rust-based CLI, uses the Term Frequency-Inverse Document Frequency (TF-IDF) algorithm to search for terms across your text documents in a given directory. By generating an inverted index, it helps you retrieve information quickly and efficiently.

## Installation

To install Looker, you first need to have Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone the Looker repository to your local machine using the following command in your terminal:

```bash
git clone https://github.com/your_username/looker.git
```

Navigate to the `looker` directory:

```bash
cd looker
```

Finally, build the project:

```bash
cargo build --release
```

You should now have the `looker` executable in the `target/release` directory.

## Setup

To configure Looker, the utility automatically generates a configuration file `.env` in a `looker-cli` subdirectory within your system's configuration directory. This file contains two key entries:

1. `INDEX_PATH`: The location where the `index.json` index file will be stored.
2. `PERSONAL_DATA`: The directory Looker will search and index.

By default, Looker will create these paths under the `looker-cli` directory. If you wish to specify different directories, you can edit the `.env` file and replace the paths next to `INDEX_PATH` and `PERSONAL_DATA`.

## Usage

Looker offers two main commands: `for` and `reindex`.

### The 'for' Command

The 'for' command facilitates searching for a specific term within the indexed documents. Use it as follows:

```bash
looker for "your_query"
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
looker reindex
```

## Tests

Looker comes with a suite of tests to ensure optimal functionality. These tests cover the 'search' and 'reindex' functions.

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

## High level index data structure

In the context of a search architecture, the primary goal is to find the most relevant documents for a given query. To achieve this, we need a way to quantify the relevance of each document in relation to the query. The search query logic in our architecture involves several components, including the inverted index, term frequency-inverse document frequency (TF-IDF), and cosine similarity.

**Inverted Index**: The inverted index is a data structure that maps terms (words) to the documents containing them. It allows us to efficiently find the documents that contain the terms in the query. For each term in the query, we look up the corresponding documents in the inverted index and retrieve a list of candidate documents that may be relevant.

**TF-IDF**: Term Frequency-Inverse Document Frequency (TF-IDF) is a numerical statistic that helps us determine the importance of a term in a document within a corpus. It takes into account both the term frequency (how often the term appears in a document) and the inverse document frequency (a measure of how common the term is across all documents). By using TF-IDF, we give more weight to the terms that are more specific to the document, making them more important in determining the document's relevance.

**Cosine Similarity:** In our search architecture, we use cosine similarity to calculate the similarity between the query and the candidate documents. The idea is to represent both the query and the documents as vectors in a multi-dimensional space, where each dimension corresponds to a term in the corpus. The cosine similarity measures the cosine of the angle between these vectors. If the angle is small (i.e., the cosine is close to 1), it means the vectors are similar, indicating that the document is relevant to the query. If the angle is large (i.e., the cosine is close to 0), it means the vectors are dissimilar, indicating that the document is less relevant to the query.

In summary, the search query logic in our architecture involves the following steps:

1. Tokenize and process the query.
2. Retrieve the candidate documents using the inverted index.
3. Calculate the TF-IDF for the query and the documents.
4. Compute the cosine similarity between the query and the documents using their TF-IDF representations.
5. Rank the documents based on their cosine similarity scores.
6. By following these steps, our search architecture can efficiently find and rank the most relevant documents for a given query.
