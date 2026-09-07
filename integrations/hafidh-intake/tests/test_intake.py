import httpx
import pytest
from pydantic import ValidationError

from hafidh_intake.client import IntakeClient, Settings
from hafidh_intake.schemas import ListRequest, SourceRef
from hafidh_intake.triage import classify

ULID = "01ARZ3NDEKTSV4RRFFQ69G5FAV"


def row(**overrides):
    return dict(ulid=ULID, ascSubmissionId="asc-1", comment="audio has to be fixed",
                testerEmail="tester@example.invalid", testerName="Private Tester",
                deviceModel="iPhone", osVersion="18", locale="en", buildVersion="42",
                appPlatform="IOS", submittedAt=None, status="pending", severity="medium",
                tags=["audio"], notes="PRIVATE NOTES", screenshots=[],
                lastModifiedAt="2026-09-07T10:00:00Z", lastModifiedByName="Private Operator",
                **overrides)


def response(items):
    return {"succeeded": True, "hasErrors": False, "value": {
        "items": items, "total": 9999, "open": 9999, "resolved": 0,
        "highSeverityOpen": 0, "tagCounts": {}}}


def client(handler, configured=True):
    return IntakeClient(Settings("https://fixture.invalid", "hafidh", "fixture-token")
                        if configured else Settings(), transport=httpx.MockTransport(handler))


async def test_normalization_privacy_and_read_only():
    requests = []
    def handler(req):
        requests.append(req)
        return httpx.Response(200, json=response([row()]))
    c = client(handler)
    result = await c.list_feedback(ListRequest(source="testflight"))
    assert result.status == "ok"
    record = result.value.items[0]
    assert record.build_number == "42" and record.submitted_at is None
    assert record.missing_required == ["screen", "reciter", "log"]
    assert record.triage.confirmed_severity is None
    assert record.triage.seeded_severity == "medium"
    assert result.value.scan_complete  # ignores whole-inbox total
    encoded = result.model_dump_json()
    for private in ("tester@example.invalid", "Private Tester", "PRIVATE NOTES",
                    "Private Operator", "fixture-token", "testerEmail"):
        assert private not in encoded
    reread = await c.get_feedback(record.source_ref)
    assert reread.status == "ok" and len(requests) == 2
    assert all(r.method == "GET" and r.url.path == "/api/v1/admin/testflight" for r in requests)


@pytest.mark.parametrize("status", [401, 403, 302, 500])
async def test_failures_are_sanitized(status):
    c = client(lambda r: httpx.Response(status, json={"error": "fixture-token PRIVATE"}))
    result = await c.list_feedback(ListRequest(source="testflight"))
    assert result.status == "error"
    assert result.error.code == ("access_denied" if status in (401, 403) else "upstream_unavailable")
    assert "PRIVATE" not in result.model_dump_json()


async def test_no_config_and_no_in_app_route():
    def forbidden(r):
        pytest.fail("unavailable source must not request HTTP")
    c = client(forbidden, configured=False)
    assert (await c.list_feedback(ListRequest(source="testflight"))).error.code == "not_configured"
    assert (await c.list_feedback(ListRequest(source="in_app"))).error.code == "source_unavailable"
    assert (await c.status()).error.code == "not_configured"


async def test_failed_envelope_and_invalid_dto():
    c = client(lambda r: httpx.Response(200, json={"succeeded": False, "errors": ["PRIVATE"]}))
    assert (await c.list_feedback(ListRequest(source="testflight"))).status == "error"
    bad = row()
    bad["buildVersion"] = 42  # strict source shape; do not coerce
    c = client(lambda r: httpx.Response(200, json=response([bad])))
    assert (await c.list_feedback(ListRequest(source="testflight"))).error.code == "invalid_source"


