#![cfg(test)]

#[macro_use]
mod macros;

test!(
    if_true,
    "a {\n  color: if(true, 1, 2)\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    if_named_args,
    "a {\n  color: if($condition: true, $if-true: 1, $if-false: 2)\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(
    if_false,
    "a {\n  color: if(false, 1, 2);\n}\n",
    "a {\n  color: 2;\n}\n"
);
test!(
    feature_exists_at_error_dbl_quoted,
    "a {\n  color: feature-exists(\"at-error\")\n}\n",
    "a {\n  color: true;\n}\n"
);
test!(
    feature_exists_at_error_sgl_quoted,
    "a {\n  color: feature-exists('at-error')\n}\n",
    "a {\n  color: true;\n}\n"
);
test!(
    feature_exists_at_error_no_quotes,
    "a {\n  color: feature-exists(at-error)\n}\n",
    "a {\n  color: true;\n}\n"
);
test!(
    feature_exists_at_error_named_arg,
    "a {\n  color: feature-exists($feature: at-error)\n}\n",
    "a {\n  color: true;\n}\n"
);
test!(
    unit_px,
    "a {\n  color: unit(1px)\n}\n",
    "a {\n  color: \"px\";\n}\n"
);
test!(
    unit_none,
    "a {\n  color: unit(1)\n}\n",
    "a {\n  color: \"\";\n}\n"
);
test!(
    unit_non_numeric,
    "a {\n  color: unit(red)\n}\n",
    "a {\n  color: \"\";\n}\n"
);
test!(
    unit_named_args,
    "a {\n  color: unit($number: 1px)\n}\n",
    "a {\n  color: \"px\";\n}\n"
);
test!(
    type_of_number,
    "a {\n  color: type-of(1)\n}\n",
    "a {\n  color: number;\n}\n"
);
test!(
    type_of_number_unit,
    "a {\n  color: type-of(1px)\n}\n",
    "a {\n  color: number;\n}\n"
);
test!(
    type_of_unquoted,
    "a {\n  color: type-of(foo)\n}\n",
    "a {\n  color: string;\n}\n"
);
test!(
    type_of_sgl_unquoted,
    "a {\n  color: type-of('red')\n}\n",
    "a {\n  color: string;\n}\n"
);
test!(
    type_of_dbl_unquoted,
    "a {\n  color: type-of(\"red\")\n}\n",
    "a {\n  color: string;\n}\n"
);
// test!(
//     type_of_3_hex_color,
//     "a {\n  color: type-of(#fff)\n}\n",
//     "a {\n  color: color;\n}\n"
// );
// test!(
//     type_of_6_hex_color,
//     "a {\n  color: type-of(#ffffff)\n}\n",
//     "a {\n  color: color;\n}\n"
// );
// test!(
//     type_of_named_color,
//     "a {\n  color: type-of(red)\n}\n",
//     "a {\n  color: color;\n}\n"
// );
// test!(
//     type_of_spaced_list,
//     "a {\n  color: type-of(1 2 3)\n}\n",
//     "a {\n  color: list;\n}\n"
// );
test!(
    type_of_true,
    "a {\n  color: type-of(true)\n}\n",
    "a {\n  color: bool;\n}\n"
);
test!(
    type_of_false,
    "a {\n  color: type-of(false)\n}\n",
    "a {\n  color: bool;\n}\n"
);
test!(
    type_of_null,
    "a {\n  color: type-of(null)\n}\n",
    "a {\n  color: null;\n}\n"
);
test!(
    type_of_ident_plus_ident,
    "a {\n  color: type-of(hi + bye)\n}\n",
    "a {\n  color: string;\n}\n"
);
