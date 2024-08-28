# SM.MS Tools

A lightweight [SM.MS](https://sm.ms/) tools, no external dependencies, no interface, support for multiple image uploads
and can work with typora.

## Features

- Modern cli build by `clap`
- Asynchronous multi-file upload and delete by `tokio`
- Get your profile (disk usage, disk limit, etc...)
- Get the list of upload history

## Installation

### From Source

```bash
git clone https://github.com/c3b2a7/smmstools.git
cargo install --path smmstools
```

### Manual

You can also download and extract the release from https://github.com/c3b2a7/smmstools/releases


## Usage

```bash
A lightweight SM.MS (https://sm.ms/) tools

Usage: smmstools.exe --token <SMMS_TOKEN> <COMMAND>

Commands:
  profile  Get user profile
  upload   Upload image(s) to sm.ms
  delete   Delete image(s)
  history  Get upload history
  help     Print this message or the help of the given subcommand(s)

Options:
  -t, --token <SMMS_TOKEN>  API token of sm.ms, visit https://sm.ms/home/apitoken to get your token [env: SMMS_TOKEN=]
  -h, --help                Print help
  -V, --version             Print version
```

## License

MIT

