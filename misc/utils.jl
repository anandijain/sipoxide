using CSV, DataFrames

function join_lines_scores()
    df, df2 = CSV.read.(["data/scores.csv", "data/root.csv"])
    join(df, df2, on = :id; makeunique = true, kind = :outer)
end

function refine_joined(df::DataFrame)
    dropmissing!(df, :sport)
    df = df[df.sport .!= "NUMBERS", :]
    CSV.write("data/bov_joined.csv", df)
end


df = join_lines_scores()
refine_joined(df)

df = CSV.read("./data/mlb_index_1960_2017.csv")

s1 = df[:, [:pk, :away_name, :away_wins, :away_losses]]
s2 = df[:, [:pk, :home_name, :home_wins, :home_losses]]
rename!(s1, [:away_name=> :name, :away_wins =>:wins, :away_losses=>:losses])
rename!(s2, [:home_name=> :name, :home_wins =>:wins, :home_losses=>:losses])
vcat(s1, s2)
df[:, r"h.*.:US"]
df[:, r"away_*"]

df[:, [r"home_*", :pk]]
