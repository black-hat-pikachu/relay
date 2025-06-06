/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use fixture_tests::Fixture;
use graphql_ir::build;
use graphql_syntax::FragmentArgumentSyntaxKind;
use graphql_syntax::ParserFeatures;
use graphql_syntax::parse_executable_with_features;
use graphql_text_printer::print_ir;
use relay_test_schema::TEST_SCHEMA;

pub async fn transform_fixture(fixture: &Fixture<'_>) -> Result<String, String> {
    let source_location = SourceLocationKey::standalone(fixture.file_name);
    let ast = parse_executable_with_features(
        fixture.content,
        source_location,
        ParserFeatures {
            fragment_argument_capability:
                FragmentArgumentSyntaxKind::SpreadArgumentsAndFragmentVariableDefinitions,
            allow_string_literal_alias: false,
        },
    )
    .unwrap();
    build(&TEST_SCHEMA, &ast.definitions)
        .map(|definitions| print_ir(&TEST_SCHEMA, &definitions).join("\n\n"))
        .map_err(|errors| {
            errors
                .into_iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<_>>()
                .join("\n\n")
        })
}
