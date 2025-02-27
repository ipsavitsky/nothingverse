default: make_logo make_database

make_database:
    scripts/init_db_if_missing.sh

make_logo: pull_font
    toilet -f resources/chunky.flf "nothingverse" --html > templates/logo.html

pull_font:
    mkdir -p resources
    wget http://www.figlet.org/fonts/chunky.flf -O resources/chunky.flf

watch:
    watchexec -r -- cargo run
