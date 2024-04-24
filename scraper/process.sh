for svg in ./OUT/images/*.svg; do
        convert "$svg" -background white -flatten -fuzz 1% -trim +repage "${svg%.svg}.png"
done
