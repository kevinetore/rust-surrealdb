# Examples

A project to learn about Rust using (Surrealdb)[https://surrealdb.com/] and Warp.

## Run the project

To get started, run ` cargo watch -q -c -x "run -q"` 

This will start a web server running on your localhost port 8080.

Open another terminal and run:

### Get all available books

```bash
> curl http://localhost:8080/books
[]%
```

### Create a book
```bash
> curl -o /dev/null -s -w "%{http_code}\n" --location --request GET 'http://localhost:8080/books' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "The Rust programming language",
    "author": "Steve Klabnik, Carol Nichols",
    "num_pages": 527,
    "tags": ["Ferris", "Rust"]
}'
201
```

### List a book
```bash
> curl --location --request GET 'http://localhost:8080/books/ukonjk599zx0pfvwp868'
{
    "author": "Steve Klabnik, Carol Nichols",
    "created_at": "2023-08-17 19:11:02.309722 UTC",
    "id": "book:ukonjk599zx0pfvwp868",
    "name": "The Rust programming language",
    "num_pages": "527",
    "tags": [
        "Ferris",
        "Rust"
    ]
}
```

### Delete a book
```bash
> curl --location --request DELETE 'http://localhost:8080/books/ukonjk599zx0pfvwp868'
200
```


