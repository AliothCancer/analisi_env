import polars as pl
from pprint import pprint as print



df = pl.read_csv(
    "data/data_usable_head.csv", 
    infer_schema_length=10_000,
    try_parse_dates=True
)

body_data_cols = ['sex',
 'ID',
 'data',
 'nr pr.',
 'data nascita',
 'età',
 'MaxFreq',
 'Freq85p100',
 'Kg',
 'Cm',
 'BMI',
 'Stress']

# selecting columns
body_data = df.select_seq(pl.col(body_data_cols))

print(body_data.head())