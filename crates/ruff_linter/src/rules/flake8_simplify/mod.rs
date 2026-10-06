//! Rules from [flake8-simplify](https://pypi.org/project/flake8-simplify/).
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use ruff_diagnostics::Applicability;
    use ruff_python_ast::PythonVersion;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::settings::LinterSettings;
    use crate::test::{test_path, test_snippet};
    use crate::{assert_diagnostics, assert_diagnostics_diff, settings};

    #[test_case(Rule::DuplicateIsinstanceCall, Path::new("SIM101.py"))]
    #[test_case(Rule::CollapsibleIf, Path::new("SIM102.py"))]
    #[test_case(Rule::NeedlessBool, Path::new("SIM103.py"))]
    #[test_case(Rule::SuppressibleException, Path::new("SIM105_0.py"))]
    #[test_case(Rule::SuppressibleException, Path::new("SIM105_1.py"))]
    #[test_case(Rule::SuppressibleException, Path::new("SIM105_2.py"))]
    #[test_case(Rule::SuppressibleException, Path::new("SIM105_3.py"))]
    #[test_case(Rule::SuppressibleException, Path::new("SIM105_4.py"))]
    #[test_case(Rule::ReturnInTryExceptFinally, Path::new("SIM107.py"))]
    #[test_case(Rule::IfElseBlockInsteadOfIfExp, Path::new("SIM108.py"))]
    #[test_case(Rule::CompareWithTuple, Path::new("SIM109.py"))]
    #[test_case(Rule::ReimplementedBuiltin, Path::new("SIM110.py"))]
    #[test_case(Rule::ReimplementedBuiltin, Path::new("SIM111.py"))]
    #[test_case(Rule::UncapitalizedEnvironmentVariables, Path::new("SIM112.py"))]
    #[test_case(Rule::EnumerateForLoop, Path::new("SIM113.py"))]
    #[test_case(Rule::IfWithSameArms, Path::new("SIM114.py"))]
    #[test_case(Rule::OpenFileWithContextHandler, Path::new("SIM115.py"))]
    #[test_case(Rule::IfElseBlockInsteadOfDictLookup, Path::new("SIM116.py"))]
    #[test_case(Rule::MultipleWithStatements, Path::new("SIM117.py"))]
    #[test_case(Rule::InDictKeys, Path::new("SIM118.py"))]
    #[test_case(Rule::NegateEqualOp, Path::new("SIM201.py"))]
    #[test_case(Rule::NegateNotEqualOp, Path::new("SIM202.py"))]
    #[test_case(Rule::DoubleNegation, Path::new("SIM208.py"))]
    #[test_case(Rule::IfExprWithTrueFalse, Path::new("SIM210.py"))]
    #[test_case(Rule::IfExprWithFalseTrue, Path::new("SIM211.py"))]
    #[test_case(Rule::IfExprWithTwistedArms, Path::new("SIM212.py"))]
    #[test_case(Rule::ExprAndNotExpr, Path::new("SIM220.py"))]
    #[test_case(Rule::ExprOrNotExpr, Path::new("SIM221.py"))]
    #[test_case(Rule::ExprOrTrue, Path::new("SIM222.py"))]
    #[test_case(Rule::ExprAndFalse, Path::new("SIM223.py"))]
    #[test_case(Rule::YodaConditions, Path::new("SIM300.py"))]
    #[test_case(Rule::IfElseBlockInsteadOfDictGet, Path::new("SIM401.py"))]
    #[test_case(Rule::SplitStaticString, Path::new("SIM905.py"))]
    #[test_case(Rule::DictGetWithNoneDefault, Path::new("SIM910.py"))]
    #[test_case(Rule::ZipDictKeysAndValues, Path::new("SIM911.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.name(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("flake8_simplify").join(path).as_path(),
            &settings::LinterSettings::for_rule(rule_code),
        )?;
        assert_diagnostics!(snapshot, diagnostics);
        Ok(())
    }

    #[test_case(
        "def f(a, b):\n    if not a and b:\n        return True\n    elif a and b:\n        return True\n    else:\n        return False\n",
        "return bool(not a and b or a and b)"
    )]
    #[test_case(
        "def f(a, b, c):\n    if a:\n        return True\n    elif b:\n        return True\n    elif c:\n        return True\n    else:\n        return False\n",
        "return bool(a or b or c)"
    )]
    #[test_case(
        "def f(a, b, c):\n    if a:\n        return False\n    elif b:\n        return False\n    elif c:\n        return False\n    else:\n        return True\n",
        "return not (a or b or c)"
    )]
    fn needless_bool_chain(source: &str, replacement: &str) {
        let diagnostics = test_snippet(source, &LinterSettings::for_rule(Rule::NeedlessBool));
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert!(!diagnostic.has_applicable_fix(Applicability::Safe));
        assert!(diagnostic.has_applicable_fix(Applicability::Unsafe));
        assert_eq!(
            diagnostic
                .fix()
                .and_then(|fix| fix.edits().first())
                .and_then(|edit| edit.content()),
            Some(replacement)
        );
    }

    #[test_case(Rule::EnumerateForLoop, Path::new("SIM113.py"))]
    fn preview_rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("preview__{}_{}", rule_code.name(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("flake8_simplify").join(path).as_path(),
            &LinterSettings::for_rule(rule_code).with_preview_mode(),
        )?;
        assert_diagnostics!(snapshot, diagnostics);
        Ok(())
    }

    #[test_case(Rule::SuppressibleException, Path::new("SIM105_5.py"))]
    fn version_specific_rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("diff_{}_{}", rule_code.name(), path.to_string_lossy());
        assert_diagnostics_diff!(
            snapshot,
            Path::new("flake8_simplify").join(path).as_path(),
            &settings::LinterSettings::for_rule(rule_code)
                .with_target_version(PythonVersion::PY311),
            &settings::LinterSettings::for_rule(rule_code)
                .with_target_version(PythonVersion::PY312)
        );
        Ok(())
    }
}
