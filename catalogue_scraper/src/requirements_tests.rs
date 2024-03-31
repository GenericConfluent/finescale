use crate::requirements;
use crate::requirements_actions::Expr;
use expect_test::{expect, Expect};
use rustemo::Parser as _;

#[track_caller]
fn parse_requirement(s: &str) -> Result<Expr, String> {
    let parser = requirements::RequirementsParser::new();
    match parser.parse(s) {
        Ok(forest) => forest
            .get_first_tree()
            .map(|tree| {
                let mut builder = requirements::DefaultBuilder::new();
                let expr = tree.build(&mut builder);
                expr
            })
            .ok_or_else(|| "no tree".to_owned()),
        Err(e) => Err(e.to_string()),
    }
}

#[track_caller]
fn check(input: &str, output: Expect) {
    let res = parse_requirement(input);
    match res {
        Ok(req) => {
            output.assert_eq(&format!("{req:#?}"));
        }
        Err(e) => {
            output.assert_eq(&format!("Error: {e}"));
        }
    }
}

#[test]
fn parse_simple_course_requirement() {
    check("CMPUT 174", expect!["(CMPUT 174)"]);
}

#[test]
fn parse_any_of_course_requirements() {
    check(
        "CMPUT 174 or CMPUT 274",
        expect!["(any (CMPUT 174) (CMPUT 274))"],
    );
    check(
        "CMPUT 174, CMPUT 274, CMPUT 175 or CMPUT 275",
        expect!["(any (CMPUT 174) (CMPUT 274) (CMPUT 175) (CMPUT 275))"],
    );
}

#[test]
fn parse_any_of_course_requirements_with_prefix() {
    check(
        "one of CMPUT 175 or 275",
        expect!["(any (CMPUT 175) (CMPUT 275))"],
    );
    check(
        "one of CMPUT 175 or CMPUT 275",
        expect!["(any (CMPUT 175) (CMPUT 275))"],
    );
    check(
        "any of CMPUT 175 or CMPUT 275",
        expect!["(any (CMPUT 175) (CMPUT 275))"],
    );
}

#[test]
fn parse_any_of_course_requirements_without_repeated_topic() {
    check("CMPUT 174 or 274", expect!["(any (CMPUT 174) (CMPUT 274))"]);
}

#[test]
fn parse_all_of_course_requirements() {
    check(
        "ECON 101 and ECON 102",
        expect!["(all (ECON 101) (ECON 102))"],
    );
}

#[test]
fn parse_all_of_course_requirements_without_repeated_topic() {
    check("ECON 101 and 102", expect!["(all (ECON 101) (ECON 102))"]);
}

#[test]
fn parse_top_level_comma_separated_list() {
    check("ACCTG 614, FIN 501", expect!["(all (ACCTG 614) (FIN 501))"]);
    check(
        "ACCTG 614, ACCTG 610, FIN 501",
        expect!["(all (ACCTG 614) (ACCTG 610) (FIN 501))"],
    );
    check(
        "ACCTG 614 or 610, FIN 501 or 503",
        expect!["(all (any (ACCTG 614) (ACCTG 610)) (any (FIN 501) (FIN 503)))"],
    );
}

