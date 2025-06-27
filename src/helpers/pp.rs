
use rosu_pp::{GameMods, Performance};
use bigdecimal::ToPrimitive;
use crate::models::score::score::Score;
use crate::models::map::beatmap::Beatmap;
use tracing::{error, info, warn};

pub async fn calculate_pp_for_score(
    score: &Score,
    beatmap: &Beatmap,
) -> Result<f64, String> {
    let filename = beatmap.file_path.split("/").last().unwrap();
    let filename_without_ext = filename.strip_suffix(".osu").unwrap_or(filename);
    let url = format!("https://osu.ppy.sh/osu/{}", filename_without_ext);
    let response = reqwest::get(url).await.map_err(|e| format!("Failed to download beatmap: {}", e))?;
    let beatmap_bytes = response.bytes().await.map_err(|e| format!("Failed to get bytes: {}", e))?;
    
    let map = rosu_pp::Beatmap::from_bytes(&beatmap_bytes).map_err(|e| format!("Failed to parse beatmap: {}", e))?;

    // Calculate difficulty attributes
    let diff_attrs = rosu_pp::Difficulty::new()
        .mods(score.mods as u32) 
        .calculate(&map);

    let accuracy = score.accuracy.to_f64().unwrap() * 100.0;
    info!("Accuracy : {}", accuracy.clone());
    info!("Misses : {}", score.statistics.count_miss);
    let game_mods = GameMods::from(score.mods as u32);
    info!("Game mods : {:?}", game_mods);
    
    let diff_stars = diff_attrs.stars();
    let perf_attrs = rosu_pp::Performance::new(diff_attrs)
        .mods(score.mods as u32)
        .accuracy(accuracy)
        .misses(score.statistics.count_miss as u32)
        .n300(score.statistics.count_300 as u32)
        .n100(score.statistics.count_100 as u32)
        .n50(score.statistics.count_50 as u32)
        .n_katu(score.statistics.count_katu as u32)
        .n_geki(score.statistics.count_geki as u32)
        .calculate();

    
    let pp = perf_attrs.pp();
    info!("Stars : {} - PP : {}", diff_stars, pp);
    
    Ok(pp)
} 