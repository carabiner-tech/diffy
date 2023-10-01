use diffy::{apply, Patch};

#[test]
fn test_short_valid_diff() {
    let orig = "foo\nbar\nbaz";
    let diff = r#"--- in.txt
+++ out.txt
@@ -1,3 +1,3 @@
 foo
-bar
+qux
 baz
\ No newline at end of file
"#;
    let patch = Patch::from_str(diff).unwrap();
    let result = apply(orig, &patch).unwrap();
    assert_eq!(result, "foo\nqux\nbaz");
}

#[test]
fn test_short_invalid_headers() {
    let orig = "foo\nbar\nbaz";
    let diff = r#"--- in.txt
+++ out.txt
@@ -1,1 +1,5 @@
 foo
-bar
+qux
 baz
\ No newline at end of file
"#;
    let patch = Patch::from_str(diff).unwrap();
    let result = apply(orig, &patch).unwrap();
    assert_eq!(result, "foo\nqux\nbaz");
}

#[test]
fn test_short_invalid_headers_with_newline() {
    let orig = "foo\nbar\nbaz\n\n";
    let diff = r#"--- in.txt
+++ out.txt
@@ -1,1 +1,5 @@
 foo
-bar
+qux
 baz
"#;
    let patch = Patch::from_str(diff).unwrap();
    let result = apply(orig, &patch).unwrap();
    assert_eq!(result, "foo\nqux\nbaz\n\n");
}
