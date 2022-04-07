//! Tests beginning with `i` simply check that the parser does not panic when parsing them.
//! Tests beginning with `y` should be successfully parsed.
//! Tests beginning with `n` should throw an error but not panic.

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

macro_rules! create_test {
    ($($name:ident: $path:literal,)+) => {
        mod testcases {
            $(
                pub const $name: &[u8] = include_bytes!(concat!("./testcases/", $path));
            )*
        }

        $(
            #[test]
            #[ignore]
            fn $name() {
                use crate::Value;

                if let Ok(string) = std::str::from_utf8(testcases::$name) {
                    let value = Value::parse(string);

                    if $path.starts_with('y') {
                        assert!(value.is_ok());
                    } else if $path.starts_with('n') {
                        assert!(value.is_err());
                    }
                }
            }
        )*
    };
}

create_test! {
    i_number_double_huge_neg_exp: "i_number_double_huge_neg_exp.json",
    i_number_huge_exp: "i_number_huge_exp.json",
    i_number_neg_int_huge_exp: "i_number_neg_int_huge_exp.json",
    i_number_pos_double_huge_exp: "i_number_pos_double_huge_exp.json",
    i_number_real_neg_overflow: "i_number_real_neg_overflow.json",
    i_number_real_pos_overflow: "i_number_real_pos_overflow.json",
    i_number_real_underflow: "i_number_real_underflow.json",
    i_number_too_big_neg_int: "i_number_too_big_neg_int.json",
    i_number_too_big_pos_int: "i_number_too_big_pos_int.json",
    i_number_very_big_negative_int: "i_number_very_big_negative_int.json",
    i_object_key_lone_2nd_surrogate: "i_object_key_lone_2nd_surrogate.json",
    i_string_1st_surrogate_but_2nd_missing: "i_string_1st_surrogate_but_2nd_missing.json",
    i_string_1st_valid_surrogate_2nd_invalid: "i_string_1st_valid_surrogate_2nd_invalid.json",
    i_string_incomplete_surrogates_escape_valid: "i_string_incomplete_surrogates_escape_valid.json",
    i_string_incomplete_surrogate_and_escape_valid: "i_string_incomplete_surrogate_and_escape_valid.json",
    i_string_incomplete_surrogate_pair: "i_string_incomplete_surrogate_pair.json",
    i_string_invalid_lonely_surrogate: "i_string_invalid_lonely_surrogate.json",
    i_string_invalid_surrogate: "i_string_invalid_surrogate.json",
    i_string_invalid_utf: "i_string_invalid_utf-8.json",
    i_string_inverted_surrogates: "i_string_inverted_surrogates_U+1D11E.json",
    i_string_iso_latin_1: "i_string_iso_latin_1.json",
    i_string_lone_second_surrogate: "i_string_lone_second_surrogate.json",
    i_string_lone_utf8_continuation_byte: "i_string_lone_utf8_continuation_byte.json",
    i_string_not_in_unicode_range: "i_string_not_in_unicode_range.json",
    i_string_overlong_sequence_2_bytes: "i_string_overlong_sequence_2_bytes.json",
    i_string_overlong_sequence_6_bytes: "i_string_overlong_sequence_6_bytes.json",
    i_string_overlong_sequence_6_bytes_null: "i_string_overlong_sequence_6_bytes_null.json",
    i_string_truncated_utf: "i_string_truncated-utf-8.json",
    i_string_sixteen_with_bom: "i_string_UTF-16LE_with_BOM.json",
    i_string_utf_invalid_sequence: "i_string_UTF-8_invalid_sequence.json",
    i_structure_500_nested_arrays: "i_structure_500_nested_arrays.json",
    n_array_1_true_without_comma: "n_array_1_true_without_comma.json",
    n_array_a_invalid_utf8: "n_array_a_invalid_utf8.json",
    n_array_colon_instead_of_comma: "n_array_colon_instead_of_comma.json",
    n_array_comma_after_close: "n_array_comma_after_close.json",
    n_array_comma_and_number: "n_array_comma_and_number.json",
    n_array_double_comma: "n_array_double_comma.json",
    n_array_double_extra_comma: "n_array_double_extra_comma.json",
    n_array_extra_close: "n_array_extra_close.json",
    n_array_extra_comma: "n_array_extra_comma.json",
    n_array_incomplete: "n_array_incomplete.json",
    n_array_incomplete_invalid_value: "n_array_incomplete_invalid_value.json",
    n_array_inner_array_no_comma: "n_array_inner_array_no_comma.json",
    n_array_invalid_utf8: "n_array_invalid_utf8.json",
    n_array_items_separated_by_semicolon: "n_array_items_separated_by_semicolon.json",
    n_array_just_comma: "n_array_just_comma.json",
    n_array_just_minus: "n_array_just_minus.json",
    n_array_missing_value: "n_array_missing_value.json",
    n_array_newlines_unclosed: "n_array_newlines_unclosed.json",
    n_array_number_and_comma: "n_array_number_and_comma.json",
    n_array_number_and_several_commas: "n_array_number_and_several_commas.json",
    n_array_spaces_vertical_tab_formfeed: "n_array_spaces_vertical_tab_formfeed.json",
    n_array_star_inside: "n_array_star_inside.json",
    n_array_unclosed: "n_array_unclosed.json",
    n_array_unclosed_trailing_comma: "n_array_unclosed_trailing_comma.json",
    n_array_unclosed_with_new_lines: "n_array_unclosed_with_new_lines.json",
    n_array_unclosed_with_object_inside: "n_array_unclosed_with_object_inside.json",
    n_incomplete_false: "n_incomplete_false.json",
    n_incomplete_null: "n_incomplete_null.json",
    n_incomplete_true: "n_incomplete_true.json",
    n_multidigit_number_then_00: "n_multidigit_number_then_00.json",
    n_object_bad_value: "n_object_bad_value.json",
    n_object_bracket_key: "n_object_bracket_key.json",
    n_object_comma_instead_of_colon: "n_object_comma_instead_of_colon.json",
    n_object_double_colon: "n_object_double_colon.json",
    n_object_emoji: "n_object_emoji.json",
    n_object_garbage_at_end: "n_object_garbage_at_end.json",
    n_object_key_with_single_quotes: "n_object_key_with_single_quotes.json",
    n_object_lone_continuation_byte_in_key_and_trailing_comma: "n_object_lone_continuation_byte_in_key_and_trailing_comma.json",
    n_object_missing_colon: "n_object_missing_colon.json",
    n_object_missing_key: "n_object_missing_key.json",
    n_object_missing_semicolon: "n_object_missing_semicolon.json",
    n_object_missing_value: "n_object_missing_value.json",
    n_object_no_colon: "n_object_no-colon.json",
    n_object_non_string_key: "n_object_non_string_key.json",
    n_object_non_string_key_but_huge_number_instead: "n_object_non_string_key_but_huge_number_instead.json",
    n_object_repeated_null_null: "n_object_repeated_null_null.json",
    n_object_several_trailing_commas: "n_object_several_trailing_commas.json",
    n_object_single_quote: "n_object_single_quote.json",
    n_object_trailing_comma: "n_object_trailing_comma.json",
    n_object_trailing_comment: "n_object_trailing_comment.json",
    n_object_trailing_comment_open: "n_object_trailing_comment_open.json",
    n_object_trailing_comment_slash_open: "n_object_trailing_comment_slash_open.json",
    n_object_trailing_comment_slash_open_incomplete: "n_object_trailing_comment_slash_open_incomplete.json",
    n_object_two_commas_in_a_row: "n_object_two_commas_in_a_row.json",
    n_object_unquoted_key: "n_object_unquoted_key.json",
    n_object_unterminated_value: "n_object_unterminated-value.json",
    n_object_with_single_string: "n_object_with_single_string.json",
    n_object_with_trailing_garbage: "n_object_with_trailing_garbage.json",
    n_single_space: "n_single_space.json",
    n_string_1_surrogate_then_escape: "n_string_1_surrogate_then_escape.json",
    n_string_1_surrogate_then_escape_u: "n_string_1_surrogate_then_escape_u.json",
    n_string_1_surrogate_then_escape_u1: "n_string_1_surrogate_then_escape_u1.json",
    n_string_1_surrogate_then_escape_u1x: "n_string_1_surrogate_then_escape_u1x.json",
    n_string_accentuated_char_no_quotes: "n_string_accentuated_char_no_quotes.json",
    n_string_backslash_00: "n_string_backslash_00.json",
    n_string_escaped_backslash_bad: "n_string_escaped_backslash_bad.json",
    n_string_escaped_ctrl_char_tab: "n_string_escaped_ctrl_char_tab.json",
    n_string_escaped_emoji: "n_string_escaped_emoji.json",
    n_string_escape_x: "n_string_escape_x.json",
    n_string_incomplete_escape: "n_string_incomplete_escape.json",
    n_string_incomplete_escaped_character: "n_string_incomplete_escaped_character.json",
    n_string_incomplete_surrogate: "n_string_incomplete_surrogate.json",
    n_string_incomplete_surrogate_escape_invalid: "n_string_incomplete_surrogate_escape_invalid.json",
    n_string_invalid_utf8_in_escape: "n_string_invalid-utf-8-in-escape.json",
    n_string_invalid_backslash_esc: "n_string_invalid_backslash_esc.json",
    n_string_invalid_unicode_escape: "n_string_invalid_unicode_escape.json",
    n_string_invalid_utf8_after_escape: "n_string_invalid_utf8_after_escape.json",
    n_string_leading_uescaped_thinspace: "n_string_leading_uescaped_thinspace.json",
    n_string_no_quotes_with_bad_escape: "n_string_no_quotes_with_bad_escape.json",
    n_string_single_doublequote: "n_string_single_doublequote.json",
    n_string_single_quote: "n_string_single_quote.json",
    n_string_single_string_no_double_quotes: "n_string_single_string_no_double_quotes.json",
    n_string_start_escape_unclosed: "n_string_start_escape_unclosed.json",
    n_string_unescaped_ctrl_char: "n_string_unescaped_ctrl_char.json",
    n_string_unescaped_newline: "n_string_unescaped_newline.json",
    n_string_unescaped_tab: "n_string_unescaped_tab.json",
    n_string_unicode_capital_u: "n_string_unicode_CapitalU.json",
    n_string_with_trailing_garbage: "n_string_with_trailing_garbage.json",
    n_structure_100000_opening_arrays: "n_structure_100000_opening_arrays.json",
    n_structure_angle_bracket: "n_structure_angle_bracket_..json",
    n_structure_angle_bracket_null: "n_structure_angle_bracket_null.json",
    n_structure_array_trailing_garbage: "n_structure_array_trailing_garbage.json",
    n_structure_array_with_extra_array_close: "n_structure_array_with_extra_array_close.json",
    n_structure_array_with_unclosed_string: "n_structure_array_with_unclosed_string.json",
    n_structure_ascii_unicode_identifier: "n_structure_ascii-unicode-identifier.json",
    n_structure_capitalized_true: "n_structure_capitalized_True.json",
    n_structure_close_unopened_array: "n_structure_close_unopened_array.json",
    n_structure_comma_instead_of_closing_brace: "n_structure_comma_instead_of_closing_brace.json",
    n_structure_double_array: "n_structure_double_array.json",
    n_structure_end_array: "n_structure_end_array.json",
    n_structure_lone_invalid_utf8: "n_structure_lone-invalid-utf-8.json",
    n_structure_lone_open_bracket: "n_structure_lone-open-bracket.json",
    n_structure_no_data: "n_structure_no_data.json",
    n_structure_null_byte_outside_string: "n_structure_null-byte-outside-string.json",
    n_structure_number_with_trailing_garbage: "n_structure_number_with_trailing_garbage.json",
    n_structure_object_followed_by_closing_object: "n_structure_object_followed_by_closing_object.json",
    n_structure_object_unclosed_no_value: "n_structure_object_unclosed_no_value.json",
    n_structure_object_with_comment: "n_structure_object_with_comment.json",
    n_structure_object_with_trailing_garbage: "n_structure_object_with_trailing_garbage.json",
    n_structure_open_array_apostrophe: "n_structure_open_array_apostrophe.json",
    n_structure_open_array_comma: "n_structure_open_array_comma.json",
    n_structure_open_array_object: "n_structure_open_array_object.json",
    n_structure_open_array_open_object: "n_structure_open_array_open_object.json",
    n_structure_open_array_open_string: "n_structure_open_array_open_string.json",
    n_structure_open_array_string: "n_structure_open_array_string.json",
    n_structure_open_object: "n_structure_open_object.json",
    n_structure_open_object_close_array: "n_structure_open_object_close_array.json",
    n_structure_open_object_comma: "n_structure_open_object_comma.json",
    n_structure_open_object_open_array: "n_structure_open_object_open_array.json",
    n_structure_open_object_open_string: "n_structure_open_object_open_string.json",
    n_structure_open_object_string_with_apostrophes: "n_structure_open_object_string_with_apostrophes.json",
    n_structure_open_open: "n_structure_open_open.json",
    n_structure_single_eacute: "n_structure_single_eacute.json",
    n_structure_single_star: "n_structure_single_star.json",
    n_structure_trailing_hash: "n_structure_trailing_#.json",
    n_structure_uescaped_lf_before_string: "n_structure_uescaped_LF_before_string.json",
    n_structure_unclosed_array: "n_structure_unclosed_array.json",
    n_structure_unclosed_array_partial_null: "n_structure_unclosed_array_partial_null.json",
    n_structure_unclosed_array_unfinished_false: "n_structure_unclosed_array_unfinished_false.json",
    n_structure_unclosed_array_unfinished_true: "n_structure_unclosed_array_unfinished_true.json",
    n_structure_unclosed_object: "n_structure_unclosed_object.json",
    n_structure_unicode_identifier: "n_structure_unicode-identifier.json",
    n_structure_utf8_bom_no_data: "n_structure_UTF8_BOM_no_data.json",
    n_structure_whitespace_formfeed: "n_structure_whitespace_formfeed.json",
    y_array_arrays_with_spaces: "y_array_arraysWithSpaces.json",
    y_array_empty_string: "y_array_empty-string.json",
    y_array_empty: "y_array_empty.json",
    y_array_ending_with_newline: "y_array_ending_with_newline.json",
    y_array_false: "y_array_false.json",
    y_array_heterogeneous: "y_array_heterogeneous.json",
    y_array_null: "y_array_null.json",
    y_array_with_1_and_newline: "y_array_with_1_and_newline.json",
    y_array_with_leading_space: "y_array_with_leading_space.json",
    y_array_with_several_null: "y_array_with_several_null.json",
    y_array_with_trailing_space: "y_array_with_trailing_space.json",
    y_number: "y_number.json",
    y_number_0e1: "y_number_0e+1.json",
    y_number_0eplus1: "y_number_0e1.json",
    y_number_after_space: "y_number_after_space.json",
    y_number_double_close_to_zero: "y_number_double_close_to_zero.json",
    y_number_int_with_exp: "y_number_int_with_exp.json",
    y_number_minus_zero: "y_number_minus_zero.json",
    y_number_negative_int: "y_number_negative_int.json",
    y_number_negative_one: "y_number_negative_one.json",
    y_number_negative_zero: "y_number_negative_zero.json",
    y_number_real_capital_e: "y_number_real_capital_e.json",
    y_number_real_capital_e_neg_exp: "y_number_real_capital_e_neg_exp.json",
    y_number_real_capital_e_pos_exp: "y_number_real_capital_e_pos_exp.json",
    y_number_real_exponent: "y_number_real_exponent.json",
    y_number_real_fraction_exponent: "y_number_real_fraction_exponent.json",
    y_number_real_neg_exp: "y_number_real_neg_exp.json",
    y_number_real_pos_exponent: "y_number_real_pos_exponent.json",
    y_number_simple_int: "y_number_simple_int.json",
    y_number_simple_real: "y_number_simple_real.json",
    y_object: "y_object.json",
    y_object_basic: "y_object_basic.json",
    y_object_duplicated_key: "y_object_duplicated_key.json",
    y_object_duplicated_key_and_value: "y_object_duplicated_key_and_value.json",
    y_object_empty: "y_object_empty.json",
    y_object_empty_key: "y_object_empty_key.json",
    y_object_escaped_null_in_key: "y_object_escaped_null_in_key.json",
    y_object_extreme_numbers: "y_object_extreme_numbers.json",
    y_object_long_strings: "y_object_long_strings.json",
    y_object_simple: "y_object_simple.json",
    y_object_string_unicode: "y_object_string_unicode.json",
    y_object_with_newlines: "y_object_with_newlines.json",
    y_string_accepted_surrogate_pair: "y_string_accepted_surrogate_pair.json",
    y_string_accepted_surrogate_pairs: "y_string_accepted_surrogate_pairs.json",
    y_string_allowed_escapes: "y_string_allowed_escapes.json",
    y_string_backslash_and_u_escaped_zero: "y_string_backslash_and_u_escaped_zero.json",
    y_string_backslash_doublequotes: "y_string_backslash_doublequotes.json",
    y_string_comments: "y_string_comments.json",
    y_string_double_escape_a: "y_string_double_escape_a.json",
    y_string_double_escape_n: "y_string_double_escape_n.json",
    y_string_escaped_control_character: "y_string_escaped_control_character.json",
    y_string_escaped_noncharacter: "y_string_escaped_noncharacter.json",
    y_string_in_array: "y_string_in_array.json",
    y_string_in_array_with_leading_space: "y_string_in_array_with_leading_space.json",
    y_string_last_surrogates_1_and_2: "y_string_last_surrogates_1_and_2.json",
    y_string_nbsp_uescaped: "y_string_nbsp_uescaped.json",
    y_string_null_escape: "y_string_null_escape.json",
    y_string_onebyteutf8: "y_string_one-byte-utf-8.json",
    y_string_pi: "y_string_pi.json",
    y_string_simple_ascii: "y_string_simple_ascii.json",
    y_string_space: "y_string_space.json",
    y_string_u_escape: "y_string_uEscape.json",
    y_string_uescaped_newline: "y_string_uescaped_newline.json",
    y_string_unescaped_char_delete: "y_string_unescaped_char_delete.json",
    y_string_unicode: "y_string_unicode.json",
    y_string_unicode_escaped_backslash: "y_string_unicodeEscapedBackslash.json",
    y_string_unicode_2: "y_string_unicode_2.json",
    y_string_unicode_escaped_double_quote: "y_string_unicode_escaped_double_quote.json",
    y_string_utf8: "y_string_utf8.json",
    y_string_with_del_character: "y_string_with_del_character.json",
    y_structure_lonely_false: "y_structure_lonely_false.json",
    y_structure_lonely_int: "y_structure_lonely_int.json",
    y_structure_lonely_negative_real: "y_structure_lonely_negative_real.json",
    y_structure_lonely_null: "y_structure_lonely_null.json",
    y_structure_lonely_string: "y_structure_lonely_string.json",
    y_structure_lonely_true: "y_structure_lonely_true.json",
    y_structure_string_empty: "y_structure_string_empty.json",
    y_structure_trailing_newline: "y_structure_trailing_newline.json",
    y_structure_true_in_array: "y_structure_true_in_array.json",
    y_structure_whitespace_array: "y_structure_whitespace_array.json",
}
