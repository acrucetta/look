

# Architecture

## Data ingestion module
- Read and process various file formats (e.g., plain text, markdown, PDF, Word, HTML)
- Perform text processing techniques, such as stemming or lemmatization, using an NLP library
- Indexer module

## Tokenize documents using more advanced text processing techniques
- Store the index on disk using a database or an inverted index
- Implement incremental indexing to update the index efficiently when files are added or modified
- Calculate and store ranking information, such as term frequency-inverse document frequency (TF-IDF)
- Query processing module

## Parse and tokenize search queries
- Retrieve search results from the index, considering the ranking information for relevance
- Handle exceptions and edge cases during the search process
- CLI query interface

## Provide a user-friendly interface to input search queries
- Display search results, sorted by relevance
- Offer helpful feedback and error messages
