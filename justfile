default: make_database make_model

pull_npm:
    npm install

make_database: make_styles pull_npm make_logo
    scripts/init_db_if_missing.sh

make_logo: pull_font
    toilet -f resources/chunky.flf "nothingverse" --html | htmlq 'body' > templates/logo.html

pull_font:
    mkdir -p resources
    wget http://www.figlet.org/fonts/chunky.flf -O resources/chunky.flf

make_model:
    ollama create nothing -f ./ollama/nothing.modelfile

make_styles:
    tailwindcss -i ./templates/styles-in.css -o ./templates/styles.css

watch:
    watchexec -r -- "just make_styles; cargo run -- --log-level debug"

lint: make_database
    -nix flake check
    -statix check .
    -cargo clippy
