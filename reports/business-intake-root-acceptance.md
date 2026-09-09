# Root acceptance of the unified intake fixture

Active review; final browser/design/native acceptance is pending.

Fixture handoff1042a0b7 identifies source7ed0dd0c28f5065f1c37983f366cc6dcb0727e08
and producte55f3bfd1f069d6d6111370223993596b19ecb9b. Root imported the fixture
recipe and evidence, verified all15 listed SHA256 digests and independently read
the stable executable:305495032 bytes, SHA256
313d987c2c5eb159bef700e19b9dffc2f942385e76e3b9a9612cf78ad08866ea.
Committed fixture source matches its advertisedb37a13cf hash. Real health reports
PID66200, backend4351/upstream4352, memory-only secrets and no engine.

## Independently executed HTTP flow

[Harness](business-intake-root-api.mjs) runs against the actual loopback listener,
production router/identity/intake/task core and synthetic reader/store. It reads
only root-credentials.json and owner-controls.json in the authorized fixture
directory; values never enter the harness, result files or command arguments.
Only root tenant records are mutated. Root suspension is restored to active
before completion, including the failure cleanup path.

[Corrected execution](business-intake-root-evidence/api-second.json): exit0,
32 HTTP status checks plus content/authority/replay assertions pass. Evidence covers:

- Empty root binding list and Fireflies-only setup scope; Manager setup denied,
  member bearer denied on the protected platform endpoint.
- Provider setup creates zero grants and does not confer source access. Explicit
  owner and read-only viewer grants allow viewer reads but deny viewer import.
- A bounded real adapter import creates three synthetic source records; a human
  prepares exact public task text from explicitly selected source passages.
- Actual root tenant suspend/resume denies suspended reads. Fresh authentication
  leaves old passages and draft withheld. Identical-content detail refresh makes
  source access fresh but cannot renew the old preview; explicit edit/rebase is
  required before acceptance.
- Acceptance preserves reviewed notes, human creator and calendar date. Replaying
  the exact operation returns the same task and a replay receipt.

[First execution](business-intake-root-evidence/api-first.json) is retained: exit1
after two passing checks, before creating any binding. The harness tried to parse
the platform middleware's plain-text401 body as JSON. The correction records and
checks status first and parses by response content type. This is a harness error,
not a backend defect or a failed authorization boundary. The successful run used
the same root namespace, which still had zero bindings.

Run command uses OPS_INTAKE_FIXTURE_DIR for the authorized directory and
OPS_INTAKE_RESULT for a new result file, then node reports/business-intake-root-api.mjs.
Do not rerun the pristine-namespace harness against the now-populated root tenant;
continue from the recorded IDs for browser acceptance. No fixture reset is needed.

## Documentation grounding and limits

Read exact e55 tenant_http_cases/types/platform and7ed fixture query/guard/control
source before API work. Node runtime24.19.0 was verified locally. Installed
@types/node25.2.2 fs/crypto declarations were read; they are newer than the runtime
and do not establish runtime-version parity. Context7's v24 lookup returned no
snippets. Exact Node v24.19.0 fs.js readFileSync/writeFileSync and crypto/random.js
randomUUID source were then read via gh api. A docs-first reminder fired on the
initial harness edit; this subsequent grounding and correction are disclosed.

These checks exercise real HTTP with synthetic upstream data. They do not certify
browser rendering, native restricted-window isolation, external Fireflies behavior
or credential persistence across fixture restart. Earlier independent949/a58 tests
and owner's38-case suite retain their separate attribution.

The fixture supports tenant-owned Fireflies. Original-operator email/Hafidh
projection core/tests remain separate existing evidence; new-tenant legacy resource
entrustment is still unimplemented. Its future protected PlatformContext operation
needs platform-attributed receipt/audit, target active organization/epoch CAS and
immutable host-resource/source-owner binding, without reconstructing a tenant
Principal. This gap does not block these Fireflies checks and is not counted as
completed B scope.
