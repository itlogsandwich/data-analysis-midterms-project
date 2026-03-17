# Data Analysis Midterms Project

### About
A simple data analysis on chess dataset from Lichess (source: <a href="https://www.kaggle.com/datasets/datasnaek/chess">Kaggle</a>). I wanted to try and experiment using Rust's data engineering and analysis <br/>


### Key Findings and Insights
<strong> Rating Difference by Winner </strong> — games were grouped by outcome and the mean rate_diff was computed per group. Finding: stronger players win consistently regardless of color, while draws occur between near-equal players.

<strong> Average Turns by Winner </strong> — games were grouped by outcome and mean turn count was computed. Finding: draws last significantly longer than decisive games, suggesting closely matched players play more complex, prolonged games.

<strong> Top 10 Opening/Winner Combinations </strong> — games were grouped by both opening_name and winner, counted, and the top 10 most frequent combinations were extracted. Finding: the Van't Kruijs Opening and Sicilian Defense dominate winning games, with black and white winners favoring different openings — suggesting opening preparation plays a meaningful role in outcomes.
