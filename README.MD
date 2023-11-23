## srun-cli

Yet another SRUN client written in Rust.

## Usage

```bash
SRUN Commandline Client

Usage: srun-cli.exe [OPTIONS] [COMMAND]

Commands:
  query   Query account status
  login   Log in to campus network
  logout  Log out from campus network
  help    Print this message or the help of the given subcommand(s)

Options:
  -u, --username <username>            Username for the client
  -p, --password <password>            Password for the client
  -s, --server <server>                Server url
  -r, --redirect                       Trigger login by accessing some site (may be required under spefific network)
      --redirect-host <redirect-host>  Site to trigger redirection [default: http://www.google.cn/generate_204]
  -c, --config <config>                Configuration file
  -i, --interface <interface>          Network interface to use
  -o, --output <output>                Output format
  -h, --help                           Print help
  -V, --version                        Print version
```

## LICENSE

This software is licensed under the MIT license. See [LICENSE](LICENSE) for more details.