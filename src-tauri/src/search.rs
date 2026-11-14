use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashSet, time::Duration};

#[derive(Serialize)]
pub struct Track { id: String, title: String, subtitle: String, thumbnail: String }

fn text(v: &Value) -> String {
    v["runs"].as_array().map(|a| a.iter().filter_map(|r| r["text"].as_str()).collect::<Vec<_>>().join("")).unwrap_or_default()
}

fn collect(v: &Value, out: &mut Vec<Track>, seen: &mut HashSet<String>) {
    if out.len() >= 30 { return; }
    if let Some(item) = v.get("musicResponsiveListItemRenderer") {
        let id = item.pointer("/playlistItemData/videoId").and_then(Value::as_str)
            .or_else(|| item.pointer("/navigationEndpoint/watchEndpoint/videoId").and_then(Value::as_str));
        if let Some(id) = id {
            let cols: Vec<String> = item["flexColumns"].as_array().map(|a| a.iter().map(|c| text(&c["musicResponsiveListItemFlexColumnRenderer"]["text"])).collect()).unwrap_or_default();
            let title = cols.first().cloned().unwrap_or_default();
            if !title.is_empty() && seen.insert(id.to_owned()) {
                let thumbnail = item.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails").and_then(Value::as_array).and_then(|a| a.last()).and_then(|t| t["url"].as_str()).filter(|s| s.starts_with("https://")).unwrap_or_default().to_owned();
                out.push(Track { id: id.to_owned(), title, subtitle: cols.iter().skip(1).cloned().collect::<Vec<_>>().join(" · "), thumbnail });
            }
        }
    }
    match v {
        Value::Object(m) => for child in m.values() { collect(child, out, seen); },
        Value::Array(a) => for child in a { collect(child, out, seen); },
        _ => {}
    }
}

#[tauri::command]
pub async fn search_tracks(query: String) -> Result<Vec<Track>, String> {
    let query = query.trim();
    if query.is_empty() || query.chars().count() > 200 { return Err("Inserisci da 1 a 200 caratteri.".into()); }
    let client = reqwest::Client::builder().timeout(Duration::from_secs(20)).user_agent("Mozilla/5.0").build().map_err(|e| e.to_string())?;
    let page = client.get("https://music.youtube.com/").send().await.map_err(|e| format!("Connessione: {e}"))?.error_for_status().map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;
    let re = regex::Regex::new(r#""INNERTUBE_CLIENT_VERSION"\s*:\s*"([^"]+)""#).map_err(|e| e.to_string())?;
    let version = re.captures(&page).and_then(|c| c.get(1)).map(|m| m.as_str().to_owned()).ok_or("Inizializzazione non riuscita: possibile pagina di consenso o formato YouTube Music cambiato.")?;
    let data: Value = client.post("https://music.youtube.com/youtubei/v1/search")
        .header("Origin", "https://music.youtube.com")
        .header("Referer", "https://music.youtube.com/")
        .header("X-Youtube-Client-Name", "67")
        .header("X-Youtube-Client-Version", version.as_str())
        .json(&json!({"context":{"client":{"clientName":"WEB_REMIX","clientVersion":version,"hl":"it","gl":"IT"}},"query":query}))
        .send().await.map_err(|e| format!("Ricerca: {e}"))?.error_for_status().map_err(|e| e.to_string())?.json().await.map_err(|e| format!("Risposta non valida: {e}"))?;
    if let Some(e) = data.get("error") { return Err(format!("YouTube Music: {e}")); }
    let mut out = Vec::new();
    collect(&data, &mut out, &mut HashSet::new());
    if out.is_empty() { return Err("Nessun brano riconosciuto: nessun risultato oppure formato della risposta non supportato. Prova un'altra ricerca.".into()); }
    Ok(out)
}
