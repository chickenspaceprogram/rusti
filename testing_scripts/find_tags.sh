cd $(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

# really inefficient but i do not care
grep "<[^/]*>" ../tokens/8X.xml | sed -e 's/<\/.*>//' | sed -e 's/^[ 	]*<//' | sed -e 's/>.*//' | sed -e 's/\?xml.*//' | grep . > temp

# this is a dumb way to do this
cat temp | sed -e 's/ .*//' | sort -u > tags.txt # putting all the tags into the file `tags.txt`

grep "=" temp | sed -e 's/ .*//' | sort -u > tags_with_attribs.txt

cat temp | sed -e 's/ /\n/g' | grep "=" | sed 's/=.*//' | sort -u > attributes.txt

rm temp