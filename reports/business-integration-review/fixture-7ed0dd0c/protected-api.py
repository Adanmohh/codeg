"""Reviewer-only synthetic HTTP probes. No credential value is printed or saved.

Wire flow grounded in Codeg Apache-2.0 at7ed0dd0c, types/http and
tests/tenant_http_cases.rs; exact attribution in this directory's NOTICE.
Python3.12.13 http.client implementation read before use. Fixed IPv4 loopback;
no proxy, redirect, HTTP retry, provider client, database or engine access.
"""
import hashlib
import http.client
import json
import pathlib
import stat
import time
import uuid

HERE = pathlib.Path(__file__).resolve().parent
CREDENTIALS = pathlib.Path("/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/review-credentials.json")
RESULT = HERE / "protected-api-results.json"
RESOURCES = HERE / "review-resources.json"
report = {"sourceCommit": "7ed0dd0c28f5065f1c37983f366cc6dcb0727e08", "productionCommit": "e55f3bfd1f069d6d6111370223993596b19ecb9b", "namespace": "review", "backend": "http://127.0.0.1:4351", "requests": [], "assertions": [], "status": "running", "providerTraffic": "Injected fixed loopback reader only", "httpRetries": 0}
resources = {}
secrets = []
sessions = {}


class ProbeFailure(Exception):
    pass


def save():
    RESULT.write_text(json.dumps(report, indent=2) + "\n")
    RESOURCES.write_text(json.dumps(resources, indent=2) + "\n")


def check(label, condition):
    report["assertions"].append({"label": label, "pass": bool(condition)})
    save()
    if not condition:
        raise ProbeFailure(label)


def request(label, path, data=None, role="owner", expected=200, method="POST", origin=None):
    if not path.startswith(("/api/business/", "/api/platform/business/", "/__business_intake_fixture/")):
        raise ProbeFailure("probe_path_not_allowed")
    headers = {"Content-Type": "application/json"}
    if role is not None:
        headers["Authorization"] = "Bearer " + sessions[role]["token"]
    if origin is not None:
        headers["Origin"] = origin
    body = None if method == "GET" else json.dumps({"input": data or {}})
    conn = http.client.HTTPConnection("127.0.0.1", 4351, timeout=20)
    start = time.monotonic()
    try:
        conn.request(method, path, body=body, headers=headers)
        response = conn.getresponse()
        raw = response.read(2 * 1024 * 1024 + 1)
        item = {"label": label, "path": path, "role": role, "method": method, "status": response.status, "expected": expected, "seconds": round(time.monotonic() - start, 4), "cacheControl": response.getheader("Cache-Control"), "contentType": response.getheader("Content-Type", "")}
    finally:
        conn.close()
    report["requests"].append(item)
    save()
    check(label + ": status", item["status"] == expected)
    check(label + ": bounded response", len(raw) <= 2 * 1024 * 1024)
    text = raw.decode()
    check(label + ": no credential disclosure", all(secret not in text for secret in secrets))
    if path.startswith("/api/business/") and expected != 403:
        check(label + ": no-store", item["cacheControl"] == "no-store")
    # Pinned web/auth.rs returns a plain-text401; JSON is not its error contract.
    if not item["contentType"].startswith("application/json"):
        check(label + ": expected original-token rejection body", path.startswith("/api/platform/business/") and expected == 401 and text == "Invalid or missing token")
        return None
    return json.loads(text)


def intake(label, route, data=None, **options):
    return request(label, "/api/business/intake/" + route, data, **options)


def op():
    return str(uuid.uuid4())


def advance(job, prefix):
    for index in range(8):
        if job["state"] == "complete":
            return job
        check(prefix + ": advance permitted", job["capabilities"]["advance"])
        job = intake(prefix + ": advance " + str(index), "imports/advance", {"operationId": op(), "importId": job["id"], "expectedRevision": job["revision"]})
    raise ProbeFailure(prefix + ": exceeded bounded advance count")


def refresh(binding, source, label):
    job = intake(label + ": start", "imports/start", {"operationId": op(), "bindingId": binding, "selection": {"kind": "record", "sourceId": source}})
    advance(job, label)
    return intake(label + ": detail", "sources/get", {"sourceId": source})


