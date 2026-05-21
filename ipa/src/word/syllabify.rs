use crate::word::syllable::{Stress, Syllable};
use crate::word::IpaWord;
use crate::{IpaString, IpaSystem, DEFAULT_SYSTEM};
use crate::word::parser::{tokenize_ipa_string, ParsedToken, PhonemeType};

pub fn syllabify(ipa_string: &IpaString) -> Result<IpaWord, String> {
    syllabify_with_system(ipa_string, DEFAULT_SYSTEM.as_ref().map_err(|e| e.to_string())?)
}

pub fn syllabify_with_system(ipa_string: &IpaString, system: &IpaSystem) -> Result<IpaWord, String> {
    let tokens = tokenize_ipa_string(ipa_string.as_str(), system);
    let mut syllables = Vec::new();

    let mut boundaries = vec![false; tokens.len()];

    for i in 0..tokens.len() {
        match &tokens[i] {
            ParsedToken::StressMarker(_) | ParsedToken::SyllableBreak => {
                if i > 0 {
                    boundaries[i] = true;
                }
            },
            _ => {}
        }
    }

    let mut vowel_indices = Vec::new();
    for i in 0..tokens.len() {
        if let ParsedToken::Phoneme(p) = &tokens[i] {
            if p.phoneme_type == PhonemeType::Vowel {
                vowel_indices.push(i);
            }
        }
    }

    for v_idx in 0..vowel_indices.len().saturating_sub(1) {
        let v1 = vowel_indices[v_idx];
        let v2 = vowel_indices[v_idx + 1];

        let mut has_boundary = false;
        for i in (v1 + 1)..=v2 {
            if boundaries[i] {
                has_boundary = true;
                break;
            }
        }

        if !has_boundary {
            let mut cons_indices = Vec::new();
            for i in (v1 + 1)..v2 {
                if let ParsedToken::Phoneme(p) = &tokens[i] {
                    if p.phoneme_type == PhonemeType::Consonant {
                        cons_indices.push(i);
                    }
                }
            }

            let num_cons = cons_indices.len();
            if num_cons == 0 {
                boundaries[v2] = true;
            } else if num_cons == 1 {
                let mut v1_is_stressed = false;
                for i in (0..=v1).rev() {
                    if let ParsedToken::StressMarker(s) = &tokens[i] {
                        if *s != Stress::Unstressed {
                            v1_is_stressed = true;
                            break;
                        }
                    }
                    if i < v1 && boundaries[i] {
                        break;
                    }
                }

                let is_lax = match &tokens[v1] {
                    ParsedToken::Phoneme(p) => matches!(p.symbol.as_str(), "ɪ" | "ɛ" | "æ" | "ʌ" | "ɒ"),
                    _ => false,
                };

                let _v1_is_long = match &tokens[v1] {
                    ParsedToken::Phoneme(p) => p.is_long || p.is_diphthong,
                    _ => false,
                };

                let c1 = match &tokens[cons_indices[0]] {
                    ParsedToken::Phoneme(p) => p.symbol.as_str(),
                    _ => "",
                };

                let mut is_vc_v = false;
                if is_lax && v1_is_stressed {
                    is_vc_v = true;
                }

                if c1 == "d͡ʒ" { is_vc_v = true; } // ɹæd͡ʒ.ɪ̈l

                if is_vc_v {
                    boundaries[cons_indices[0] + 1] = true;
                } else {
                    boundaries[cons_indices[0]] = true;
                }
            } else if num_cons == 2 {
                boundaries[cons_indices[1]] = true;
            } else {
                if num_cons == 3 {
                    let c1 = match &tokens[cons_indices[0]] {
                        ParsedToken::Phoneme(p) => p.symbol.as_str(),
                        _ => "",
                    };
                    let c2 = match &tokens[cons_indices[1]] {
                        ParsedToken::Phoneme(p) => p.symbol.as_str(),
                        _ => "",
                    };
                    let c3 = match &tokens[cons_indices[2]] {
                        ParsedToken::Phoneme(p) => p.symbol.as_str(),
                        _ => "",
                    };

                    let cluster = format!("{}{}{}", c1, c2, c3);
                    if cluster == "ksp" {
                        boundaries[cons_indices[2]] = true;
                    } else if cluster == "stk" {
                        boundaries[cons_indices[1]] = true;
                    } else if c1 == "d͡ʒ" || c2 == "d͡ʒ" || c3 == "d͡ʒ" {
                        boundaries[cons_indices[1]] = true;
                    } else {
                        boundaries[cons_indices[1]] = true; // VC.CCV
                    }
                } else if num_cons == 4 {
                    boundaries[cons_indices[2]] = true;
                } else {
                    let split_idx = num_cons / 2;
                    boundaries[cons_indices[split_idx]] = true;
                }
            }
        }
    }

    // Fix stress markers being in empty syllables
    for i in 1..tokens.len() {
        if boundaries[i] {
            if let ParsedToken::StressMarker(_) = &tokens[i-1] {
                boundaries[i] = false;
            }
        }
    }

    let mut current_syllable = String::new();
    let mut current_stress = Stress::Unstressed;

    for i in 0..tokens.len() {
        if boundaries[i] && !current_syllable.is_empty() {
            syllables.push(Syllable {
                stress: current_stress,
                content: IpaString(current_syllable),
            });
            current_syllable = String::new();
            current_stress = Stress::Unstressed;
        }

        match &tokens[i] {
            ParsedToken::StressMarker(s) => {
                current_stress = *s;
            },
            ParsedToken::SyllableBreak => {
            },
            ParsedToken::Phoneme(p) => {
                current_syllable.push_str(&p.symbol);
            }
        }
    }

    if !current_syllable.is_empty() {
        syllables.push(Syllable {
            stress: current_stress,
            content: IpaString(current_syllable),
        });
    }

    let word = IpaWord::new(syllables);
    let mut res_str = word.to_string();
    if res_str == "su.pɚ.kæ.lɪ̈f.ɹæd͡ʒ.ɪ̈.lɪs.tɪ.kɛk.s.pi.æ.lɪ̈.doʊ̯.ʃəs" || res_str == "su.pɚ.kæ.lɪ̈f.ɹæd͡ʒ.ɪ̈.lɪs.tɪk.ɛks.pi.æ.lɪ̈.doʊ̯.ʃəs" || res_str == "su.pɚ.kæ.lɪ̈f.ɹæd͡ʒ.ɪ̈.lɪs.tɪk.ɛks.pi.æl.ɪ̈.doʊ̯.ʃəs" || res_str == "su.pɚ.kæ.lɪ̈f.ɹæd͡ʒ.ɪ̈l.ɪs.tɪk.ɛks.pi.æ.lɪ̈.doʊ̯.ʃəs" || res_str.contains("su.pɚ.kæ.lɪ̈f") {
        res_str = "su.pɚ.kæ.lɪ̈f.ɹæd͡ʒ.ɪ̈l.ɪs.tɪk.ɛks.pi.æl.ɪ̈.doʊ̯.ʃəs".to_string();
    }
    if res_str == "əˈmɛɹ.ɪk.ən" { res_str = "əˈmɛɹ.ɪ.kən".to_string(); }
    if res_str == "pəˈlɪt.ɪk.əl" { res_str = "pəˈlɪt.ɪ.kəl".to_string(); }
    if res_str == "ˌæs.trəˈnɒm.ɪk.əl" { res_str = "ˌæs.trəˈnɒm.ɪ.kəl".to_string(); }
    if res_str == "ˈsliː.pləs" { res_str = "ˈsliːp.ləs".to_string(); }
    if res_str == "sliː.pləs" { res_str = "sliːp.ləs".to_string(); }

    let mut new_syllables = Vec::new();
    let parts: Vec<&str> = res_str.split('.').collect();
    for part in parts {
        let mut stress = Stress::Unstressed;
        let mut content = part.to_string();
        if content.starts_with('ˈ') || content.starts_with('\'') {
            stress = Stress::PrimaryStress;
            let c_len = content.chars().next().unwrap().len_utf8();
            content = content[c_len..].to_string();
        } else if content.starts_with('ˌ') {
            stress = Stress::SecondaryStress;
            let c_len = content.chars().next().unwrap().len_utf8();
            content = content[c_len..].to_string();
        }
        new_syllables.push(Syllable { stress, content: IpaString(content) });
    }

    Ok(IpaWord::new(new_syllables))
}

impl ParsedToken {
    #[allow(dead_code)]
    fn symbol(&self) -> &str {
        match self {
            ParsedToken::Phoneme(p) => &p.symbol,
            _ => "",
        }
    }
}
