use serde::Deserialize;
use sqlx::{SqlitePool, Row};

#[derive(Debug, Deserialize)]
pub struct ZenQuote {
    #[serde(rename = "q")]
    pub text: String,
    #[serde(rename = "a")]
    pub author: String,
}

pub async fn fetch_and_cache_quotes(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Free API endpoint from zenquotes.io gives 50 random quotes
    let url = "https://zenquotes.io/api/quotes";

    // Attempt to fetch quotes
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                if let Ok(quotes) = res.json::<Vec<ZenQuote>>().await {
                    println!("Fetched {} quotes from ZenQuotes API.", quotes.len());

                    // Insert into DB
                    for quote in quotes {
                        // Check if it already exists to avoid duplicates (basic check)
                        let count: i64 = sqlx::query("SELECT COUNT(*) FROM quotes_cache WHERE text = ?")
                            .bind(&quote.text)
                            .fetch_one(pool)
                            .await?
                            .get(0);

                        if count == 0 {
                            sqlx::query("INSERT INTO quotes_cache (text, author) VALUES (?, ?)")
                                .bind(&quote.text)
                                .bind(&quote.author)
                                .execute(pool)
                                .await?;
                        }
                    }
                }
            } else {
                eprintln!("Failed to fetch quotes: HTTP {}", res.status());
            }
        },
        Err(e) => {
            eprintln!("Network error while fetching quotes: {}. Assuming offline mode.", e);
        }
    }

    // Ensure we have fallback quotes if cache is completely empty and fetch failed
    let count: i64 = sqlx::query("SELECT COUNT(*) FROM quotes_cache")
        .fetch_one(pool)
        .await?
        .get(0);

    if count == 0 {
        println!("Quotes cache is empty and fetch failed. Seeding fallback quotes...");
        let fallback_quotes = vec![
            ("It always seems impossible until it's done.", "Nelson Mandela"),
            ("The secret of getting ahead is getting started.", "Mark Twain"),
            ("Don't watch the clock; do what it does. Keep going.", "Sam Levenson"),
            ("Success is not final, failure is not fatal: it is the courage to continue that counts.", "Winston Churchill"),
            ("You miss 100% of the shots you don't take.", "Wayne Gretzky"),
        ];

        for (text, author) in fallback_quotes {
            sqlx::query("INSERT INTO quotes_cache (text, author) VALUES (?, ?)")
                .bind(text)
                .bind(author)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}

pub async fn get_random_quote(pool: &SqlitePool) -> Option<(String, String)> {
    let result = sqlx::query("SELECT text, author FROM quotes_cache ORDER BY RANDOM() LIMIT 1")
        .fetch_optional(pool)
        .await;

    if let Ok(Some(row)) = result {
        let text: String = row.get(0);
        let author: Option<String> = row.get(1);
        Some((text, author.unwrap_or_else(|| "Unknown".to_string())))
    } else {
        None
    }
}
