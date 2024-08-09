# Logit

A lightweight vector store. Work in progress.

## (Planned) Features of the logit Local Database

01. **Lightweight and Embedded**:

    - **Description**: A lightweight, disk-based storage engine that does not require a separate
      server process.
    - **Inspired by**: SQLite

02. **Efficient Storage for Logits**:

    - **Description**: Specialized in storing high-dimensional vector representations (logits) with
      optimized read/write performance.

03. **Basic CRUD Operations**:

    - **Create**: Insert logits and associated metadata.
    - **Read**: Query logits based on metadata, vector similarity, or ID.
    - **Update**: Modify existing logits and their metadata.
    - **Delete**: Remove logits from the database.

04. **Similarity Search**:

    - **Description**: Built-in functionality to search for vectors similar to a given query vector.
    - **Features**: Cosine similarity, Euclidean distance, and other similarity metrics.

05. **Indexing**:

    - **Description**: Efficient indexing mechanisms for fast retrieval of high-dimensional data.
    - **Features**: Support for KD-trees, Ball-trees, and approximate nearest neighbors (ANN)
      indexing techniques.

06. **Batch Operations**:

    - **Description**: Support for batch insertions and updates to improve performance.

07. **Metadata Storage**:

    - **Description**: Ability to store and query metadata associated with each logit, such as
      timestamps, source identifiers, or tags.

08. **Transaction Support**:

    - **Description**: ACID-compliant transactions to ensure data integrity.

09. **APIs for Common Languages**:

    - **Description**: Provide APIs for Python and potentially other languages to interact with the
      database.

10. **Data Export/Import**:

    - **Description**: Functions to export and import data in standard formats (e.g., JSON, CSV) for
      interoperability.

11. **Compression**:

    - **Description**: Support for compressing stored data to save disk space.

12. **Security**:

    - **Description**: Basic encryption support for data at rest and in transit.
