use polars::prelude::*;
use plotters::prelude::*;
use std::path::Path;

type PolarsResult<T> = Result<T, PolarsError>;
type LazyResult<T> = Result<T, Box<dyn core::error::Error>>;
const FILE_PATH: &str = "./assets/games.csv";

fn get_data_frame() -> PolarsResult<DataFrame>
{
    let path = Path::new(FILE_PATH);
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

fn winner_to_rate_diff_chart(winners: Vec<&str>, means: Vec<f64>) -> LazyResult<()>
{
    let root = BitMapBackend::new("winner_rating.png", (800,600))
        .into_drawing_area();
    
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Rating Diff by Winner", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            ["black", "draw","white"].into_segmented(),
            0f64..300f64
        )?;

    chart.configure_mesh().draw()?;
   
    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(70)
            .style_func(|cat, _val| 
            {
                match cat 
                {
                    SegmentValue::Exact(s) | SegmentValue::CenterOf(s) => match **s 
                    {
                        "black" => RED.filled(),
                        "draw"  => GREEN.filled(),
                        "white" => BLUE.filled(),
                        _       => BLACK.filled(),
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(
                winners.iter().zip(means.iter()).map(|(cat, val)| (cat, *val))
            )
    )?;

    root.present()?;
    Ok(())
}

fn winner_to_turns_chart(winners: Vec<&str>, turns: Vec<f64>) -> LazyResult<()>
{
    let root = BitMapBackend::new("winner_turns.png", (800,600))
        .into_drawing_area();
    
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Turns by Winner", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            ["black", "draw","white"].into_segmented(),
            0f64..80f64
        )?;

    chart.configure_mesh().draw()?;
   
    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(70)
            .style_func(|cat, _val| 
            {
                match cat 
                {
                    SegmentValue::Exact(s) | SegmentValue::CenterOf(s) => match **s 
                    {
                        "black" => RED.filled(),
                        "draw"  => GREEN.filled(),
                        "white" => BLUE.filled(),
                        _       => BLACK.filled(),
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(
                winners.iter().zip(turns.iter()).map(|(cat, val)| (cat, *val))
            )
    )?;

    root.present()?;
    Ok(())
}

fn winner_to_opening_chart(winners: Vec<String>, count: Vec<u32>, unique_labels: Vec<String>) -> LazyResult<()> 
{
    let axis_labels: Vec<&str> = unique_labels.iter().map(|s| s.as_str()).collect();

    let label_to_winner: std::collections::HashMap<&str, &str> = axis_labels.iter()
        .copied()
        .zip(winners.iter().map(|s| s.as_str()))
        .collect();

    let max_count = (*count.iter().max().unwrap_or(&0) as f64 * 1.1).max(1.0);
    let root = BitMapBackend::new("winner_openings.png", (2500, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Top 10 Opening/Winner Combinations", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(250)
        .y_label_area_size(80)
        .build_cartesian_2d(
            axis_labels.as_slice().into_segmented(),
            0f64..(max_count+60.0)
        )?;

    chart.configure_mesh()
        .x_label_style(("sans-serif", 12))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(25)
            .style_func(|cat, _val| 
            {
                match cat 
                {
                    SegmentValue::Exact(s) | SegmentValue::CenterOf(s) => 
                    {
                        match label_to_winner.get(*s).copied().unwrap_or("") 
                        {
                            "black" => RED.filled(),
                            "draw"  => GREEN.filled(),
                            "white" => BLUE.filled(),
                            _       => BLACK.filled(),
                        }
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(
                axis_labels.iter().zip(count.iter()).map(|(l, c)| (l, *c as f64))
            )
    )?;

    root.present()?;
    Ok(())
}

fn rated_to_turns_chart(rated: Vec<&str>, turns: Vec<f64>) -> LazyResult<()>
{
    let root = BitMapBackend::new("rated_turns.png", (800, 600))
        .into_drawing_area();
    
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Turns by Rated", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            ["Unrated", "Rated"].into_segmented(),
            0f64..80f64
        )?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(70)
            .style_func(|cat, _val| 
            {
                match cat 
                {
                    SegmentValue::Exact(s) | SegmentValue::CenterOf(s) => match **s 
                    {
                        "Unrated" => RED.filled(),
                        "Rated"  => BLUE.filled(),
                        _       => BLACK.filled(),
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(rated.iter().zip(turns.iter()).map(|(cat, val)| (cat, *val)))
    )?;

    root.present()?;
    Ok(())
}

fn main() -> LazyResult<()>
{
    let df = get_data_frame()?;

    println!("Data frame Schema {:?}",df.schema());
    println!("Null Count: {}", df.null_count());
    println!("First 10 values {}", df.head(Some(10)));

    let lz_rate_difference = df.lazy()
        .with_column(
            when(col("winner").eq(lit("white")))
                .then(col("white_rating") - col("black_rating"))
                .otherwise(col("black_rating") - col("white_rating"))
                .alias("rate_diff")
        )
        .collect()?;
  
    let winner_rating = lz_rate_difference.clone().lazy()
        .group_by([col("winner")])
        .agg([
            col("rate_diff").mean()
        ])
        .sort(["winner"], SortMultipleOptions::default())
        .collect()?;

    let winner_rate_source = 
    {
        let winners: Vec<&str> = winner_rating["winner"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap())
            .collect();

        let means: Vec<f64> = winner_rating["rate_diff"]
            .f64()?
            .into_iter()
            .map(|x| x.unwrap_or(0.0))
            .collect();
        
        (winners, means)
    };
    
    let winner_turns = lz_rate_difference.clone().lazy()
        .group_by([col("winner")])
        .agg([
            col("turns").mean().alias("avg_turns")
        ])
        .sort(["winner"], SortMultipleOptions::default())
        .collect()?;

    let winner_turns_source = 
    {
        let winners: Vec<&str> = winner_turns["winner"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap())
            .collect();

        let turns: Vec<f64> = winner_turns["avg_turns"]
            .f64()?
            .into_iter()
            .map(|x| x.unwrap_or(0.0))
            .collect();
        
        (winners, turns)
    };


    let winner_opening = lz_rate_difference.clone().lazy()
        .group_by([col("winner"), col("opening_name")])
        .agg([
            col("opening_name").count().alias("opn_count")
        ])
        .sort(["opn_count"], SortMultipleOptions::default().with_order_descending(true))
        .limit(10)
        .collect()?;

    let winner_opening_source = 
    {
        let winners: Vec<String> = winner_opening["winner"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap().to_string())
            .collect();

        let opening_count: Vec<u32> = winner_opening["opn_count"]
            .u32()?
            .into_iter()
            .map(|x| x.unwrap_or(0))
            .collect();
        
        let opening_names: Vec<String> = winner_opening["opening_name"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap().to_string())
            .collect();

        let unique_labels: Vec<String> = opening_names.iter().zip(winners.iter())
            .map(|(o, w)| format!("{} ({})", o, w))
            .collect();

        (winners, opening_count, unique_labels)
    };

    let winner_rated = lz_rate_difference.lazy()
        .group_by([col("rated")])
        .agg([col("turns").mean().alias("avg_turns")])
        .sort(["rated"], SortMultipleOptions::default())
        .collect()?;

    let winner_rated_source =
    {
        let rated: Vec<&str> = winner_rated["rated"]
            .bool()?
            .into_iter()
            .map(|x| if x.unwrap_or(false) { "Rated" } else { "Unrated" })
            .collect();

        let turns: Vec<f64> = winner_rated["avg_turns"]
            .f64()?
            .into_iter()
            .map(|x| x.unwrap_or(0.0))
            .collect();

        (rated, turns)
    };

    let _ = winner_to_rate_diff_chart(winner_rate_source.0, winner_rate_source.1);
    let _ = winner_to_turns_chart(winner_turns_source.0, winner_turns_source.1);
    let _ = winner_to_opening_chart(winner_opening_source.0, winner_opening_source.1, winner_opening_source.2);
    let _ = rated_to_turns_chart(winner_rated_source.0, winner_rated_source.1);
    Ok(())
}
