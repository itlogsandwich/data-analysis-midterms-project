use polars::prelude::*;
use plotters::prelude::*;
use std::path::Path;
use std::sync::Arc;

type PolarsResult<T> = Result<T, PolarsError>;
type LazyResult<T> = Result<T, Box<dyn core::error::Error>>;
const FILE_PATH: &str = "./new_assets/creditcard.csv";

fn get_data_frame() -> PolarsResult<DataFrame>
{
    let path = Path::new(FILE_PATH);
    let schema_override = Schema::from_iter([
        Field::new("Time".into(), DataType::Float64),
    ]);

    CsvReadOptions::default()
        .with_has_header(true)
        .with_schema_overwrite(Some(Arc::new(schema_override)))
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

fn class_to_avg_amount_chart(labels: Vec<&str>, means: Vec<f64>) -> LazyResult<()>
{
    let max_val = means.iter().cloned().fold(0.0f64, f64::max) * 1.2;

    let root = BitMapBackend::new("avg_amount_by_class.png", (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Transaction Amount: Fraud vs Legitimate", ("sans-serif", 28))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(
            ["Legitimate", "Fraud"].into_segmented(),
            0f64..max_val
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
                        "Legitimate" => BLUE.filled(),
                        "Fraud"      => RED.filled(),
                        _            => BLACK.filled(),
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(labels.iter().zip(means.iter()).map(|(cat, val)| (cat, *val)))
    )?;

    root.present()?;
    Ok(())
}

fn class_to_avg_time_chart(labels: Vec<&str>, means: Vec<f64>) -> LazyResult<()>
{
    let max_val = means.iter().cloned().fold(0.0f64, f64::max) * 1.2;

    let root = BitMapBackend::new("avg_time_by_class.png", (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Time Elapsed: Fraud vs Legitimate", ("sans-serif", 28))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(
            ["Legitimate", "Fraud"].into_segmented(),
            0f64..max_val
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
                        "Legitimate" => BLUE.filled(),
                        "Fraud"      => RED.filled(),
                        _            => BLACK.filled(),
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(labels.iter().zip(means.iter()).map(|(cat, val)| (cat, *val)))
    )?;

    root.present()?;
    Ok(())
}

fn class_to_count_chart(labels: Vec<&str>, counts: Vec<u32>) -> LazyResult<()>
{
    let max_count = *counts.iter().max().unwrap_or(&0) as f64 * 1.1;

    let root = BitMapBackend::new("transaction_count_by_class.png", (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Transaction Count: Fraud vs Legitimate", ("sans-serif", 28))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(
            ["Legitimate", "Fraud"].into_segmented(),
            (1f64..max_count).log_scale()
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
                        "Legitimate" => BLUE.filled(),
                        "Fraud"      => RED.filled(),
                        _            => BLACK.filled(),
                    },
                    _ => BLACK.filled(),
                }
            })
            .data(labels.iter().zip(counts.iter()).map(|(cat, val)| (cat, *val as f64)))
    )?;

    root.present()?;
    Ok(())
}

fn main() -> LazyResult<()>
{
    let df = get_data_frame()?;

    println!("Schema: {:?}", df.schema());
    println!("Null Count:\n{}", df.null_count());
    println!("First 10 rows:\n{}", df.head(Some(10)));

    let lazy_df = df.lazy()
        .with_column(
            when(col("Class").eq(lit(0)))
                .then(lit("Legitimate"))
                .otherwise(lit("Fraud"))
                .alias("class_label")
        );

    // Analysis 1: avg amount by class
    let avg_amount = lazy_df.clone()
        .group_by([col("class_label")])
        .agg([col("Amount").mean().alias("avg_amount")])
        .sort(["class_label"], SortMultipleOptions::default())
        .collect()?;

    let avg_amount_source =
    {
        let labels: Vec<&str> = avg_amount["class_label"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap())
            .collect();

        let means: Vec<f64> = avg_amount["avg_amount"]
            .f64()?
            .into_iter()
            .map(|x| x.unwrap_or(0.0))
            .collect();

        (labels, means)
    };

    // Analysis 2: avg time by class
    let avg_time = lazy_df.clone()
        .group_by([col("class_label")])
        .agg([col("Time").mean().alias("avg_time")])
        .sort(["class_label"], SortMultipleOptions::default())
        .collect()?;

    let avg_time_source =
    {
        let labels: Vec<&str> = avg_time["class_label"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap())
            .collect();

        let means: Vec<f64> = avg_time["avg_time"]
            .f64()?
            .into_iter()
            .map(|x| x.unwrap_or(0.0))
            .collect();

        (labels, means)
    };

    // Analysis 3: transaction count by class
    let class_counts = lazy_df.clone()
        .group_by([col("class_label")])
        .agg([col("Class").count().alias("tx_count")])
        .sort(["class_label"], SortMultipleOptions::default())
        .collect()?;

    let class_count_source =
    {
        let labels: Vec<&str> = class_counts["class_label"]
            .str()?
            .into_iter()
            .map(|s| s.unwrap())
            .collect();

        let counts: Vec<u32> = class_counts["tx_count"]
            .u32()?
            .into_iter()
            .map(|x| x.unwrap_or(0))
            .collect();

        (labels, counts)
    };

    let _ = class_to_avg_amount_chart(avg_amount_source.0, avg_amount_source.1);
    let _ = class_to_avg_time_chart(avg_time_source.0, avg_time_source.1);
    let _ = class_to_count_chart(class_count_source.0, class_count_source.1);

    Ok(())
}
