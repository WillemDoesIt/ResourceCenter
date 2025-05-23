# Resource Center
This is the source code to a [Resource Center](https://willemdoesit.github.io/ResourceCenter/) webpage. The page acts as a search engine to Organizations and Services in the Spokane Area provided by Spokane World Relief.
For more information, check out the documentation directory to explain the "What?"s and "Why?"s of the project, specifically starting at [documentation/Overview.md](https://github.com/WillemDoesIt/ResourceCenter/blob/main/documentation/Overview.md).

# Roadmap
MVP: A search box that queries a sqlite db that we made the schema of and then dynamically load bellow the organizations or services.
- [X] Setup minimal wasm project
    - [X] Setup deploy script for automation
- [X] Setup sample db file
- [X] Use wasm to do a proof of concept query
- [X] Create db file
    - [X] Create db schema
    - [X] Sample db of real data
    - [X] Finished version of db for proj
- [X] Turn searches into queries
- [X] Dynamically load content from query
- [ ] Toggle-able options to specify queries
- [X] Make website pretty


# Usage
## Linux, MacOS or WSL (Recommended)
[Install nix](https://nixos.org/download/) if you haven't already. Then run the following Bash:
```bash
# Download
git clone https://github.com/WillemDoesIt/ResourceCenter.git
cd ResourceCenter/

# Enter developer shell
nix develop

# Allow permission to execute
chmod +x deploy.sh

# Script to automate compilation and serving on localhost
./deploy.sh
```

> [!NOTE]
> Nix develop may take a **VERY** long time the first time entering the developer environment.
> This has yet to be tested on MacOS or WSL, but given it runs on linux, it should work.

## Windows
Install rust from [winget](https://winget.run/pkg/Rustlang/Rustup) or [rust-lang.org](https://www.rust-lang.org/tools/install) if you haven't already.
Then run this in GitBash:
```bash
# Download
git clone https://github.com/WillemDoesIt/ResourceCenter.git
cd ResourceCenter/

# Install dependencies 
rustup target add wasm32-unknown-unknown        # Adds ability to compile to wasm32-unknown-unknown
cargo install wasm-bindgen-cli                  # Install wasm-bindgen-cli so js bindings can be generated
cargo install miniserve                         # Installs miniserve for easy localhost serving

# Allow permission to execute
chmod +x deploy.sh

# Script to automate compilation and serving on localhost
./deploy.sh
```

> [!NOTE]
> If you install via winget you will need to restart your gitbash session before cargo will act as installed.
>
> All except for the last two lines can be run in Powershell.
>
> `./deploy.sh` may take a minute upon the first compilation.
> 
> If running `./deploy.sh` has trouble in compiling, there may be an issue with your rust install, check to see if you can make any rust project with `cargo init` and `cargo run` in GitBash or Powershell, if this gives a linking error, installing [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) should fix the problem.
