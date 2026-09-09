"""Owned review namespace only, fixed synthetic4351 API, no secret output.

Adapted from protected-api.py at Codeg6895bedf (Apache-2.0), see NOTICE.
No database, fixture control, token mutation, launch, send or retry transport.
"""
import http.client
import json
from pathlib import Path
import stat
import sys
import time
import uuid

HERE = Path(__file__).resolve().parent
CREDENTIALS = Path("/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/review-credentials.json")
PRIOR = HERE.parent / "fixture-7ed0dd0c/review-resources.json"
STATE = HERE / "browser-cases.json"
action = sys.argv[1]
assert action in ("prepare", "newer", "verify")
LOG = HERE / ("api-" + action + ".json")
assert not LOG.exists(), "Preserve prior attempt; inspect before any retry"
mode = CREDENTIALS.stat().st_mode
assert stat.S_ISREG(mode) and stat.S_IMODE(mode) == 0o600
credentials = json.loads(CREDENTIALS.read_text())
assert credentials["synthetic"] and credentials["namespace"] == "review"
owner = credentials["sessions"]["owner"]
secrets = [entry["token"] for entry in credentials["sessions"].values()]
secrets += [credentials["firefliesApiKey"], credentials["controlToken"]]
prior = json.loads(PRIOR.read_text())
assert prior["organizationId"] == credentials["organization"]["id"]
state = json.loads(STATE.read_text()) if STATE.exists() else {"namespace": "review"}
report = {"action": action, "namespace": "review", "requests": [], "status": "running", "httpRetries": 0}


def save():
    LOG.write_text(json.dumps(report, indent=2) + "\n")
    STATE.write_text(json.dumps(state, indent=2) + "\n")


def call(operation, data=None):
    assert operation in ("context", "intake/sources/get", "intake/candidates/get", "intake/candidates/list", "intake/imports/start", "intake/imports/advance", "intake/candidates/create", "intake/candidates/edit", "tasks/get", "tasks/list")
    payload = {"input": data or {}}
    item = {"operation": operation, "operationId": (data or {}).get("operationId"), "status": "not_dispatched"}
    report["requests"].append(item)
    save()
    conn = http.client.HTTPConnection("127.0.0.1", 4351, timeout=20)
    try:
        conn.request("POST", "/api/business/" + operation, body=json.dumps(payload), headers={"Content-Type": "application/json", "Authorization": "Bearer " + owner["token"]})
        response = conn.getresponse()
        raw = response.read(2 * 1024 * 1024 + 1)
        item["status"] = response.status
        save()
        assert response.status == 200 and len(raw) <= 2 * 1024 * 1024
        assert all(secret.encode() not in raw for secret in secrets)
        return json.loads(raw)
    finally:
        conn.close()


def op():
    return str(uuid.uuid4())


def refresh(source_id):
    job = call("intake/imports/start", {"operationId": op(), "bindingId": prior["bindingId"], "selection": {"kind": "record", "sourceId": source_id}})
    for _ in range(8):
        if job["state"] == "complete":
            break
        assert job["capabilities"]["advance"]
        job = call("intake/imports/advance", {"operationId": op(), "importId": job["id"], "expectedRevision": job["revision"]})
    assert job["state"] == "complete"
    result = call("intake/sources/get", {"sourceId": source_id})
    assert result["disclosure"] == "fresh"
    assert result["source"]["bindingId"] == prior["bindingId"]
    return result


def create(source):
    return call("intake/candidates/create", {"operationId": op(), "sourceId": source["source"]["id"], "expectedSourceRevision": source["source"]["revision"], "passageIds": [source["passages"][0]["id"]]})


def prepare(candidate_id, revision, source, title, notes):
    return call("intake/candidates/edit", {"operationId": op(), "candidateId": candidate_id, "expectedRevision": revision, "expectedSourceRevision": source["source"]["revision"], "passageIds": [source["passages"][0]["id"]], "task": {"title": title, "notes": notes, "domain": "feedback"}})


try:
    context = call("context")
    assert context["operator"] is False and context["organization"]["id"] == prior["organizationId"] and context["member"]["id"] == owner["memberId"]
    if action == "prepare":
        assert "unprepared" not in state and "prepared" not in state
        source = refresh(prior["sourceA"])
        candidate = create(source)
        assert candidate["draft"] is None and not candidate["hasPreparedDraft"]
        state["unprepared"] = {"sourceId": source["source"]["id"], "sourceTitle": source["source"]["title"], "sourceRevision": source["source"]["revision"], "accessValidUntil": source["source"]["accessValidUntil"], "candidateId": candidate["id"], "revision": candidate["revision"]}
        save()
        source = refresh(prior["sourceB"])
        candidate = create(source)
        current = prepare(candidate["id"], candidate["revision"], source, "Reviewer4355: prepared recovery control", "Reviewer4355 initial private prepared text")
        state["prepared"] = {"sourceId": source["source"]["id"], "sourceTitle": source["source"]["title"], "sourceRevision": source["source"]["revision"], "accessValidUntil": source["source"]["accessValidUntil"], "candidateId": candidate["id"], "revision": current["candidate"]["revision"], "title": current["candidate"]["draft"]["title"], "notes": current["candidate"]["draft"]["notes"]}
        save()
    elif action == "newer":
        value = state["prepared"]
        source = refresh(value["sourceId"])
        latest = call("intake/candidates/get", {"candidateId": value["candidateId"]})
        assert latest["candidate"]["revision"] == value["revision"]
        current = prepare(value["candidateId"], value["revision"], source, value["title"], "Reviewer4355 newer exact private saved text")
        state["preparedNewer"] = {"candidateId": value["candidateId"], "revision": current["candidate"]["revision"], "notes": current["candidate"]["draft"]["notes"]}
    else:
        unprepared = call("intake/candidates/get", {"candidateId": state["unprepared"]["candidateId"]})
        prepared = call("intake/candidates/get", {"candidateId": state["prepared"]["candidateId"]})
        old = call("intake/candidates/get", {"candidateId": prior["browserCandidateA"]})
        task = call("tasks/get", {"taskId": prior["publishedTask"]})
        tasks = call("tasks/list")
        report["checks"] = {
            "unpreparedNotSaved": unprepared["candidate"]["revision"] == state["unprepared"]["revision"] and unprepared["candidate"]["draft"] is None,
            "preparedExactRevision": prepared["candidate"]["revision"] == state["preparedNewer"]["revision"],
            "priorCandidateRevisionUnchanged": old["candidate"]["revision"] == 3,
            "priorSharedTextUnchanged": task["task"]["notes"] == "Only this deliberately reviewed business brief is shared.",
            "noPublicTaskCreated": len(tasks["tasks"]) == 2,
        }
        assert all(report["checks"].values())
    report["status"] = "passed"
except Exception as error:
    report["status"] = "failed"
    report["errorType"] = type(error).__name__
    raise SystemExit(1) from None
finally:
    report["finishedAtUnix"] = time.time()
    save()
    print(json.dumps({"status": report["status"], "action": action, "requests": len(report["requests"]), "checks": report.get("checks"), "errorType": report.get("errorType")}))
