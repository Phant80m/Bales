# Bales
super simple rust archive utility based on freight


usage: 
```bash
bales compress bar.txt foo/ baz.rs -o main.tar.gz  
```
```bash
bales compress bar.txt foo/ baz.rs -o main.zip
```

### How to extract a file
works with .tar.gz | .zip
usage: 
```bash
bales extract file.tar.gz -o output/ 
```
output can be assumed: 
```bash
bales extract file.tar.gz # output will be file/
```

### How to extract a file from a url
```bash
bales extract --url https://example.com/sample.tar.gz -o main.zip # output must be specified 
```
