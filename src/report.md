

subset generation
```bash 
for i in 0 1 2 3 4 5 6 7; do
	shuf data/retail.dat | head -n 1024 > data/subset$i.dat;
done
```