async def test_cursor_binding_dedup_and_revalidation_failure():
    requests = []
    def handler(r):
        requests.append(r)
        return httpx.Response(200, json=response([row(), row()]))
    c = client(handler)
    result = await c.list_feedback(ListRequest(source="testflight", limit=2))
    assert len(result.value.items) == 1 and not result.value.scan_complete
    assert result.value.next_cursor
    wrong = await c.list_feedback(ListRequest(source="testflight", limit=2, status="closed",
                                               cursor=result.value.next_cursor))
    assert wrong.error.code == "invalid_cursor" and len(requests) == 1
    c.transport = httpx.MockTransport(lambda r: httpx.Response(403))
    assert (await c.get_feedback(result.value.items[0].source_ref)).error.code == "access_denied"
    assert (await c.get_feedback(SourceRef(product_id="different", source="testflight", ulid=ULID))).error.code == "access_denied"


async def test_free_text_redaction_and_status_no_raw_error():
    data = row()
    data["comment"] = "Private Tester tester@example.invalid https://host.invalid/log?token=private Bearer abc.def.ghi"
    c = client(lambda r: httpx.Response(200, json=response([data])))
    record = (await c.list_feedback(ListRequest(source="testflight"))).value.items[0]
    encoded = record.model_dump_json()
    for private in ("Private Tester", "tester@example.invalid", "host.invalid", "abc.def.ghi"):
        assert private not in encoded
    c.transport = httpx.MockTransport(lambda r: httpx.Response(200, json={"succeeded": True,
        "hasErrors": False, "value": {"syncEnabled": True, "lastRun": {
            "status": "failed", "finishedAt": "2026-09-07T10:00:00Z", "error": "PRIVATE"}}}))
    result = await c.status()
    assert result.value.sync_state == "failed" and "PRIVATE" not in result.model_dump_json()


def test_strict_input_and_exact_triage_port():
    with pytest.raises(ValidationError):
        ListRequest(source="testflight", token="DO NOT ECHO")
    with pytest.raises(ValidationError):
        ListRequest(source="testflight", limit="10")
    assert classify(None).tags == ["other"]
    assert classify("the audio has to be fixed").severity.value == "medium"
    assert classify("wrong verse").severity.value == "high"


async def test_bounded_scan_and_get_missing_record_do_not_claim_complete():
    requested = []
    def handler(r):
        requested.append(int(r.url.params["offset"]))
        return httpx.Response(200, json=response([row()]))
    c = client(handler)
    cursor = None
    for _ in range(5):
        page = await c.list_feedback(ListRequest(source="testflight", limit=1, cursor=cursor))
        assert page.status == "ok" and not page.value.scan_complete
        cursor = page.value.next_cursor
    assert cursor is None and requested == [0, 1, 2, 3, 4]
    source_ref = SourceRef(product_id="hafidh", source="testflight", ulid=ULID, external_id="asc-1")
    c.transport = httpx.MockTransport(lambda r: httpx.Response(200, json=response([])))
    assert (await c.get_feedback(source_ref)).error.code == "stale_source"


async def test_malformed_json_oversize_and_redirect_never_return_source_details():
    for reply in (httpx.Response(200, content=b"PRIVATE invalid json"),
                  httpx.Response(200, content=b"x" * (2 * 1024 * 1024 + 1)),
                  httpx.Response(302, headers={"Location": "https://private.invalid/steal"})):
        requests = []
        def handler(r):
            requests.append(r)
            return reply
        c = client(handler)
        result = await c.list_feedback(ListRequest(source="testflight"))
        assert result.status == "error" and len(requests) == 1
        assert "PRIVATE" not in result.model_dump_json()


def test_configuration_cannot_smuggle_paths_auth_or_plain_http():
    for origin in ("http://remote.invalid", "https://user:pass@host.invalid",
                   "https://host.invalid/path", "https://host.invalid?token=private"):
        assert not Settings(origin, "hafidh", "synthetic").configured
    assert "synthetic" not in repr(Settings("https://host.invalid", "hafidh", "synthetic"))