#[test]
fn misc_working() {
    check(
        "ECON 109 and MATH 156",
        expect!["(all (ECON 109) (MATH 156))"],
    );
    check(
        "ECON 109, and MATH 156",
        expect!["(all (ECON 109) (MATH 156))"],
    );
    check(
        "ECON 109, ECON 110 and MATH 156",
        expect!["(all (ECON 109) (ECON 110) (MATH 156))"],
    );
    check(
        "ECON 109, ECON 110, and MATH 156",
        expect!["(all (ECON 109) (ECON 110) (MATH 156))"],
    );
    check("MATH 156 or equivalent", expect!["(MATH 156)"]);
    check(
        "ECON 109, and MATH 156 or equivalent",
        expect!["(all (ECON 109) (MATH 156))"],
    );
    check(
        "ECON 281, 282 and 299",
        expect!["(all (ECON 281) (ECON 282) (ECON 299))"],
    );
    check(
        "one of CMPUT 175 or CMPUT 275",
        expect!["(any (CMPUT 175) (CMPUT 275))"],
    );
    check(
        "ECON 281, 282 and 299",
        expect!["(all (ECON 281) (ECON 282) (ECON 299))"],
    );
    check(
        "one of PHYS 124, PHYS 144, or EN PH 131",
        expect!["(any (PHYS 124) (PHYS 144) (EN PH 131))"],
    );
    check(
        "ECON 281, and MATH 156",
        expect!["(all (ECON 281) (MATH 156))"],
    );
    check(
        "ACCTG 614 or 610, FIN 501 or 503",
        expect!["(all (any (ACCTG 614) (ACCTG 610)) (any (FIN 501) (FIN 503)))"],
    );
    check("PL SC 345", expect!["(PL SC 345)"]);
    check(
        "ECON 281 and ECON 282, and MATH 156",
        expect!["(all (all (ECON 281) (ECON 282)) (MATH 156))"],
    );

    check(
        "ECON 274 or 274, CMPUT 274",
        expect!["(all (any (ECON 274) (ECON 274)) (CMPUT 274))"],
    );

    check("ECON 281 or 282", expect!["(any (ECON 281) (ECON 282))"]);

    check("CMPUT 274", expect!["(CMPUT 274)"]);
    check(
        "ECON 281 or 282, CMPUT 274",
        expect!["(all (any (ECON 281) (ECON 282)) (CMPUT 274))"],
    );

    check(
        "ECON 281, 282 or 299",
        expect!["(any (ECON 281) (ECON 282) (ECON 299))"],
    );
    check(
        "ECON 281, 282 and 299, and MATH 156",
        expect!["(all (all (ECON 281) (ECON 282) (ECON 299)) (MATH 156))"],
    );
    check(
        "ECON 109, ECON 281, 282 and 299, and MATH 156",
        expect!["(all (ECON 109) (all (ECON 281) (ECON 282) (ECON 299)) (MATH 156))"],
    );

    check(
        "one of PHYS 126, PHYS 146, or PHYS 130 and PHYS 208 or 271",
        expect!["(all (any (PHYS 126) (PHYS 146) (PHYS 130)) (any (PHYS 208) (PHYS 271)))"],
    );
    check(
        "PHYS 130 and PHYS 208 or 271",
        expect!["(all (PHYS 130) (any (PHYS 208) (PHYS 271)))"],
    );
    check(
        "one of PHYS 124, PHYS 144, or EN PH 131",
        expect!["(any (PHYS 124) (PHYS 144) (EN PH 131))"],
    );

    check(
        "one of PHYS 126, PHYS 146, or PHYS 130 and PHYS 208 or 271",
        expect!["(all (any (PHYS 126) (PHYS 146) (PHYS 130)) (any (PHYS 208) (PHYS 271)))"],
    );

    check(
        "one of PHYS 126, PHYS 146, or PHYS 130 and PHYS 208 or 271",
        expect!["(all (any (PHYS 126) (PHYS 146) (PHYS 130)) (any (PHYS 208) (PHYS 271)))"],
    );

    check(
        "MATH 115, 118, 136, 146 or 156, and one of PHYS 124, PHYS 144, or EN PH 131, and one of \
         PHYS 126, PHYS 146, or PHYS 130 and PHYS 208 or 271",
        expect![
            "(all (all (any (MATH 115) (MATH 118) (MATH 136) (MATH 146) (MATH 156)) (PHYS 124)) \
             (any (PHYS 144) (EN PH 131)) (all (any (PHYS 126) (PHYS 146) (PHYS 130)) (any (PHYS \
             208) (PHYS 271))))"
        ],
    );
    check(
        "MATH 115, 118, 136, 146 or 156, and one of PHYS 124, PHYS 144, or EN PH 131, and one of \
         PHYS 126, PHYS 146, or PHYS 130 and PHYS 208 or 271",
        expect![
            "(all (all (any (MATH 115) (MATH 118) (MATH 136) (MATH 146) (MATH 156)) (PHYS 124)) \
             (any (PHYS 144) (EN PH 131)) (all (any (PHYS 126) (PHYS 146) (PHYS 130)) (any (PHYS \
             208) (PHYS 271))))"
        ],
    );
    check(
        "ECON 109, ECON 281, 282 and 299 or equivalent",
        expect!["(all (ECON 109) (all (ECON 281) (ECON 282) (ECON 299)))"],
    );
    check(
        "ECON 281, 282 and 299 or equivalent",
        expect!["(all (ECON 281) (ECON 282) (ECON 299))"],
    );

    check(
        "ECON 109, ECON 281, 282 and 299 or equivalent, and MATH 156 or equivalent",
        expect!["(all (ECON 109) (all (ECON 281) (ECON 282) (ECON 299)) (MATH 156))"],
    );
    check(
        "ECON 281, 282 and 299 or equivalent, and MATH 156 or equivalent",
        expect!["(all (all (ECON 281) (ECON 282) (ECON 299)) (MATH 156))"],
    );
    check("IMIN 200 and consent of instructor", expect!["(IMIN 200)"]);
    check(
        "ACCTG 211 or 311 and ACCTG 222 or 322",
        expect!["(all (any (ACCTG 211) (ACCTG 311)) (any (ACCTG 222) (ACCTG 322)))"],
    );
    check("EASIA 323", expect!["(EASIA 323)"]);
    check(
        "EASIA 323 or EASIA 325",
        expect!["(any (EASIA 323) (EASIA 325))"],
    );
    check(
        "EASIA 323, or EASIA 325",
        expect!["(any (EASIA 323) (EASIA 325))"],
    );
    check(
        "RELIG 240, RELIG 343, EASIA 223, EASIA 323, or EASIA 325",
        expect!["(any (RELIG 240) (RELIG 343) (EASIA 223) (EASIA 323) (EASIA 325))"],
    );
    check(
        "RELIG 240, RELIG 343, EASIA 223, EASIA 323, and EASIA 325",
        expect!["(all (RELIG 240) (RELIG 343) (EASIA 223) (EASIA 323) (EASIA 325))"],
    );
    check(
        "RELIG 240, RELIG 343, EASIA 223, EASIA 323, EASIA 325 or consent of Instructor",
        expect!["(any (RELIG 240) (RELIG 343) (EASIA 223) (EASIA 323) (EASIA 325))"],
    );
    check(
        "RELIG 240, RELIG 343, EASIA 223, EASIA 323, EASIA 325 or consent of Instructor",
        expect!["(any (RELIG 240) (RELIG 343) (EASIA 223) (EASIA 323) (EASIA 325))"],
    );
    check(
        "one of RELIG 240, RELIG 343, EASIA 223, EASIA 323, EASIA 325 or consent of Instructor",
        expect!["(any (RELIG 240) (RELIG 343) (EASIA 223) (EASIA 323) (EASIA 325))"],
    );
    check(
        "BIOCH 200, PL SC 345, or consent of instructor",
        expect!["(any (BIOCH 200) (PL SC 345))"],
    );
    check(
        "One of AUHIS 366, 369, 372, 378",
        expect!["(any (AUHIS 366) (AUHIS 369) (AUHIS 372) (AUHIS 378))"],
    );
    check("ECE 202 or E E 240", expect!["(any (ECE 202) (E E 240))"]);
    check(
        "CMPUT 204, one of CMPUT 229, E E 380 or ECE 212 and one of MATH 225, 227, or 228 or \
         consent of the instructor",
        expect![
            "(all (any (CMPUT 204) (CMPUT 229) (E E 380) (ECE 212)) (any (MATH 225) (MATH 227) \
             (MATH 228)))"
        ],
    );
    check(
        "MATH 100/114/117/134/144",
        expect!["(any (MATH 100) (MATH 114) (MATH 117) (MATH 134) (MATH 144))"],
    );
    check(
        "MATH 100/114/117/134/144, PHYS 124/144 or EN PH131",
        expect![
            "(any (any (MATH 100) (MATH 114) (MATH 117) (MATH 134) (MATH 144)) (any (PHYS 124) \
             (PHYS 144)) (EN PH 131))"
        ],
    );
    check(
        "MATH 100/114/117/134/144, PHYS 124/144 or EN PH 131",
        expect![
            "(any (any (MATH 100) (MATH 114) (MATH 117) (MATH 134) (MATH 144)) (any (PHYS 124) \
             (PHYS 144)) (EN PH 131))"
        ],
    );
    check(
        "AUCSC 112, or AUCSC 211 or AUSCI 235",
        expect!["(any (AUCSC 112) (any (AUCSC 211) (AUSCI 235)))"],
    );
    check(
        "CMPUT 174, 274 or consent of the instructor",
        expect!["(any (CMPUT 174) (CMPUT 274))"],
    );
    check(
        "FIN 201 or 301, and a minimum grade of C- in ACCTG 314 or 414",
        expect!["(all (any (FIN 201) (FIN 301)) (any (ACCTG 314) (ACCTG 414)))"],
    );
    check(
        "ACCTG 414 or 412; FIN 301",
        expect!["(all (any (ACCTG 414) (ACCTG 412)) (FIN 301))"],
    );
    check(
        "CMPUT 174 or 274; one of MATH 100, 114, 117, 134, 144, or 154",
        expect![
            "(all (any (CMPUT 174) (CMPUT 274)) (any (MATH 100) (MATH 114) (MATH 117) (MATH 134) \
             (MATH 144) (MATH 154)))"
        ],
    );
    check(
        "CMPUT 101, 174, 175, 274, or SCI 100",
        expect!["(any (any (CMPUT 101) (CMPUT 174) (CMPUT 175) (CMPUT 274)) (SCI 100))"],
    );
    check(
        "CMPUT 204 and 267; one of MATH 115, 118, 136, 146, or 156",
        expect![
            "(all (all (CMPUT 204) (CMPUT 267)) (any (MATH 115) (MATH 118) (MATH 136) (MATH 146) \
             (MATH 156)))"
        ],
    );
    check(
        "CMPUT 204 or 275; MATH 125; CMPUT 267 or MATH 214; or consent of the instructor",
        expect![
            "(all (any (CMPUT 204) (CMPUT 275)) (any (MATH 125) (any (CMPUT 267) (MATH 214))))"
        ],
    );

    check(
        "CMPUT 115 or 175; one of MATH 100, 113, 114, 117, 134, 144, 154; MATH 125; STAT 141, 151 \
         or 235",
        expect![
            "(all (any (CMPUT 115) (CMPUT 175)) (any (MATH 100) (MATH 113) (MATH 114) (MATH 117) \
             (MATH 134) (MATH 144) (MATH 154)) (MATH 125) (any (STAT 141) (STAT 151) (STAT 235)))"
        ],
    );
    check(
        "CMPUT 175 or 275 and CMPUT 272; one of MATH 100, 113, 114, 117, 134, 144, 154, or SCI 100",
        expect![
            "(all (any (CMPUT 175) (CMPUT 275)) (any (CMPUT 272) (any (MATH 100) (MATH 113) (MATH \
             114) (MATH 117) (MATH 134) (MATH 144) (MATH 154)) (SCI 100)))"
        ],
    );
    check(
        "CMPUT 201 or 275; one of CMPUT 229, E E 380 or ECE 212",
        expect!["(all (any (CMPUT 201) (CMPUT 275)) (any (CMPUT 229) (E E 380) (ECE 212)))"],
    );
}

