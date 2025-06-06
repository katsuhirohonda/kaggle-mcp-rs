# kaggle-mcp-rs

Kaggle MCP server implementation in Rust. This server provides access to the Kaggle API through the Model Context Protocol (MCP).

## Features

Currently implemented:
- ✅ Authentication with Kaggle API credentials

Planned:
- Competition tools (8 tools)
- Dataset tools (10 tools)
- Kernel tools (8 tools)
- Model tools (16 tools)
- Configuration tools (4 tools)

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