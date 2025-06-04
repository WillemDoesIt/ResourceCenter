# refrences:
# - https://www.geeksforgeeks.org/how-to-exit-when-errors-occur-in-bash-scripts/
# - https://www.squash.io/preventing-terminal-print-from-bash-scripts-in-linux/
# - https://linuxize.com/post/bash-functions/
# - https://unix.stackexchange.com/questions/298706/how-to-pass-parameters-to-function-in-a-bash-script
# - https://stackoverflow.com/questions/1298066/how-can-i-check-if-a-package-is-installed-and-install-it-if-not

function safe_runner() {
    $1 > /dev/null 2>&1
    if [ $? -ne 0 ]
    then
        echo -e "\033[0;31m[  Error  ]\033[0m $2"
        $1
        exit 1
    else
        echo -e "\033[0;32m[   OK    ]\033[0m $2"
    fi
}

safe_runner "cargo build --target wasm32-unknown-unknown" "Compiled to WASM"
# display warnings
cargo build --target wasm32-unknown-unknown -q

safe_runner "wasm-bindgen --target web --out-dir web/wasm ./target/wasm32-unknown-unknown/debug/ResourceCenter.wasm" "Created JS bindings"

safe_runner "miniserve --version" "Miniserve check"


echo -e "\033[0;32m[ Serving ]\033[0m On http://localhost:8080"
echo -e "\nPress Ctrl+C to stop the server"
miniserve web --index index.html --port 8080 > /dev/null 2>&1
