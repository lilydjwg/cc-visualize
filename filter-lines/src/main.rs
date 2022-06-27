use std::collections::HashMap;
use std::io::{BufRead, Write, BufWriter};

use eyre::Result;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct CharInfo {
  rel: Vec<String>,
  #[serde(default)]
  is_rad: bool,
  #[serde(default)]
  is_simp: bool,
  #[serde(default)]
  is_trad: bool,
  #[serde(default)]
  is_comp: bool,
  #[serde(default)]
  is_chi: bool,
  #[serde(default, rename="isVari_HK")]
  is_vari_hk: bool,
  #[serde(default, rename="isVari_TW")]
  is_vari_tw: bool,
  #[serde(default, rename="isVari_JP")]
  is_vari_jp: bool,
}

fn filter(ch: char, data: &HashMap<char, CharInfo>) -> bool {
  if is_cjk_compat(ch) || is_others(ch) {
    return true;
  }

  if let Some(info) = data.get(&ch) {
    info.is_comp
      || (info.is_vari_jp && !info.is_chi && !info.is_simp)
      || (info.is_trad && !info.is_simp)
  } else {
    false
  }
}

fn main() -> Result<()> {
  color_eyre::config::HookBuilder::new()
    .capture_span_trace_by_default(false)
    .install()?;

  let raw_data = std::fs::read_to_string("../summary-data/summary-data-map2.js")?;
  let raw_data = raw_data.split_once('\n').unwrap().1;
  let raw_data = raw_data.rsplit_once('\n').unwrap().0;
  let data: HashMap<char, CharInfo> = serde_json::from_str(raw_data)?;

  let mut line = String::new();
  let mut stdin = std::io::stdin().lock();
  let stdout = std::io::stdout().lock();
  let mut stdout = BufWriter::new(stdout);
  while {
    line.clear();
    stdin.read_line(&mut line)? != 0
  } {
    if line.chars().any(|ch| filter(ch, &data)) {
      stdout.write_all(&line.as_bytes())?;
    }
  }

  Ok(())
}

fn is_cjk_compat(ch: char) -> bool {
  let blocks = [
    unicode_blocks::CJK_COMPATIBILITY,
    unicode_blocks::CJK_COMPATIBILITY_FORMS,
    unicode_blocks::CJK_COMPATIBILITY_IDEOGRAPHS,
    unicode_blocks::CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT,
  ];
  blocks.iter().any(|b| b.contains(ch))
}

fn is_others(ch: char) -> bool {
  let blocks = [
    unicode_blocks::CJK_RADICALS_SUPPLEMENT,
    unicode_blocks::CJK_SYMBOLS_AND_PUNCTUATION,
    unicode_blocks::CJK_STROKES,
    unicode_blocks::ENCLOSED_CJK_LETTERS_AND_MONTHS,
    unicode_blocks::CJK_COMPATIBILITY,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS,
    unicode_blocks::CJK_COMPATIBILITY_IDEOGRAPHS,
    unicode_blocks::CJK_COMPATIBILITY_FORMS,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F,
    unicode_blocks::CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT,
    unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G,
    // not defined:
    // unicode_blocks::CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H,
    unicode_blocks::KANGXI_RADICALS,
    unicode_blocks::HALFWIDTH_AND_FULLWIDTH_FORMS,
    unicode_blocks::GENERAL_PUNCTUATION,
    unicode_blocks::BASIC_LATIN,
    unicode_blocks::ENCLOSED_ALPHANUMERICS,
  ];
  !blocks.iter().any(|b| b.contains(ch))
}
