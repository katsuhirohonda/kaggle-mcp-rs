# kaggle-mcp-rs

A Rust implementation of the Kaggle MCP (Model Context Protocol) server that enables Claude AI and other MCP-compatible clients to interact with the Kaggle API.

## Overview

This crate provides a complete MCP server implementation for Kaggle API integration, allowing AI assistants to:
- Authenticate with Kaggle credentials
- Browse and download competition data
- Search and manage datasets
- Access kernels (notebooks) and models
- Configure API settings

## Features

### Currently Implemented
- ✅ **Authentication**: Secure authentication with Kaggle API credentials
  - Environment variable support
  - Credential file management (~/.kaggle/kaggle.json)
  - Automatic credential loading

### Planned Features
- 📋 **Competitions** (8 tools): List, search, download data, submit predictions
- 📊 **Datasets** (10 tools): Search, download, create, update datasets
- 📓 **Kernels** (8 tools): Search, pull, push notebooks
- 🤖 **Models** (16 tools): List, download, manage ML models
- ⚙️ **Configuration** (4 tools): Manage API settings and preferences

## Installation

```bash
cargo build --release
```

## Usage

### Claude Desktop Configuration

Add the following to your Claude Desktop configuration file:

```json
{
  "mcpServers": {
    "kaggle-rs": {
      "command": "/path/to/kaggle-mcp-rs/target/release/kaggle-mcp-rs"
    }
  }
}
```

### Kaggle API Credentials

Before using the server, you need to set up your Kaggle API credentials:

1. Go to your [Kaggle account settings](https://www.kaggle.com/settings/account)
2. In the API section, click "Create New API Token"
3. This will download a `kaggle.json` file
4. Move this file to `~/.kaggle/kaggle.json`
5. Set the correct permissions: `chmod 600 ~/.kaggle/kaggle.json`

Alternatively, you can authenticate directly through Claude using the `authenticate` tool.

## Available Tools

### Authentication

- `authenticate`: Authenticate with the Kaggle API using your username and API key
  - Parameters:
    - `kaggle_username`: Your Kaggle username
    - `kaggle_key`: Your Kaggle API key

## Development

This project uses the [rmcp](https://github.com/modelcontextprotocol/rust-sdk) Rust SDK for MCP.

### Building

```bash
cargo build
```

### Running locally

```bash
cargo run
```

## License

MIT