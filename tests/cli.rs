use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::predicate;

#[test]
fn run_help() {
    let mut cmd = cargo_bin_cmd!("edi-format");
    cmd.arg("--help");

    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("--dry-run"));
    cmd.assert().stdout(predicate::str::contains("--log-level"));
    cmd.assert().stdout(predicate::str::contains("--stdin"));
    cmd.assert().stdout(predicate::str::contains("--version"));
    cmd.assert().stdout(predicate::str::contains("version"));
}

#[test]
fn run_formatted_file() {
    let mut cmd = cargo_bin_cmd!("edi-format");
    cmd.arg("--log-level");
    cmd.arg("debug");
    cmd.arg("tests/valid_formatted.edi");

    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains(
        "Already formatted skipping tests/valid_formatted.edi",
    ));
}

#[test]
fn run_dry_run() {
    let mut cmd = cargo_bin_cmd!("edi-format");

    cmd.arg("--dry-run").arg("tests/valid_not_formatted.edi");

    let formatted_content = "UNA:+.? '
UNB+IATB:1+6XPPC:ZZ+LHPPC:ZZ+940101:0950+1'
UNH+1+PAORES:93:1:IA'
MSG+1:45'
IFT+3+XYZCOMPANY AVAILABILITY'
ERC+A7V:1:AMD'
IFT+3+NO MORE FLIGHTS'
ODI'
TVL+240493:1000::1220+FRA+JFK+DL+400+C'
PDI++C:3+Y::3+F::1'
APD+74C:0:::6++++++6X'
TVL+240493:1740::2030+JFK+MIA+DL+081+C'
PDI++C:4'
APD+EM2:0:1630::6+++++++DA'
UNT+13+1'
UNZ+1+1'";

    cmd.assert().success();
    cmd.assert()
        .stdout(predicate::str::contains("Running in dry-run mode"));
    cmd.assert()
        .stdout(predicate::str::contains(formatted_content));
}
