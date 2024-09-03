# Webhook Service

This Rust project is a command-line tool designed to send messages to Slack channels. The tool is structured for easy customization and scalability, making it suitable for interacting with Slack's webhook API.

## Features

- Flexible command-line interface for sending messages to Slack channels.
- Supports customizable logging levels (debug, info, warn, error, off).
- Environment variable support for storing sensitive information like Slack webhook URLs.
- Asynchronous HTTP requests with detailed logging for development and debugging.

## Prerequisites

Before running the tool, ensure you have the following:

- Rust installed on your system.
- A `.env` file containing your Slack webhook URLs.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/slack_tools.git
    cd slack_tools/webhook_service/
    ```

2. **Build the Project**:
    Make sure you have Rust installed on your machine. You can build the project with Cargo:
    ```bash
    cargo build --release
    ```

3. **Set Up Environment Variables**:
    Create a `.env` file in the root of the project and add your Slack webhook URLs. Example:
    ```env
    CHANNEL_NAME=<your-slack-webhook-url>
    ```

## Usage

To run the tool and send a message to a Slack channel, use the following command:

```sh
./target/release/slack_messenger --message "Your message" --channel "CHANNEL_NAME"
```

### Options

The tool supports different logging levels, which can be controlled with command-line flags:

- `--debug`: Enables debug-level logging.
- `--info`: Enables info-level logging (default).
- `--warn`: Enables warn-level logging.
- `--error`: Enables error-level logging.
- `--off`: Disables logging.

Example with debug logging:

```sh
./target/release/template_cli --debug --message "Your message" --channel "your-channel"
```

## Logging

The tool uses the `env_logger` crate for logging. By default, the log level is set to `info`, but you can change it using command-line options or by setting the `RUST_LOG` environment variable:

```sh
RUST_LOG=debug ./target/release/template_cli --message "Your message" --channel "your-channel"
```

This allows for more granular control over the logging output, which can be useful for debugging or monitoring.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/slack_tools/blob/main/LICENSE) file for details.