#[test]
fn misc_broken() {
    check(
        "AUCSC 120, AUMAT 110 or 111 or 116, and AUMAT 120",
        expect![[r#"
            Error: Error at <str>:[1,31]:
            	... 110 or 111 or -->116, and AUMAT ...
            	Expected EnOrAlternativeOption."#]],
    );
    check(
        "French 30 ou l'équivalent, ou FRANC 101 ou FREN 100 ou 111/112",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->French 30 ou l'...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "CMPUT 174 or 274, or consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,18]:
            	...UT 174 or 274, -->or consent of t...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "Any introductory-level Computing Science course, plus knowledge of introductory-level \
         MATH and STAT; or consent of the instructor or SCI 100",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->Any introductor...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "Any 100-level course",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->Any 100-level c...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "Second-year standing",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->Second-year sta...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "one of STAT 141, 151, 235, or 265, or SCI 151",
        expect![[r#"
            Error: Error at <str>:[1,35]:
            	..., 235, or 265, -->or SCI 151...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "CMPUT 175 or 275; CMPUT 272; MATH 125 or 127; one of STAT 141, 151, 235, or 265, or SCI \
         151",
        expect![[r#"
            Error: Error at <str>:[1,81]:
            	..., 235, or 265, -->or SCI 151...
            	Expected one of Number, Topic, OneOf, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Or, Ou."#]],
    );
    check(
        "Math 30 or 31",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->Math 30 or 31...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "CMPUT 175 or 274, and 272",
        expect![[r#"
            Error: Error at <str>:[1,22]:
            	...75 or 274, and -->272...
            	Expected one of Number, Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative."#]],
    );
    check(
        "Any introductory-level Computing Science course or SCI 100, and any 200-level course",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->Any introductor...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "CMPUT 204; one of STAT 141, 151, 235 or 265 or SCI 151; one of MATH 225, 227, 228; or \
         consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,47]:
            	... 235 or 265 or -->SCI 151; one of...
            	Expected EnOrAlternativeOption."#]],
    );
    check(
        "one of CMPUT 206, 308, or 411; or consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,31]:
            	..., 308, or 411; -->or consent of t...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "CMPUT 201, 206, MATH 125 or 127, STAT 151 or 265, or consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,50]:
            	...AT 151 or 265, -->or consent of t...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "CMPUT 340 or 418, or ECE 240",
        expect![[r#"
            Error: Error at <str>:[1,18]:
            	...UT 340 or 418, -->or ECE 240...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "CMPUT 201 and 204 or 275; one of CMPUT 229, E E 380 or ECE 212; and STAT 252 or 266",
        expect![[r#"
            Error: Error at <str>:[1,21]:
            	...201 and 204 or -->275; one of CMP...
            	Expected EnOrAlternativeOption."#]],
    );
    check(
        "CMPUT 201 and 204 or 275; one of CMPUT 229, E E 380 or ECE 212, and MATH 125",
        expect![[r#"
            Error: Error at <str>:[1,21]:
            	...201 and 204 or -->275; one of CMP...
            	Expected EnOrAlternativeOption."#]],
    );
    check(
        "CMPUT 204 or 275; MATH 125, 214; one of STAT 141, 151, 235 or 265 or SCI 151",
        expect![[r#"
            Error: Error at <str>:[1,69]:
            	... 235 or 265 or -->SCI 151...
            	Expected EnOrAlternativeOption."#]],
    );
    check(
        "CMPUT 201 or 275, and 204",
        expect![[r#"
            Error: Error at <str>:[1,22]:
            	...01 or 275, and -->204...
            	Expected one of Number, Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative."#]],
    );
    check(
        "any 200-level Computing Science course",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->any 200-level C...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "CMPUT 175 or 275; one of CMPUT 267, 466, or STAT 265; or consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,54]:
            	..., or STAT 265; -->or consent of t...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "CMPUT 201 and 204, or 275; one of CMPUT 229, E E 380 or ECE 212",
        expect![[r#"
            Error: Error at <str>:[1,19]:
            	...T 201 and 204, -->or 275; one of ...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et."#]],
    );
    check(
        "CMPUT 201 and 204, or 275; and CMPUT 291",
        expect![[r#"
            Error: Error at <str>:[1,19]:
            	...T 201 and 204, -->or 275; and CMP...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et."#]],
    );
    check(
        "One of CMPUT 201 or CMPUT 275, CMPUT 204, and any 300-level Computing Science course, or \
         consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,46]:
            	...CMPUT 204, and -->any 300-level C...
            	Expected one of Number, Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative."#]],
    );
    check(
        "CMPUT 301 and 291, or consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,19]:
            	...T 301 and 291, -->or consent of t...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et."#]],
    );
    check(
        "CMPUT 204 or 275, 301; one of CMPUT 340, 418 or equivalent knowledge, and MATH 214",
        expect![[r#"
            Error: Error at <str>:[1,18]:
            	...UT 204 or 275, -->301; one of CMP...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "CMPUT 201 and 204, or 275; one of CMPUT 340, 418 or equivalent knowledge; MATH 214",
        expect![[r#"
            Error: Error at <str>:[1,19]:
            	...T 201 and 204, -->or 275; one of ...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et."#]],
    );
    check(
        "one of CMPUT 229, E E 380 or ECE 212, and a 300-level Computing Science course or \
         consent of the instructor",
        expect![[r#"
            Error: Error at <str>:[1,42]:
            	...r ECE 212, and -->a 300-level Com...
            	Expected one of Number, Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative."#]],
    );
    check(
        "CMPUT 201 or 275; one of CMPUT 340, 418, ECE 240, or equivalent knowledge; one of MATH \
         101, 115, 118, 136, 146 or 156, and one of MATH 102, 125, or 127",
        expect![[r#"
            Error: Error at <str>:[1,64]:
            	... or equivalent -->knowledge; one ...
            	Expected one of STOP, Comma, Semicolon, AndOr, And, Et, Or, Ou, Unless."#]],
    );
    check(
        "any 300-level Computing Science course",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->any 300-level C...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "201 or 275, and any 300-level Computing Science course",
        expect![[r#"
            Error: Error at <str>:[1,0]:
            	...-->201 or 275, and...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, None."#]],
    );
    check(
        "one of CMPUT 340 or 418; one of STAT 141, 151, 235 or 265 or SCI 151; or consent of the \
         instructor",
        expect![[r#"
            Error: Error at <str>:[1,61]:
            	... 235 or 265 or -->SCI 151; or con...
            	Expected EnOrAlternativeOption."#]],
    );
    check(
        "CMPUT 204 and CMPUT 267; any 300-level Computing Science course; and one of MATH 101, \
         115, 118, 136, 146, or 156",
        expect![[r#"
            Error: Error at <str>:[1,25]:
            	...and CMPUT 267; -->any 300-level C...
            	Expected one of Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative, AndOr, And, Et, Number, Or, Ou."#]],
    );
    check(
        "one of CMPUT 191 or 195, one of CMPUT 200, NS 115, or PHIL 385, and three of CMPUT 267, \
         CMPUT 291, CMPUT 328, CMPUT 361, CMPUT 367, CMPUT 461, CMPUT 466, BIOIN 301, BIOIN 401, \
         BIOL 330, BIOL 331, BIOL 332, BIOL 380, BIOL 430, BIOL 471, IMIN 410, MA SC 475, EAS \
         221, EAS 351, EAS 364, EAS 372, GEOPH 426, GEOPH 431, GEOPH 438, PHYS 234, PHYS 295, \
         PHYS 420, STAT 441, STAT 471, STAT 479, AREC 313, REN R 201, REN R 426, REN R 480, FIN \
         440, MARK 312, OM 420, or SEM 330",
        expect![[r#"
            Error: Error at <str>:[1,68]:
            	... PHIL 385, and -->three of CMPUT ...
            	Expected one of Number, Topic, OneOf, Both, Either, AMinimumGradeIn, Alternative."#]],
    );
}