def run():
    global sessions, secrets
    mode = CREDENTIALS.lstat().st_mode
    check("credential input is regular0600", stat.S_ISREG(mode) and stat.S_IMODE(mode) == 0o600)
    values = json.loads(CREDENTIALS.read_text())
    check("only review synthetic file", values["synthetic"] is True and values["namespace"] == "review")
    sessions = values["sessions"]
    secrets = [v["token"] for v in sessions.values()] + [values["firefliesApiKey"], values["controlToken"]]
    org = values["organization"]["id"]
    resources.update({"namespace": "review", "organizationId": org, "linkTarget": values["linkTarget"]})
    health = request("live fixture identity", "/__business_intake_fixture/health", role=None, method="GET")
    check("exact fixture runtime", health["pid"] == 66200 and health["backendPort"] == 4351 and health["upstreamPort"] == 4352)
    check("fixed synthetic runtime boundary", health["synthetic"] and health["engineStarted"] is False and health["tenantWindowAvailable"] is False and health["secretStore"] == "injected-memory-only" and health["readerEndpoint"] == "http://127.0.0.1:4352/graphql")
    report["startCountsSharedNotExclusive"] = health["counts"]
    request("missing session", "/api/business/context", role=None, expected=401)
    request("member cannot use platform", "/api/platform/business/context", expected=401)
    for role in ["owner", "manager", "viewer"]:
        context = request(role + " real principal", "/api/business/context", role=role)
        check(role + ": server-derived identity", context["organization"]["id"] == org and context["member"]["id"] == sessions[role]["memberId"] and context["member"]["role"] == role and context["operator"] is False)
    initial = intake("initial setup scope", "bindings/list")
    check("review namespace initially empty", initial["items"] == [] and initial["setupKinds"] == ["fireflies"] and "feedback" in initial["setupDomains"])
    create = {"operationId": op(), "label": "Review synthetic customer meetings", "domain": "feedback", "sourceOwnerId": sessions["owner"]["memberId"], "source": {"kind": "fireflies", "apiKey": values["firefliesApiKey"]}, "publicationDomains": ["feedback"], "retainedTaskText": True}
    for field in ["organizationId", "actorId", "authorizationEpoch", "credentialRef"]:
        intake("closed setup " + field, "bindings/create", {**create, field: "forged"}, expected=400)
    for role in ["manager", "viewer"]:
        intake(role + " setup denial", "bindings/create", create, role=role, expected=403)
    binding = intake("owner setup", "bindings/create", create)
    bid = binding["id"]
    resources["bindingId"] = bid
    check("setup starts disabled and bound to own source", not binding["enabled"] and binding["sourceOwnerId"] == sessions["owner"]["memberId"] and binding["resource"] == {"kind": "fireflies", "providerUserId": "synthetic-owner-review", "mine": True})
    grants = intake("zero initial grants", "grants/list", {"bindingId": bid})
    check("no implicit source-owner grant", grants["items"] == [])
    intake("owner source use before grant denied", "sources/list", {"bindingId": bid}, expected=404)
    # Foreign IDs only in read denial probes: no other namespace mutations.
    request("foreign existing task denied", "/api/business/tasks/get", {"taskId": "22edab65-ac1a-4ac2-9a26-440a8e9900aa"}, expected=404)
    active = {}
    for role in ["owner", "manager"]:
        grant = intake(role + " explicit binding grant", "grants/upsert", {"operationId": op(), "bindingId": bid, "expectedBindingRevision": binding["revision"], "memberId": sessions[role]["memberId"], "expectedGrantRevision": None, "scope": "binding_current_and_future_sources", "read": True, "import": True, "triage": True, "publicationDomains": ["feedback"], "expiresAt": None})
        binding = grant["binding"]
        active[role] = grant["grant"]
    binding = intake("explicit source enable", "bindings/update", {"operationId": op(), "bindingId": bid, "expectedRevision": binding["revision"], "label": binding["label"], "enabled": True, "publicationDomains": ["feedback"], "retainedTaskText": True})
    window = intake("durable window import", "imports/start", {"operationId": op(), "bindingId": bid, "selection": {"kind": "window", "fromDate": "2026-09-01T00:00:00Z", "toDate": "2026-09-09T00:00:00Z"}})
    resources["importId"] = window["id"]
    window = advance(window, "window")
    check("bounded import complete", window["state"] == "complete" and window["completed"] == 3 and window["discovered"] == 3 and window["failed"] == 0 and window["coverage"] == "bounded_end")
    unfinished = intake("unfinished rediscovery", "imports/list", {"bindingId": bid})
    all_jobs = intake("all rediscovery", "imports/list", {"bindingId": bid, "view": "all"})
    check("durable terminal rediscovery", unfinished["items"] == [] and any(item["id"] == window["id"] for item in all_jobs["items"]))
    listing = intake("source list", "sources/list", {"bindingId": bid})
    check("three sources from own provider principal", len(listing["items"]) == 3 and all(item["title"].startswith("review:") for item in listing["items"]))
    by_title = {item["title"]: item["id"] for item in listing["items"]}
    a = by_title["review: meeting-follow-up"]
    b = by_title["review: meeting-long"]
    empty = by_title["review: meeting-empty"]
    resources.update({"sourceA": a, "sourceB": b, "emptySource": empty})
    da = intake("source A actual private read", "sources/get", {"sourceId": a})
    check("source A fresh exact passages", da["disclosure"] == "fresh" and len(da["passages"]) == 3 and all("review" in item["text"] for item in da["passages"]))
    intake("manager explicitly granted source read", "sources/get", {"sourceId": a}, role="manager")
    intake("viewer no private source grant", "sources/get", {"sourceId": a}, role="viewer", expected=404)
    de = intake("empty source detail", "sources/get", {"sourceId": empty})
    ce = intake("empty source candidates", "candidates/list", {"sourceId": empty})
    check("empty summary makes no invented candidate", de["passages"] == [] and ce["items"] == [] and de["source"]["providerSummaryStatus"] == "processing")
    candidate = intake("source A candidate", "candidates/list", {"sourceId": a})["items"][0]
    check("candidate has no automatic public draft", candidate["draft"] is None)
    edit = {"operationId": op(), "candidateId": candidate["id"], "expectedRevision": candidate["revision"], "expectedSourceRevision": da["source"]["revision"], "passageIds": [da["passages"][0]["id"]], "task": {"title": "Reviewer API: customer follow-up", "notes": "Only this deliberately reviewed business brief is shared.", "domain": "feedback", "dueDate": "2026-11-01"}}
    prepared = intake("exact human preparation", "candidates/edit", edit)
    resources["preparedCandidate"] = candidate["id"]
    intake("stale candidate CAS rejected", "candidates/edit", {**edit, "operationId": op()}, expected=409)
    after_stale = intake("candidate after rejected CAS", "candidates/get", {"candidateId": candidate["id"]})
    check("rejected edit left revision and exact draft unchanged", after_stale["candidate"] == prepared["candidate"])
    acceptance = {"operationId": op(), "candidateId": candidate["id"], "expectedRevision": prepared["candidate"]["revision"], "expectedSourceRevision": da["source"]["revision"], "publishToDomain": "feedback"}
    intake("viewer publication denied", "candidates/accept", acceptance, role="viewer", expected=404)
    decision = intake("exact human task publication", "candidates/accept", acceptance)
    task = decision["task"]["task"]
    resources["publishedTask"] = task["id"]
    check("exact public text and authenticated creator", task["notes"] == edit["task"]["notes"] and task["title"] == edit["task"]["title"] and task["creatorId"] == sessions["owner"]["memberId"] and task["organizationId"] == org)
    check("no transcript in public task response", "private evidence" not in json.dumps(decision["task"]) and "private passage" not in json.dumps(decision["task"]))
    replay = intake("receipt replay", "candidates/accept", acceptance)
    check("receipt replays same task and decision", replay["replayed"] is True and replay["task"]["task"]["id"] == task["id"] and replay["decision"]["id"] == decision["decision"]["id"])
    shared = request("viewer shared task positive", "/api/business/tasks/get", {"taskId": task["id"]}, role="viewer")
    check("public task separate from private source", shared["task"]["notes"] == edit["task"]["notes"])
    link = intake("viewer source-link redaction", "tasks/sources", {"taskId": task["id"]}, role="viewer")
    check("withheld source reference", len(link["links"]) == 1 and link["links"][0]["accessible"] is False and link["links"][0]["source"] is None)
    task_list = request("review task count after replay", "/api/business/tasks/list")
    check("no duplicate public task", len(task_list["tasks"]) == 2)
    pending = intake("extra candidate for authority rebase", "candidates/create", {"operationId": op(), "sourceId": a, "expectedSourceRevision": da["source"]["revision"], "passageIds": [da["passages"][0]["id"]]})
    revoked = intake("manager grant revocation", "grants/revoke", {"operationId": op(), "bindingId": bid, "expectedBindingRevision": binding["revision"], "grantId": active["manager"]["id"], "expectedGrantRevision": active["manager"]["revision"]})
    binding = revoked["binding"]
    intake("revoked manager private source denied", "sources/get", {"sourceId": a}, role="manager", expected=404)
    stale = intake("source after binding epoch change", "sources/get", {"sourceId": a})
    check("old observation withheld after grant epoch change", stale["disclosure"] == "metadata_only" and stale["passages"] == [] and stale["source"]["requiresRefresh"])
    da = refresh(bid, a, "fresh source A validation")
    stale_candidate = intake("old candidate after fresh observation", "candidates/get", {"candidateId": pending["id"]})
    check("fresh observation does not refresh old preview authority", stale_candidate["candidate"]["requiresRebase"] and stale_candidate["candidate"]["draft"] is None)
    selected = intake("explicit passage-only rebase", "candidates/select", {"operationId": op(), "candidateId": pending["id"], "expectedRevision": pending["revision"], "expectedSourceRevision": da["source"]["revision"], "passageIds": [da["passages"][0]["id"]]})
    check("passage rebase preserves absent draft", not selected["candidate"]["requiresRebase"] and selected["candidate"]["draft"] is None)
    browser_a = intake("prepare browser A baseline", "candidates/edit", {**edit, "operationId": op(), "candidateId": pending["id"], "expectedRevision": selected["candidate"]["revision"], "expectedSourceRevision": da["source"]["revision"], "passageIds": [da["passages"][0]["id"]], "task": {"title": "Reviewer browser A: customer conversation", "notes": "Reviewer saved candidate A baseline", "domain": "feedback"}})
    resources["browserCandidateA"] = browser_a["candidate"]["id"]
    db = refresh(bid, b, "fresh source B validation")
    cb = intake("source B pending candidate", "candidates/list", {"sourceId": b})["items"][0]
    cb = intake("source B current passage selection", "candidates/select", {"operationId": op(), "candidateId": cb["id"], "expectedRevision": cb["revision"], "expectedSourceRevision": db["source"]["revision"], "passageIds": [db["passages"][0]["id"]]})["candidate"]
    target = request("own source-link target", "/api/business/tasks/get", {"taskId": values["linkTarget"]})["task"]
    linked = intake("human source B task link", "candidates/link", {"operationId": op(), "candidateId": cb["id"], "expectedRevision": cb["revision"], "expectedSourceRevision": db["source"]["revision"], "taskId": target["id"], "expectedTaskRevision": target["revision"], "publishToDomain": "feedback"})
    check("link preserves existing public text", linked["task"]["task"]["notes"] == target["notes"])
    own_links = intake("actual task-source B browser route", "tasks/sources", {"taskId": target["id"]})
    check("real task link resolves exact source B", len(own_links["links"]) == 1 and own_links["links"][0]["accessible"] and own_links["links"][0]["source"]["id"] == b)
    resources["browserLinkCandidateB"] = cb["id"]
    report["endCountsSharedNotExclusive"] = request("fixture retained healthy", "/__business_intake_fixture/health", role=None, method="GET")["counts"]
    report["status"] = "passed"


if __name__ == "__main__":
    if RESULT.exists() or RESOURCES.exists():
        raise SystemExit("Existing probe evidence retained; do not blindly rerun mutations.")
    began = time.monotonic()
    try:
        run()
    except Exception as error:
        report["status"] = "failed"
        report["errorType"] = type(error).__name__
        # No arbitrary exception text, response data or credentials in artifacts.
        if isinstance(error, ProbeFailure):
            report["failedLabel"] = str(error)
        raise SystemExit(1) from None
    finally:
        report["elapsedSeconds"] = round(time.monotonic() - began, 4)
        report["probeSha256"] = hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest()
        save()
        print(json.dumps({"status": report["status"], "requests": len(report["requests"]), "assertions": len(report["assertions"]), "failedLabel": report.get("failedLabel"), "errorType": report.get("errorType")}))
