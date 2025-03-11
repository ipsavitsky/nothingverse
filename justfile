default: make_database make_model

make_database: make_styles pull_htmx make_logo
    scripts/init_db_if_missing.sh

make_logo: pull_font
    toilet -f resources/chunky.flf "nothingverse" --html > templates/logo.html

pull_font:
    mkdir -p resources
    wget http://www.figlet.org/fonts/chunky.flf -O resources/chunky.flf

make_model:
    ollama create nothing -f ./ollama/nothing.modelfile

make_styles:
    tailwindcss -i ./templates/styles-in.css -o ./templates/styles.css

pull_htmx:
     mkdir -p templates/assets
     wget https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js -O ./templates/assets/htmx.min.js
     wget https://unpkg.com/htmx-ext-sse@2.2.3/dist/sse.min.js -O ./templates/assets/sse.min.js

watch:
    watchexec -r -- "just make_styles; cargo run"
