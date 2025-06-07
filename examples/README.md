# kaggle-mcp-rs Examples

This directory contains examples demonstrating how to use the kaggle-mcp-rs library.

## Running Examples

Make sure you have your Kaggle API credentials set up first:

1. Create a Kaggle account at https://www.kaggle.com
2. Go to Account settings and create an API token
3. Save the credentials either as:
   - Environment variables: `KAGGLE_USERNAME` and `KAGGLE_KEY`
   - Or in `~/.kaggle/kaggle.json`

## Available Examples

### competitions_list

Demonstrates how to list and search Kaggle competitions with various filters.

```bash
cargo run --example competitions_list
```

Features shown:
- Basic authentication
- Listing all competitions
- Searching competitions by keyword
- Filtering by category (featured, research, etc.)
- Sorting by different criteria (deadline, prize, etc.)
- Pagination support

## Adding New Examples

When adding new examples:
1. Create a new `.rs` file in this directory
2. Add an `[[example]]` entry to `Cargo.toml`
3. Document the example in this README