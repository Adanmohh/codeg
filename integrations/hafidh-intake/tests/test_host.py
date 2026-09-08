"""Closed host process boundary; only synthetic loopback GET fixtures."""
import io
import json
import subprocess
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread

import httpx
import pytest

from hafidh_intake.client import IntakeClient, Settings
from hafidh_intake.host import handle, serve
from test_intake import ULID, response, row


def request(operation, data, request_id=1):
    return json.dumps({"request_id": request_id, "operation": operation, "input": data}).encode()


@pytest.mark.parametrize("payload", [
    b"not JSON",
    request("create_issue", {}),
    request("list", {"source": "testflight", "bearer": "DO_NOT_ECHO"}),
    request("status", {"url": "https://DO_NOT_ECHO.invalid"}),
    b'{"request_id":1,"operation":"status","input":{},"actor":"DO_NOT_ECHO"}',
])
async def test_host_rejects_unowned_operations_and_credentials(payload):
    def forbidden(_):
        pytest.fail("rejected input reached HTTP")
    client = IntakeClient(Settings("https://fixture.invalid", "hafidh", "synthetic"),
                          transport=httpx.MockTransport(forbidden))
    result = await handle(client, payload)
    assert result["result"]["error"]["code"] == "invalid_request"
    assert "DO_NOT_ECHO" not in json.dumps(result)


async def test_host_oversized_frame_fails_and_closes_without_parsing_remainder():
    output = io.BytesIO()
    await serve(IntakeClient(Settings()), io.BytesIO(b"x" * 20000 + b"\n" + request("status", {})), output)
    lines = output.getvalue().splitlines()
    assert len(lines) == 1
    assert json.loads(lines[0])["result"]["error"]["code"] == "invalid_request"


async def test_host_failed_revalidation_never_returns_cached_success():
    client = IntakeClient(Settings("https://fixture.invalid", "hafidh", "synthetic"),
                          transport=httpx.MockTransport(lambda _: httpx.Response(200, json=response([row()]))))
    page = await handle(client, request("list", {"source": "testflight"}))
    ref = page["result"]["value"]["items"][0]["source_ref"]
    client.transport = httpx.MockTransport(lambda _: httpx.Response(403, text="DO_NOT_ECHO"))
    result = await handle(client, request("get", ref, 2))
    assert result["request_id"] == 2
    assert result["result"]["error"]["code"] == "access_denied"
    assert result["result"]["value"] is None
    assert "DO_NOT_ECHO" not in json.dumps(result)


def test_actual_host_process_reads_and_revalidates_without_private_output():
    seen = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_GET(self):
            seen.append((self.command, self.path))
            assert self.headers["Authorization"] == "Bearer synthetic-host-only"
            body = json.dumps(response([row()])).encode()
            self.send_response(200)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    with ThreadingHTTPServer(("127.0.0.1", 0), Handler) as http:
        thread = Thread(target=http.serve_forever, daemon=True)
        thread.start()
        ref = {"product_id": "hafidh", "source": "testflight", "ulid": ULID, "external_id": "asc-1"}
        commands = b"\n".join([request("list", {"source": "testflight"}), request("get", ref, 2)]) + b"\n"
        try:
            process = subprocess.Popen([sys.executable, "-I", "-m", "hafidh_intake.host"],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                env={"HAFIDH_INTAKE_ORIGIN": f"http://127.0.0.1:{http.server_port}",
                     "HAFIDH_INTAKE_PRODUCT_ID": "hafidh", "HAFIDH_INTAKE_BEARER": "synthetic-host-only"})
            stdout, stderr = process.communicate(commands, timeout=10)
        finally:
            http.shutdown()
            thread.join(timeout=2)
    assert process.returncode == 0 and stderr == b""
    replies = [json.loads(line) for line in stdout.splitlines()]
    assert [reply["request_id"] for reply in replies] == [1, 2]
    first = replies[0]["result"]["value"]["items"][0]
    assert replies[1]["result"]["value"]["source_revision"] == first["source_revision"]
    assert replies[1]["result"]["value"]["build_number"] == "42"
    assert len(seen) == 2 and all(method == "GET" for method, _ in seen)
    assert all(path.startswith("/api/v1/admin/testflight?") for _, path in seen)
    for private in (b"synthetic-host-only", b"tester@example.invalid", b"Private Tester", b"PRIVATE NOTES"):
        assert private not in stdout


def test_actual_host_process_missing_config_is_explicit():
    process = subprocess.Popen([sys.executable, "-I", "-m", "hafidh_intake.host"],
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={})
    stdout, stderr = process.communicate(request("status", {}) + b"\n", timeout=10)
    assert process.returncode == 0 and stderr == b""
    assert json.loads(stdout)["result"]["error"]["code"] == "not_configured"
