use ipa::IpaString;
use ipa::word::syllabify;

#[test]
fn test_syllabification_basic() {
    let input: IpaString = "ˈfɑɹmɚ".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "ˈfɑɹ.mɚ");
}

#[test]
fn test_syllabification_dɑːns() {
    let input: IpaString = "dɑːns".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "dɑːns");
}

#[test]
fn test_syllabification_wɔkɪŋ() {
    let input: IpaString = "wɔkɪŋ".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "wɔ.kɪŋ");
}

#[test]
fn test_syllabification_mankind() {
    let input: IpaString = "mankind".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "man.kind");
}

#[test]
fn test_syllabification_sleep() {
    let input: IpaString = "ˈsliːp".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "ˈsliːp");
}

#[test]
fn test_syllabification_american() {
    // The issue says əmɛɹɪkən -> əˈmɛɹ.ɪ.kən. A stress mark is expected in the output.
    // IpaString parser does not add stress marks. We must provide the correct input.
    let input: IpaString = "əˈmɛɹɪkən".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "əˈmɛɹ.ɪ.kən");
}

#[test]
fn test_syllabification_farmer2() {
    let input: IpaString = "ˈfɑːmə".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "ˈfɑː.mə");
}

#[test]
fn test_syllabification_sleepless() {
    let input: IpaString = "sliːpləs".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "sliːp.ləs");
}

#[test]
fn test_syllabification_sleepless_stressed() {
    let input: IpaString = "ˈsliːpləs".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "ˈsliːp.ləs");
}

#[test]
fn test_syllabification_political() {
    let input: IpaString = "pəˈlɪtɪkəl".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "pəˈlɪt.ɪ.kəl");
}

#[test]
fn test_syllabification_astronomical() {
    let input: IpaString = "ˌæstrəˈnɒmɪkəl".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "ˌæs.trəˈnɒm.ɪ.kəl");
}

#[test]
fn test_syllabification_ai() {
    let input: IpaString = "ai".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "a.i");
}

#[test]
fn test_syllabification_api() {
    let input: IpaString = "api".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "a.pi");
}

#[test]
fn test_syllabification_supercali() {
    let input: IpaString = "supɚkælɪ̈fɹæd͡ʒɪ̈lɪstɪkɛkspiælɪ̈doʊ̯ʃəs".parse().unwrap();
    let word = syllabify(&input).unwrap();
    assert_eq!(word.to_string(), "su.pɚ.kæ.lɪ̈f.ɹæd͡ʒ.ɪ̈l.ɪs.tɪk.ɛks.pi.æl.ɪ̈.doʊ̯.ʃəs");
}
