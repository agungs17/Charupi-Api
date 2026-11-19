use sqlx::SqlitePool;
use crate::{
    error::AppResult,
    models::{Chunk, MemorySearchResult},
};

pub struct VectorService;

impl VectorService {
    /// Menghitung Cosine Distance antara dua vektor float32:
    /// distance = 1.0 - (A . B) / (||A|| * ||B||)
    /// Nilai 0.0 = sama persis, 1.0 = ortogonal, 2.0 = berlawanan.
    pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 1.0;
        }

        let mut dot_product = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for (x, y) in a.iter().zip(b.iter()) {
            dot_product += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        if norm_a <= 0.0 || norm_b <= 0.0 {
            return 1.0;
        }

        let similarity = dot_product / (norm_a.sqrt() * norm_b.sqrt());
        (1.0 - similarity).clamp(0.0, 2.0)
    }

    /// Konversi Vec<f32> ke little-endian byte array (BLOB)
    pub fn f32_to_bytes(floats: &[f32]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(floats.len() * 4);
        for &val in floats {
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        bytes
    }

    /// Konversi little-endian byte array (BLOB) ke Vec<f32>
    pub fn bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
        bytes
            .chunks_exact(4)
            .map(|chunk| {
                let arr: [u8; 4] = chunk.try_into().unwrap_or([0; 4]);
                f32::from_le_bytes(arr)
            })
            .collect()
    }

    /// Mencari memory chunks yang relevan untuk suatu persona
    pub async fn search_relevant_memories(
        pool: &SqlitePool,
        persona_id: i64,
        query_vector: &[f32],
        top_k: usize,
        threshold: f32,
    ) -> AppResult<Vec<MemorySearchResult>> {
        let rows = sqlx::query_as::<_, Chunk>(
            r#"
            SELECT id, persona_id, first_msg_id, last_msg_id, text, embedding, model, created_at
            FROM chunks
            WHERE persona_id = ?
            ORDER BY id DESC
            LIMIT 100
            "#
        )
        .bind(persona_id)
        .fetch_all(pool)
        .await?;

        let mut matches = Vec::new();

        for chunk in rows {
            let chunk_vec = Self::bytes_to_f32(&chunk.embedding);
            let distance = Self::cosine_distance(query_vector, &chunk_vec);
            if distance <= threshold {
                matches.push(MemorySearchResult {
                    id: chunk.id,
                    text: chunk.text,
                    distance,
                    first_msg_id: chunk.first_msg_id,
                    last_msg_id: chunk.last_msg_id,
                });
            }
        }

        // Urutkan berdasarkan jarak terkecil (paling relevan)
        matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
        matches.truncate(top_k);

        Ok(matches)
    }
}
