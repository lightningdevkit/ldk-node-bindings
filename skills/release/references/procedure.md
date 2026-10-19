# Release stage procedure

## Prepare

1. Resolve the requested native version's tag to a full commit. Check existing
   candidates, local/remote maintenance branches, and publication records before
   creating another preparation. Use a fresh worktree and current `YYYY-MM-`
   branch. New release lines normally
   start from `main`; patches to an older line start from its previous released
   recipe or its `MAJOR.MINOR.x` backport branch. Use the candidate manifest to
   identify that recipe. If the native tag or prior recipe cannot be identified, report
   the missing input rather than silently substituting native main.
2. Update `release.json`: native version/revision/archive URL and hash, UDL hash,
   source epoch, bindings version and exact artifact names, and compatible
   UniFFI/toolchain inputs, `features`, and `default_features`. Keep the recipe
   guards, generator invocation, and native Cargo flags in sync with those
   inputs; changing version fields alone is insufficient. Refresh
   `locks/native.Cargo.lock` from the selected native source and the generator
   lock/configuration if UniFFI changes. Review
   the native API delta, adapt packaging and consumer tests, and regenerate
   wrappers in temporary directories. Change Maven/Nix/toolchain locks only
   when the new inputs require it; preserve unrelated pins. Run formatting,
   lint, Rust tests, recipe checks, and affected consumer/platform checks with
   Cargo output under `/tmp`. Commit the reviewed recipe under repository rules
   and prepare its PR against the selected release line before requesting builds.
   For a new minor or major release, create its `MAJOR.MINOR.x` branch from this
   reviewed commit. Ensure the previous line also has a branch at its last
   published recipe, using publication provenance rather than an arbitrary
   candidate to identify it. Preserve existing maintenance branch tips; do not
   reset or force-push them. These stable branch names are the explicit exception
   to dated preparation/PR names. Patch-release PRs target their existing line.
3. Confirm `lightningdevkit/ldk-node-bindings` (or the explicitly configured build
   repository) is public and the candidate workflow has landed on its default
   branch. Authenticate locally with repository contents and Actions write
   access; pushing workflow changes also needs Workflows write permission
   (`workflow` scope for classic PATs). Do not request registry tokens or dispatch
   builds in the Swift distribution repository.
4. Run `nix run .#release -- prepare --repo lightningdevkit/ldk-node-bindings` on
   the clean recipe branch.
   Push any newly created maintenance branches explicitly during this
   authenticated preparation, checking existing remote refs before pushing.
   It pushes, dispatches the exact SHA, watches, downloads, and records the
   candidate. Read its ledger before retrying an interrupted preparation.
5. Inspect `candidate.json` and all expected assets. A new candidate uses a new
   unique tag; do not overwrite a previous prerelease or reuse a Swift version
   tag. Report the recipe SHA, candidate URL/ID, SHA-256 of `candidate.json`,
   workflow result, and `scripts/verify.sh CANDIDATE_ID MANIFEST_SHA256` command.
   Preparation never invokes publication.

## Verify and collect

Read [the reproducibility boundary](../../../docs/reproducibility.md) when
qualifying a target or diagnosing unequal bytes. The simple entrypoint is
`scripts/verify.sh CANDIDATE_ID MANIFEST_SHA256`, using the manifest checksum
received through a trusted release announcement. It selects the candidate's
exact recipe and supported host groups without changing the caller's checkout.
For the lower-level CLI, use the exact clean recipe commit named by the candidate,
with unchanged locks and the documented host.

Run `verify --candidate URL --group GROUP --output DIR` for each host group.
The verifier rebuilds project outputs even when candidate files or Nix project
outputs already exist. Tool/dependency caches are permitted. Keep failed-build
evidence; do not edit receipt hashes, skip files, or relax equality checks.

Verification may cover `generated`, `linux-x86_64`, `linux-aarch64`, `android`,
`darwin`, and `jvm`. JVM assembly imports the four candidate native libraries
and records their identities. Matching assembly alone does not establish that
the imported libraries were independently rebuilt.

Optionally choose trusted receipts and place only those in a collection directory.
Run `collect --candidate URL --output COLLECTED_FILE`, adding `--receipts DIR`
when available. Collection validates all candidate bytes and supplied receipts,
then asks whether each remaining group's artifacts were independently rebuilt
and compared elsewhere. Record yes answers as out-of-band confirmations, not
synthetic receipts. No or blank answers leave the group unconfirmed. Use
`--non-interactive` to skip prompts, or `--require-verification` to require every
group to have either form of confirmation. Report all three categories.
Consumer/platform test results should accompany the ledger even though they
are distinct from hash equality.

## Publish

Read [the release guide](../../../docs/releasing.md) for commands, credentials,
TestPyPI requirements, Swift metadata, and recovery. Present the candidate,
recorded verification coverage, and requested destinations before invoking the explicit publication
gate in the skill. Reuse existing authorization for that exact publication;
do not ask the user again when they already explicitly authorized it.

Invoke each requested destination locally with `--collected COLLECTED_FILE` and
the exact candidate confirmation. `--verified` remains a compatibility alias.
Publication requires complete receipt or out-of-band coverage by default.
Use `--allow-unverified` only for a maintainer's explicit choice to publish with
missing comparisons. Each attempt preserves its coverage and override in the
publication history; recollection does not change immutable Swift metadata.
Upload existing candidate files, never a rebuild. Treat
TestPyPI staging, Maven upload/validation, and Swift metadata commits as distinct
from a completed production publication. Inspect remote state after errors and
report partial completion precisely.
