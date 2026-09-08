# Capture attempts, not passing evidence

The initial intake mobile screenshots were taken before the inherited450ms
drawer close transition finished. The left sliver is a capture timing artifact,
not a confirmed product defect. Their original screenshots/raw measurements are
retained here. Passing replacements wait for popup detachment and sample settled
geometry. Git history also preserves the original ce8aae7f screenshots.

The original phone-confirmed keyboard first frame remains in screenshots/ and
phone-review-results.json. Its transparent ring is not a visible-focus pass;
the settled350ms still-pending FAW/FAX frames establish visible focus instead.
