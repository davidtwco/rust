// Test that a missing `--target` arg combined with `-Ztarget-spec` produces an error.
use run_make_support::rustc;

fn main() {
    rustc()
        .input("foo.rs")
        .arg("-Ztarget-spec=arch=\"aarch64\"")
        .run_fail()
        .assert_stderr_contains("`-Ztarget-spec` requires `--target` to name the target");
}
