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

## Search Architecture

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
