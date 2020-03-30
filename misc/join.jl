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