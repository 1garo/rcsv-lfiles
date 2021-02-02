# rcsv-lfiles

[post that inspired this repo](https://www.reddit.com/r/rust/comments/8833lh/performance_of_parsing_large_file_2gb/)

aim to do something similiar as this [repo](https://github.com/ErickWendel/concat-large-files-nodejs-yt), benchmark and see how it perform
## Running

```sh
$ cd dataset && curl -LO http://burntsushi.net/stuff/worldcitiespop.csv.gz
$ gunzip worldcitiespop.csv.gz
$ cargo build --release
$ time ./target/release/rcsv-lfiles < dataset/worldcitiespop.csv
```

## WIP
#### Concat the files below and change name field spaces to underscore:

2018: https://www.kaggle.com/stackoverflow/stack-overflow-2018-developer-survey?select=survey_results_public.csv

2017: https://www.kaggle.com/stackoverflow/so-survey-2017?select=survey_results_public.csv

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Made by ♥ Alexandre Vardai 👋 
[Find me here](https://www.linkedin.com/in/alexandre-vardai-b8255b15b/)