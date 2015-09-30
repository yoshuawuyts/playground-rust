#!/bin/sh

count=$(ls | wc -l)
count=$((count - 5))
count=$(printf "%02d" $count)
name="$count"_"$1"

mkdir "$name"

touch "$name"/main.rs
touch "$name"/bundle.sh
chmod +x "$name"/bundle.sh
cat > "$name"/bundle.sh << EOF
#!/bin/sh
rustc ./main.rs -o bundle
EOF
