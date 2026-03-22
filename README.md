## Credit Card Fraud Detection — Data Analysis

### Statement of the Problem
Fraudulent credit card transactions pose a significant threat to financial security worldwide. This analysis investigates the behavioral patterns of fraudulent versus legitimate transactions using a real-world dataset of 284,807 transactions from European cardholders. Specifically, we ask: **do fraudulent transactions differ meaningfully from legitimate ones in terms of transaction amount and time of occurrence?**

---

### About the Dataset
The dataset was sourced from Kaggle, originally collected from real European cardholder transactions over two days in September 2013. It contains 284,807 rows and 31 columns. Features `V1` through `V28` are PCA-transformed for privacy. The three human-readable columns are `Time` (seconds elapsed since first transaction), `Amount` (transaction value), and `Class` (0 = legitimate, 1 = fraud). The dataset was loaded in Rust using Polars with a schema override on `Time` to handle scientific notation values.

---

### Preprocessing
- Schema inspected via `df.schema()` — all column types verified
- Null count checked via `df.null_count()` — confirmed zero nulls across all columns
- `Class` column (integer 0/1) converted to human-readable labels `"Legitimate"`/`"Fraud"` using Polars `when/then/otherwise` expressions
- `Time` column schema overridden to `Float64` to handle scientific notation parsing errors

---

### Exploratory Data Analysis & Key Findings

**Transaction Count by Class** — The dataset is severely imbalanced: ~284,315 legitimate transactions vs only ~492 fraud cases (less than 0.2%). A logarithmic scale was required to make fraud visible on the chart. This imbalance reflects real-world fraud rates and is a critical characteristic of the dataset.

**Average Transaction Amount by Class** — Fraudulent transactions have a higher average amount (\~$122) compared to legitimate ones (\~$88). This suggests fraudsters tend to attempt higher-value transactions, possibly to maximize gain per stolen credential.

**Average Time Elapsed by Class** — Legitimate transactions have a slightly higher average time elapsed (~95,000s) compared to fraudulent ones (~80,000s). This may indicate that fraud is more concentrated earlier in the observation window, potentially reflecting the rapid exploitation of stolen card details before they are detected and blocked.

---

Overall, while the dataset is heavily skewed toward legitimate transactions, measurable differences in both amount and timing exist between classes — suggesting these features carry meaningful signal for fraud detection.


### NOTE: AI-Assisted cos I got lazy
