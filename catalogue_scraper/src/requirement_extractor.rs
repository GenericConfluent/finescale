use regex::{Regex, RegexSet};
use std::cmp;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequirementKind {
    Prerequisite,
    Corequisite,
}

pub fn extract_requirement_strings(s: &str) -> Vec<(RequirementKind, &str)> {
    const NO_PREREQUISITE_REGEX: &str = r"(?x)(?i)
        no \s (?<kind>pre-?requisite) ((\(s\)) | s)?
    ";

    const PREREQUISITE_REGEX: &str = r"(?x)(?i)
        (^|\.|:)\s*
        (?<kind>(pre-?requisite ((\(s\)) | s)?
            | préalable (\s? (\(s\)) | s)?)
            | prérequis)
        \s?
        (:|;|.)?
        \s+
        (?<data>[^.]*?)
        (\.|$)
    ";

    const COREQUISITE_REGEX: &str = r"(?x)(?i)
        (^|\.|:)\s*
        (?<kind>(pre-(and/or)? \s or \s | pre-?requisite((\(s\)) | s)? \s or \s)?co-?requisite((\(s\)) | s)?
            | (préalable (\s? (\(s\)) | s)? \s ou \s)?concomitant(\s? (\(s\)) | s)?)
        \s?
        (:|;|.)?
        \s+
        (?<data>[^.]*)?
        (\.|$)
    ";

    const EXPRESSIONS: [&str; 3] = [COREQUISITE_REGEX, PREREQUISITE_REGEX, NO_PREREQUISITE_REGEX];

    fn get_compiled_regexes() -> &'static [Regex; 3] {
        static STATIC: OnceLock<[Regex; 3]> = OnceLock::new();
        STATIC.get_or_init(|| EXPRESSIONS.map(|expr| Regex::new(expr).unwrap()))
    }

    let kinds = [
        Some(RequirementKind::Corequisite),
        Some(RequirementKind::Prerequisite),
        None,
    ];
    let regexes = get_compiled_regexes();
    let set = RegexSet::new(EXPRESSIONS).unwrap();

    let mut i = 0;
    let mut results = Vec::new();

    // Note: matches(&s[i..]) is intentionally used instead of matches_at(s, i).
    //
    // Previously, this code used matches_at, but it fails for this case:
    // ```
    // Prerequisite: PHYS 124 (see Note following) or 144. Corequisite: MATH 118 or 146.
    // ^ first character to match
    // ```
    //
    // The first prerequisite is matched first:
    // ```
    // Prerequisite: PHYS 124 (see Note following) or 144. Corequisite: MATH 118 or 146.
    // ^-------------------------------------------------^
    //                                                    ^ new first character to match
    // ```
    //
    // Now, presumably, we'd have the same behaviour as matching the string
    // ```
    //  Corequisite: MATH 118 or 146.
    // ```
    //
    // However, the regular expressions start with (^|\.|:)\s*` to make sure we're
    // only matching at the start of a sentence.
    //
    // The `^` rule doesn't match, because we're not actually at the start of the
    // string. The `\.` rule doesn't match, because we've removed the context before
    // the string. If the regex crate had lookbehind, we could probably get around
    // this in the expression itself. Instead, we intentionally treat the rest of
    // the string as a completely new string.
    //
    // As I'm writing this, I'm realizing that this wouldn't be a problem if we had
    // used Regex instead of RegexSet, because Regex has `find_iter` which returns
    // an iterator over non-overlapping matches. I think finding a way to do
    // something similar for a RegexSet will be the proper solution here.

    while let Some((first_match, first_match_kind)) = set
        .matches(&s[i..])
        .into_iter()
        .filter_map(|m| Some((regexes[m].captures(&s[i..])?, m)))
        .min_by_key(|&(ref captures, m)| {
            (
                captures.get(0).unwrap().start(),
                cmp::Reverse(captures.name("kind").unwrap().len()),
                m,
            )
        })
    {
        if let Some(kind) = kinds[first_match_kind] {
            let data = first_match.name("data").unwrap();
            let text = &s[i..][data.start()..data.end()];
            results.push((kind, text));
        }
        i += first_match.get(0).unwrap().end();
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};
    use std::fmt::Write;

    #[track_caller]
    fn check_no_requirements(s: &str) {
        let results = extract_requirement_strings(s);
        assert_eq!(results, []);
    }

    #[track_caller]
    fn check_prereq(s: &str, output: Expect) {
        check_requirement(s, RequirementKind::Prerequisite, output);
    }

    #[track_caller]
    fn check_coreq(s: &str, output: Expect) {
        check_requirement(s, RequirementKind::Corequisite, output);
    }

    #[track_caller]
    fn check_requirement(s: &str, requirement_kind: RequirementKind, output: Expect) {
        let results = extract_requirement_strings(s);
        let &[(result_kind, result_text)] = results.as_slice() else {
            panic!("Wanted one requirement: {results:?}, {s:?}")
        };
        assert_eq!(result_kind, requirement_kind, "{result_kind:?}");
        output.assert_eq(result_text);
    }

    #[track_caller]
    fn check_requirements(s: &str, output: Expect) {
        let mut out = String::new();
        for (kind, text) in extract_requirement_strings(s) {
            writeln!(&mut out, "{kind:?}: {text:?}").unwrap();
        }
        output.assert_eq(&out);
    }

    #[test]
    fn extract_no_prerequisite() {
        check_no_requirements("sciences. No prerequisite. May");
        check_no_requirements("and has no prerequisites. This");
        check_no_requirements("Asia. No Prerequisites. Taught");
    }

    #[test]
    fn extract_normal_prerequisites() {
        check_prereq(
            "system. Prerequisites: Math 30 or 31. Note:",
            expect!["Math 30 or 31"],
        );
    }

    #[test]
    fn extract_prerequisite_with_period_separator() {
        check_prereq(
            "included. Prerequisite. Mathematics 30-1. Note:",
            expect!["Mathematics 30-1"],
        );
    }

    #[test]
    fn extract_prerequisite_with_semicolon_separator() {
        check_prereq(
            "practices. Prerequisites; EASIA 101 and 3 units in EASIA at a senior level, or \
             consent of Department.",
            expect!["EASIA 101 and 3 units in EASIA at a senior level, or consent of Department"],
        );
    }

    #[test]
    fn extract_prerequisite_without_separator() {
        check_prereq(
            "Prerequisite MATH 227, or both MATH 225 and 228",
            expect!["MATH 227, or both MATH 225 and 228"],
        );
    }

    #[test]
    fn extract_prerequisite_with_space_before_colon_separator() {
        check_prereq(
            "Préalable(s) : un cours de niveau 200 en biologie (ZOOL 250 et IMIN ou IMINE 200 \
             recommandés).",
            expect![
                "un cours de niveau 200 en biologie (ZOOL 250 et IMIN ou IMINE 200 recommandés)"
            ],
        );
    }

    #[test]
    fn extract_prerequisite_with_rare_french_name() {
        check_prereq(
            "comptables. Prérequis: ADMI 311, 322 ou ACCTG 311, 322. Ce",
            expect!["ADMI 311, 322 ou ACCTG 311, 322"],
        );
    }

    #[test]
    fn extract_corequisite_with_sentence_following() {
        check_coreq(
            "Pre- or co-requisites: ECON 101 and 102.",
            expect!["ECON 101 and 102"],
        );
        check_coreq(
            "statements. Pre- or co-requisites: ECON 101 and 102. Students may not receive credit \
             for both ACCTG 211 and ACCTG 311.",
            expect!["ECON 101 and 102"],
        );
    }

    #[test]
    fn extract_prerequisite_with_extra_whitespace() {
        check_prereq("Prerequisite:   LAW 524", expect!["LAW 524"]);
    }

    #[test]
    fn extract_prerequisite_with_space_before_conditional_french_plural_marking() {
        check_prereq(
            "Préalable (s) : PSYCE 239 ou équivalent.",
            expect!["PSYCE 239 ou équivalent"],
        );
    }

    #[test]
    fn extract_corequisite_with_prerequisite_or_corequisite_wording() {
        check_coreq(
            "Prerequisite or corequisite: A Calculus IV course.",
            expect!["A Calculus IV course"],
        );
        check_coreq(
            "Prerequisite or Corequisite: ECON 481 and 482.",
            expect!["ECON 481 and 482"],
        );
        check_coreq(
            "Prerequisites or corequisites: DES 493 and consent of Department.",
            expect!["DES 493 and consent of Department"],
        );
        check_coreq(
            "Corequisite or prerequisite: Music 121 or 125, or 124, or consent of Department. ",
            expect!["or prerequisite: Music 121 or 125, or 124, or consent of Department"],
        );
        check_coreq(
            "Pour les étudiants du BEd/Ad : Préalable ou concomitant : EDU S 101. ",
            expect!["EDU S 101"],
        );
    }

    #[test]
    fn extract_sentence_end_lack_of_prerequisites() {
        check_no_requirements(
            "Note: Consult the Department of Psychology's website for the specific topic(s) \
             offered each year and any additional prerequisites. [Faculty of Arts]",
        );
    }

    #[test]
    fn extract_requirements_with_extra_information_surrounding() {
        check_requirements(
            "Prerequisites: Mathematics 30-1 and Physics 30. Mathematics 31 is strongly \
             recommended. Corequisites: MATH 117 or 144. Note: MATH 113 or 114 is not acceptable \
             as a co-requisite but may be used as a pre-requisite in place of MATH 117 or 144. \
             Note: Credit may be obtained for only one of PHYS 124, 144, EN PH 131 or SCI 100. ",
            expect![[r#"
                Prerequisite: "Mathematics 30-1 and Physics 30"
                Corequisite: "MATH 117 or 144"
            "#]],
        );
        check_coreq(
            " Corequisite: MATH 118 or 146. Note: MATH 115 is not acceptable as a co-requisite \
             but may be used as a pre-requisite in place of MATH 118 or 146. Note: Credit may be \
             obtained for only one of PHYS 126, 130, 146 or SCI 100. Note: To proceed to PHYS 146 \
             after taking PHYS 124, it is strongly recommended that a minimum grade of B- be \
             achieved in PHYS 124.",
            expect!["MATH 118 or 146"],
        );
        check_requirements(
            "Prerequisite: PHYS 124 (see Note following) or 144. Corequisite: MATH 118 or 146. \
             Note: MATH 115 is not acceptable as a co-requisite but may be used as a \
             pre-requisite in place of MATH 118 or 146. Note: Credit may be obtained for only one \
             of PHYS 126, 130, 146 or SCI 100. Note: To proceed to PHYS 146 after taking PHYS \
             124, it is strongly recommended that a minimum grade of B- be achieved in PHYS 124. ",
            expect![[r#"
                Prerequisite: "PHYS 124 (see Note following) or 144"
                Corequisite: "MATH 118 or 146"
            "#]],
        );
    }

    #[test]
    fn fixme_extract_prerequisite_with_internal_punctuation_trees() {
        // This might be a hard one
        // Luckily it probably isn't very common and it doesn't even matter here
        check_prereq(
            "Prerequisite: AUEAP 145 or EAP 145 or equivalent (i.e., other L2/ESL students who \
             were not required to take the Bridging Program).",
            expect!["AUEAP 145 or EAP 145 or equivalent (i"],
        );
    }

    #[test]
    fn fixme_extract_negative_requirement() {
        // This should be extracted as a new requirement kind.
        check_no_requirements("Not open to students with C LIT 100.");
        // This one should be a different kind than the one above.
        check_no_requirements(
            "Note: Credit cannot be obtained for more than one of ENGG 310, ENGG 401, ENG M 310, \
             or ENG M 401.",
        );
    }
}
