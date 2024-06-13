root := justfile_directory()

app-root := root / 'app'

paper-root := root / 'paper'
paper-src := paper-root / 'main.typ'
paper-out := root / 'out' / 'detypstify-ocr-for-formula-generation.pdf'

export TYPST_ROOT := paper-root

[private]
default:
        @just --list --unsorted

[doc('Run all formatters')]
fmt: app-format paper-format 
        nix fmt        

[private]
cd-app-root:
        cd {{ app-root }}
        
[group('app')]
app-release: cd-app-root
        dx build --release

[group('app')]
app-watch: cd-app-root
        dx serve --hot-reload

[group('app')]
app-format: cd-app-root
        dx fmt 

[group('scraper')]
scraper-run:
        cargo run -p scraper
        
[group('scraper')]
scraper-build:
        cargo build -p scraper

[group('training')]
train:
        poetry run train

[group('training')]
train_val_split:
        poetry run train_val_split

[group('training')]
symbols2svg:
        poetry run symbols2svg
        
[group('paper')]
paper-preview:
        typst-preview --root {{ paper-root }} {{ paper-src }}

[group('paper')]
paper-build: 
        typst compile {{ paper-src }} {{ paper-out }}

[group('paper')]
paper-watch: 
        typst watch {{ paper-src }} {{ paper-out }}

[group('paper')]          
paper-format: 
        typstyle -i format-all 
