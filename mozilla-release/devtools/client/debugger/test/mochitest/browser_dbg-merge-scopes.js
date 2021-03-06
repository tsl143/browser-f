/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

function getLabel(dbg, index) {
  return findElement(dbg, "scopeNode", index).innerText;
}

function expectLabels(dbg, array) {
  for (let i = 0; i < array.length; i++) {
    is(getLabel(dbg, i + 1), array[i], `Correct label ${array[i]} for index ${i + 1}`);
  }
}

// Test that adjacent scopes are merged together as expected.
add_task(async function() {
  const dbg = await initDebugger("doc-merge-scopes.html");

  invokeInTab("run");
  await waitForPaused(dbg, "doc-merge-scopes.html");

  // Function body and function lexical scopes are merged together.
  expectLabels(dbg, ["first", "<this>", "arguments", "x", "y", "z"]);

  await resume(dbg);
  await waitForPaused(dbg);

  // Function body and inner lexical scopes are not merged together.
  await toggleScopeNode(dbg, 4);
  expectLabels(dbg, ["Block", "<this>", "y", "second", "arguments", "x"]);

  await resume(dbg);
  await waitForPaused(dbg);

  // When there is a function body, function lexical, and inner lexical scope,
  // the first two are merged together.
  await toggleScopeNode(dbg, 4);
  expectLabels(dbg, ["Block", "<this>", "z", "third", "arguments", "v", "x", "y"]);
});
