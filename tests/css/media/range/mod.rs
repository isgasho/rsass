//! Tests auto-converted from "sass-spec/spec/css/media/range"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/css/media/range/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod invalid_binary_operator {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "before_colon", error tests are not supported yet.

        // Ignoring "eq", error tests are not supported yet.

        // Ignoring "gt", error tests are not supported yet.

        // Ignoring "gte", error tests are not supported yet.

        // Ignoring "in_subexpression", error tests are not supported yet.

        // Ignoring "lt", error tests are not supported yet.

        // Ignoring "lte", error tests are not supported yet.
    }
    mod invalid_comparison {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "gte", error tests are not supported yet.

        // Ignoring "lte", error tests are not supported yet.

        // Ignoring "range_gte", error tests are not supported yet.
    }
    mod mismatched_range {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "gt_lt", error tests are not supported yet.

        // Ignoring "gte_lte", error tests are not supported yet.

        // Ignoring "lt_gt", error tests are not supported yet.

        // Ignoring "lte_gte", error tests are not supported yet.
    }
}

// From "sass-spec/spec/css/media/range/from_interpolation.hrx"
#[test]
fn from_interpolation() {
    assert_eq!(
        rsass(
            "// Range syntax that\'s generated by interpolation is allowed. This falls out of\
            \n// the general property that CSS media features are parsed as\
            \n// <declaration-value>s.\
            \n\
            \n@media #{\"(100px < width < 500px)\"} {a {interpolation: total}}\
            \n@media (#{\"100px < width < 500px\"}) {a {interpolation: in-parens}}\
            \n@media (#{\"100px < width\"} < 500px) {a {interpolation: left}}\
            \n@media (100px < #{\"width < 500px\"}) {a {interpolation: right}}\
            \n"
        )
        .unwrap(),
        "@media (100px < width < 500px) {\
        \n  a {\
        \n    interpolation: total;\
        \n  }\
        \n}\
        \n@media (100px < width < 500px) {\
        \n  a {\
        \n    interpolation: in-parens;\
        \n  }\
        \n}\
        \n@media (100px < width < 500px) {\
        \n  a {\
        \n    interpolation: left;\
        \n  }\
        \n}\
        \n@media (100px < width < 500px) {\
        \n  a {\
        \n    interpolation: right;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/media/range/static.hrx"
#[test]
fn test_static() {
    assert_eq!(
        rsass(
            "// Plain CSS examples of the media query range syntax.\
            \n\
            \n// Test all the comparison operators.\
            \n@media (height < 600px) {a {b: c}}\
            \n@media (height <= 600px) {a {b: c}}\
            \n@media (height = 600px) {a {b: c}}\
            \n@media (height >= 600px) {a {b: c}}\
            \n@media (height > 600px) {a {b: c}}\
            \n\
            \n// Test all the range operators.\
            \n@media (10px < width < 15px) {a {b: c}}\
            \n@media (10px < width <= 15px) {a {b: c}}\
            \n@media (10px <= width < 15px) {a {b: c}}\
            \n@media (10px <= width <= 15px) {a {b: c}}\
            \n@media (10px > width > 15px) {a {b: c}}\
            \n@media (10px > width >= 15px) {a {b: c}}\
            \n@media (10px >= width > 15px) {a {b: c}}\
            \n@media (10px >= width >= 15px) {a {b: c}}\
            \n\
            \n// Ratio syntax should fall out from Sass\'s existing rules for handling `/`.\
            \n@media (device-aspect-ratio > 3/2) {a {b: c}}\
            \n@media (3/2 < device-aspect-ratio < 16/9) {a {b: c}}\
            \n"
        )
        .unwrap(),
        "@media (height < 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height <= 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height = 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height >= 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height > 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px < width < 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px < width <= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px <= width < 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px <= width <= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px > width > 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px > width >= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px >= width > 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px >= width >= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (device-aspect-ratio > 3/2) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (3/2 < device-aspect-ratio < 16/9) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/media/range/with_expressions.hrx"
#[test]
#[ignore] // wrong result
fn with_expressions() {
    assert_eq!(
        rsass(
            "// Media query range syntax with SassScript expressions.\
            \n\
            \n// Single comparisons.\
            \n$width: width;\
            \n@media ($width < 600px) {a {dynamic: left}}\
            \n@media (width < 500px + 100px) {a {dynamic: right}}\
            \n@media ($width < 500px + 100px) {a {dynamic: both}}\
            \n@media ($width = 500px + 100px) {a {separator: equals}}\
            \n\
            \n// Ranges.\
            \n@media (50px + 50px < width < 600px) {a {dynamic: left}}\
            \n@media (100px < $width < 600px) {a {dynamic: middle}}\
            \n@media (100px < width < 500px + 100px) {a {dynamic: right}}\
            \n@media (50px + 50px < $width < 500px + 100px) {a {dynamic: all}}\
            \n\
            \n// Comparison operators are allowed as long as they\'re not at the top level.\
            \n@media (width < (1 < 2)) {a {comparison: in-operator}}\
            \n@media (width < if(1 < 2, 500px, 600px)) {a {comparison: in-function}}\
            \n@media (width < [1 < 2]) {a {comparison: in-square-brackets}}\
            \n"
        )
        .unwrap(),
        "@media (width < 600px) {\
        \n  a {\
        \n    dynamic: left;\
        \n  }\
        \n}\
        \n@media (width < 600px) {\
        \n  a {\
        \n    dynamic: right;\
        \n  }\
        \n}\
        \n@media (width < 600px) {\
        \n  a {\
        \n    dynamic: both;\
        \n  }\
        \n}\
        \n@media (width = 600px) {\
        \n  a {\
        \n    separator: equals;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: left;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: middle;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: right;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: all;\
        \n  }\
        \n}\
        \n@media (width < true) {\
        \n  a {\
        \n    comparison: in-operator;\
        \n  }\
        \n}\
        \n@media (width < 500px) {\
        \n  a {\
        \n    comparison: in-function;\
        \n  }\
        \n}\
        \n@media (width < [true]) {\
        \n  a {\
        \n    comparison: in-square-brackets;\
        \n  }\
        \n}\
        \n"
    );
}
