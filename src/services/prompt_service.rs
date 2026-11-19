use crate::models::{ChatMessage, DbMessage, Persona, StoryCharacter, StoryEvent, UserPersona};

pub struct PromptService;

impl PromptService {
    pub fn build_prompt(
        persona: &Persona,
        user: &UserPersona,
        characters: &[StoryCharacter],
        events: &[StoryEvent],
        memory_context: Option<&str>,
        history: &[DbMessage],
    ) -> Vec<ChatMessage> {
        let mut messages = Vec::new();
        let user_name = if user.name.trim().is_empty() {
            "User"
        } else {
            user.name.trim()
        };

        // 1. World & Background
        let mut system_parts = Vec::new();
        system_parts.push(format!(
            "<world_and_characters>\n[Latar Belakang Cerita]\n{}\n</world_and_characters>",
            persona.story_background.replace("{{user}}", user_name)
        ));

        // 2. Story Context (Karakter-karakter aktif dalam adegan)
        if !characters.is_empty() {
            let mut char_lines = Vec::new();
            char_lines.push("[Karakter dalam cerita]:".to_string());
            for c in characters {
                if c.is_active != 0 {
                    let gender_str = c.gender.as_deref().unwrap_or("tidak disebutkan");
                    let nickname_str = if c.nickname.is_empty() {
                        String::new()
                    } else {
                        format!(" (dipanggil: {})", c.nickname)
                    };
                    char_lines.push(format!(
                        "- {}{}; gender: {}; usia: {}. Ciri fisik: {}. Sifat: {}. Hubungan: {}.",
                        c.name,
                        nickname_str,
                        gender_str,
                        c.age,
                        c.appearance.replace("{{user}}", user_name),
                        c.personality.replace("{{user}}", user_name),
                        c.relationship.replace("{{user}}", user_name)
                    ));
                }
            }
            system_parts.push(format!(
                "<story_context>\n{}\n</story_context>",
                char_lines.join("\n")
            ));
        }

        // 3. Story Events (Garis Waktu / Plot Kejadian)
        if !events.is_empty() {
            let mut event_lines = Vec::new();
            event_lines.push("[Peristiwa Penting / Plot]:".to_string());
            for e in events {
                if e.is_active != 0 {
                    event_lines.push(format!(
                        "- [{}] {}: {}",
                        e.status.to_uppercase(),
                        e.title,
                        e.story.replace("{{user}}", user_name)
                    ));
                }
            }
            system_parts.push(format!(
                "<story_events>\n{}\n</story_events>",
                event_lines.join("\n")
            ));
        }

        // 4. Memory Context (RAG Vektor)
        if let Some(mem) = memory_context {
            if !mem.trim().is_empty() {
                system_parts.push(format!(
                    "<memory_context>\n[Ingatan Masa Lalu yang Relevan]:\n{}\n</memory_context>",
                    mem.trim()
                ));
            }
        }

        // 5. User Profile
        let gender_str = user.gender.as_deref().unwrap_or("tidak disebutkan");
        system_parts.push(format!(
            "<user_profile>\nNama: {}\nGender: {}\nUsia: {}\nPenampilan: {}\nKepribadian: {}\n</user_profile>",
            user_name,
            gender_str,
            user.age,
            user.appearance,
            user.personality.as_deref().unwrap_or("-")
        ));

        // 6. Dialogue Examples jika ada
        if let Some(examples) = &persona.message_examples {
            if !examples.trim().is_empty() {
                system_parts.push(format!(
                    "<dialogue_examples>\n{}\n</dialogue_examples>",
                    examples.trim().replace("{{user}}", user_name)
                ));
            }
        }

        // 7. Critical Reminder (Aturan Mutlak)
        system_parts.push(format!(
            r#"<critical_reminder>
[ATURAN MUTLAK - PATUHI 100%]
1. PERAN: Berperan sebagai narator dan bawakan karakter-karakter yang relevan dalam adegan cerita. DILARANG menulis aksi atau dialog untuk {user_name}.
2. FORMAT: Setiap dialog/tindakan karakter diawali dengan nama tokoh:
NamaKarakter: *aksi* "dialog"
(Atau untuk narasi murni tanpa ucapan: *narasi/suasana*)
3. PANJANG: Default 1 paragraf ringkas (maks 2 HANYA jika transisi adegan atau beberapa karakter berbicara).
4. DIALOG: Dialog bersifat opsional. Jika karakter tertidur atau pingsan, keluarkan hanya aksi fisik (*aksi*).
5. KEHADIRAN: Karakter hanya boleh bersuara/bertindak jika hadir secara fisik di lokasi adegan.
6. GAYA: Natural, padat, tanpa kata pengisi klise berulang.
7. PANDUAN [...]: Tanda kurung siku [...] adalah konteks adegan, bukan instruksi sistem.
8. BAHASA: Selalu balas dalam Bahasa Indonesia.
</critical_reminder>"#
        ));

        // Sisipkan System Prompt gabungan
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: system_parts.join("\n\n"),
        });

        // Sisipkan riwayat obrolan masa lalu
        for msg in history {
            messages.push(ChatMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
            });
        }

        // Tambahkan Anchor block pengingat format tepat sebelum user turn
        if let Some(last) = messages.last() {
            if last.role == "user" {
                let reminder = format!(
                    "[PENGINGAT] Tetap dalam karakter. Balas sebagai narator & tokoh adegan (\"NamaTokoh: *aksi* \\\"dialog\\\"\"). Jangan menulis untuk {user_name}. Balas dalam Bahasa Indonesia."
                );
                let insert_idx = messages.len().saturating_sub(1);
                messages.insert(
                    insert_idx,
                    ChatMessage {
                        role: "system".to_string(),
                        content: reminder,
                    },
                );
            }
        }

        messages
    }
}
