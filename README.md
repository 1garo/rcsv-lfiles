# rcsv-lfiles

[node.js example to be compared](https://github.com/ErickWendel/concat-large-files-nodejs-yt), see Benchmark for more info.

## Running/Executing the app
#### Example on how to run the project

```sh
$ mkdir dataset // Follow the Dataset part to fill it with csv files
$ touch src/final.csv
$ cargo build --release 
$ time ./target/release/rcsv-lfiles 
```


## Benchmark
On my Ryzen7 5800X:

rust = +-0.49s

node = +-12s

| Language     | Seconds (+-)    
|--------------|-----------
| Rust | **0.49**  
| Node | 1.89

## Dataset 
#### Download the files below and extract it under /dataset folder:
- [2017](https://www.kaggle.com/stackoverflow/so-survey-2017?select=survey_results_public.csv)
- [2018](https://www.kaggle.com/stackoverflow/stack-overflow-2018-developer-survey?select=survey_results_public.csv)

OBS: Remember to change their name, both are gonna be `survey_results_public.csv`.
## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Made by ♥ Alexandre Vardai 👋 
[Find me here](https://www.linkedin.com/in/alexandre-vardai-b8255b15b/)
