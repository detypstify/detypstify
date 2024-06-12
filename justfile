root := justfile_directory()

paper-root := root / 'paper'
paper-src := paper-root / 'main.typ'
paper-out := root / 'out' / 'detypstify-ocr-for-formula-generation.pdf'

export TYPST_ROOT := paper-root

[private]
default:
	@just --list --unsorted

[group('python')]
train:
        poetry run train

[group('python')]
train_val_split:
        poetry run train_val_split

[group('python')]
symbols2svg:
        poetry run symbols2svg
        
[group('typst')]
typ-preview:
        typst-preview --root {{ paper-root }} {{ paper-src }}

[group('typst')]
typ-build: 
        typst compile {{ paper-src }} {{ paper-out }}

[group('typst')]
typ-watch: 
        typst watch {{ paper-src }} {{ paper-out }}

[group('typst')]          
typ-format: 
        typstyle -i format-all 
