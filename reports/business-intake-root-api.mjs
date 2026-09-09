// Root-owned acceptance harness; no product code or stored credential values.
// Grounded in e55f3bfd tenant_http_cases/types/platform and 7ed0dd0c fixture.
// Node24.19.0 fs.js/random.js read via gh api after installed @types/node25.2.2;
// Context7 v24 documentation lookup returned no snippets. No exact-version claim
// is derived from the installed newer typings.
import fs from "node:fs"
import path from "node:path"
import assert from "node:assert/strict"
import { randomUUID } from "node:crypto"

const directory = process.env.OPS_INTAKE_FIXTURE_DIR
assert(directory, "Set the authorized fixture directory")
const credentials = JSON.parse(fs.readFileSync(path.join(directory, "root-credentials.json")))
const controls = JSON.parse(fs.readFileSync(path.join(directory, "owner-controls.json")))
assert.equal(credentials.synthetic, true)
assert.equal(credentials.namespace, "root")
assert.equal(controls.synthetic, true)
const owner = credentials.sessions.owner.token
const viewer = credentials.sessions.viewer.token
const manager = credentials.sessions.manager.token
const output = process.env.OPS_INTAKE_RESULT
assert(output, "Set a new result path")
assert(!fs.existsSync(output), "Preserve earlier results")
const events = []
const result = {
  fixtureSource: "7ed0dd0c28f5065f1c37983f366cc6dcb0727e08",
  productSource: "e55f3bfd1f069d6d6111370223993596b19ecb9b",
  namespace: "root", startedAt: new Date().toISOString(), events,
  scope: "Real loopback HTTP against protected synthetic fixture; no browser/native/live-provider claim",
}
const base = "http://127.0.0.1:4351"
const intake = "/api/business/intake/"
async function request(name, route, input, token = owner, expected = 200) {
  const response = await fetch(base + route, {
    method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}` },
    body: JSON.stringify({ input }), signal: AbortSignal.timeout(20000),
  })
  const raw = await response.text()
  events.push({ name, status: response.status, expected })
  assert.equal(response.status, expected, name)
  const body = response.headers.get("content-type")?.includes("application/json")
    ? JSON.parse(raw) : { responseText: raw }
  const serialized = JSON.stringify(body)
  for (const secret of [owner, viewer, manager, credentials.firefliesApiKey, controls.platformToken])
    assert(!serialized.includes(secret), `${name}: credential response disclosure`)
  return body
}
async function api(name, route, input, token = owner, expected = 200) {
  return request(name, intake + route, input, token, expected)
}
let organization = credentials.organization
let suspended = false
async function lifecycle(status) {
  organization = await request(`root tenant ${status}`, "/api/platform/business/tenants/status", {
    operationId: randomUUID(), organizationId: organization.id,
    expectedRevision: organization.revision,
    expectedAuthorizationEpoch: organization.authorizationEpoch, status,
  }, controls.platformToken)
  suspended = status === "suspended"
}
async function importSources(bindingId, selection) {
  let job = await api("start authorized import", "imports/start", { operationId: randomUUID(), bindingId, selection })
  for (let step = 0; job.state !== "complete" && step < 8; step++) {
    job = await api("advance authorized import", "imports/advance", {
      operationId: randomUUID(), importId: job.id, expectedRevision: job.revision,
    })
  }
  assert.equal(job.state, "complete", "Bounded import completed")
  return job
}
try {
  const initial = await api("initial root binding list", "bindings/list", {})
  assert.equal(initial.items.length, 0, "Run only on pristine root namespace")
  assert.deepEqual(initial.setupKinds, ["fireflies"])
  const create = {
    operationId: randomUUID(), label: "Root acceptance: customer follow-up", domain: "feedback",
    sourceOwnerId: credentials.sessions.owner.memberId,
    source: { kind: "fireflies", apiKey: credentials.firefliesApiKey },
    publicationDomains: ["feedback"], retainedTaskText: true,
  }
  await api("manager cannot configure provider", "bindings/create", create, manager, 403)
  await request("member cannot act as platform", "/api/platform/business/tenants/list", {}, owner, 401)
  let binding = await api("owner creates scoped provider", "bindings/create", create)
  result.bindingId = binding.id
  const grants = await api("new provider has zero grants", "grants/list", { bindingId: binding.id })
  assert.equal(grants.items.length, 0)
  await api("setup does not grant source read", "sources/list", { bindingId: binding.id }, owner, 404)
  for (const role of ["owner", "viewer"]) {
    const granted = await api(`explicit ${role} source grant`, "grants/upsert", {
      operationId: randomUUID(), bindingId: binding.id, expectedBindingRevision: binding.revision,
      memberId: credentials.sessions[role].memberId, expectedGrantRevision: null,
      scope: "binding_current_and_future_sources", read: true,
      import: role === "owner", triage: role === "owner",
      publicationDomains: role === "owner" ? ["feedback"] : [], expiresAt: null,
    })
    binding = granted.binding
  }
  binding = await api("enable explicitly granted provider", "bindings/update", {
    operationId: randomUUID(), bindingId: binding.id, expectedRevision: binding.revision,
    label: binding.label, enabled: true, publicationDomains: ["feedback"], retainedTaskText: true,
  })
  await api("granted viewer cannot import", "imports/start", {
    operationId: randomUUID(), bindingId: binding.id,
    selection: { kind: "window", fromDate: "2026-09-01T00:00:00Z", toDate: "2026-09-09T00:00:00Z" },
  }, viewer, 403)
  await importSources(binding.id, { kind: "window", fromDate: "2026-09-01T00:00:00Z", toDate: "2026-09-09T00:00:00Z" })
  const list = await api("root source list", "sources/list", { bindingId: binding.id })
  assert.equal(list.items.length, 3)
  const sourceId = list.items.find(x => x.title.includes("meeting-follow-up")).id
  result.sourceId = sourceId
  let detail = await api("owner fresh source", "sources/get", { sourceId })
  assert.equal(detail.disclosure, "fresh")
  const view = await api("explicitly granted viewer may read", "sources/get", { sourceId }, viewer)
  assert.equal(view.disclosure, "fresh")
  const candidates = await api("source candidates", "candidates/list", { sourceId })
  let candidate = candidates.items[0]
  result.candidateId = candidate.id
  const draft = { title: "Root reviewed onboarding follow-up", notes: "Only this deliberately reviewed business text.", domain: "feedback", dueDate: "2026-11-01" }
  let edited = await api("prepare exact public draft", "candidates/edit", {
    operationId: randomUUID(), candidateId: candidate.id, expectedRevision: candidate.revision,
    expectedSourceRevision: detail.source.revision, passageIds: [detail.passages[0].id], task: draft,
  })
  candidate = edited.candidate
  await lifecycle("suspended")
  await api("suspended tenant cannot disclose source", "sources/get", { sourceId }, owner, 401)
  await lifecycle("active")
  detail = await api("fresh login does not renew source observation", "sources/get", { sourceId })
  assert.notEqual(detail.disclosure, "fresh")
  assert.equal(detail.passages.length, 0)
  const withheld = await api("old preparation withheld after resume", "candidates/get", { candidateId: candidate.id })
  assert.equal(withheld.candidate.draft, null)
  await importSources(binding.id, { kind: "record", sourceId })
  detail = await api("explicit refresh renews source", "sources/get", { sourceId })
  assert.equal(detail.disclosure, "fresh")
  const stillStale = await api("refresh alone does not renew preview", "candidates/get", { candidateId: candidate.id })
  assert.equal(stillStale.candidate.requiresRebase, true)
  assert.equal(stillStale.candidate.capabilities.accept, false)
  edited = await api("explicitly rebase exact reviewed task", "candidates/edit", {
    operationId: randomUUID(), candidateId: candidate.id, expectedRevision: stillStale.candidate.revision,
    expectedSourceRevision: detail.source.revision, passageIds: [detail.passages[0].id], task: draft,
  })
  const accept = { operationId: randomUUID(), candidateId: candidate.id, expectedRevision: edited.candidate.revision,
    expectedSourceRevision: detail.source.revision, publishToDomain: "feedback" }
  const accepted = await api("publish reviewed task", "candidates/accept", accept)
  assert.equal(accepted.task.task.notes, draft.notes)
  assert.equal(accepted.task.task.creatorId, credentials.sessions.owner.memberId)
  assert.equal(accepted.task.task.dueDate, draft.dueDate)
  const replay = await api("lost response replay yields same task", "candidates/accept", accept)
  assert.equal(replay.replayed, true)
  assert.equal(replay.task.task.id, accepted.task.task.id)
  result.taskId = accepted.task.task.id
  result.outcome = "pass"
} catch (error) {
  result.outcome = "fail"
  result.failure = { name: error.name, message: error.message }
  process.exitCode = 1
} finally {
  if (suspended) {
    try { await lifecycle("active") } catch { result.restoreFailed = true; process.exitCode = 1 }
  }
  result.finishedAt = new Date().toISOString()
  fs.writeFileSync(output, JSON.stringify(result, null, 2) + "\n", { flag: "wx" })
  console.log(JSON.stringify({ outcome: result.outcome, checks: events.length, result: output, failure: result.failure }))
}
