
```shell
# everything after -- are parameters passed to the compiled program.
cd prompter
cargo run -- --help
```

```shell
cargo run -- --help
```

# Example usage
```shell
# Aggregate dumped data from IEEE and springer into a singular CSV file 
# The output will be stored in "./aggregated.csv"
cargo run  -- --ieee-source ./data/ieee_dump.csv --springer-source ./data/springer_dump.csv

# Start filtering the aggregated data allocated to timmy and mikael based on titles
cd prompter
cargo run -- -t ./aggregated.csv timmy mikael

# Start filtering aggregated data allocated to timmy based on abstracts
# cd prompter, but you should already be there if you run the commands in this order
cargo run -- -a ./aggregated.sv timmy
```

The prompter will create 3 csv files containing rejected, accepted and questionable entries.