pull_font:
    mkdir -p resources
    wget http://www.figlet.org/fonts/chunky.flf -O resources/chunky.flf

make_logo: pull_font
    toilet -f resources/chunky.flf "nothingverse" --html > templates/logo.html
