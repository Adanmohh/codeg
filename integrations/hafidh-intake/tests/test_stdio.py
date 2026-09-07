"""Real SDK2 stdio discovery and GET, against a synthetic loopback HTTP source."""
import json
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread

from mcp.client import Client
from mcp.client.stdio import StdioServerParameters, stdio_client

from test_intake import response, row


async def test_stdio_discovery_and_get():
    seen = []

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_GET(self):
            seen.append((self.command, self.path))
            assert self.headers["Authorization"] == "Bearer synthetic-stdio-only"
            body = json.dumps(response([row()])).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    with ThreadingHTTPServer(("127.0.0.1", 0), Handler) as http:
        thread = Thread(target=http.serve_forever, daemon=True)
        thread.start()
        params = StdioServerParameters(command=sys.executable, args=["-m", "hafidh_intake.server"],
            env={"HAFIDH_INTAKE_ORIGIN": f"http://127.0.0.1:{http.server_port}",
                 "HAFIDH_INTAKE_PRODUCT_ID": "hafidh", "HAFIDH_INTAKE_BEARER": "synthetic-stdio-only"})
        try:
            async with Client(stdio_client(params)) as mcp:
                tools = (await mcp.list_tools()).tools
                assert {t.name for t in tools} == {"hafidh_feedback_list", "hafidh_feedback_get", "hafidh_intake_status"}
                assert all(t.annotations.read_only_hint for t in tools)
                assert all(t.output_schema for t in tools)
                result = await mcp.call_tool("hafidh_feedback_list", {"request": {"source": "testflight"}})
                assert not result.is_error
                page = result.structured_content
                assert page["status"] == "ok" and page["value"]["items"][0]["build_number"] == "42"
                result = await mcp.call_tool("hafidh_feedback_get", {"source_ref": page["value"]["items"][0]["source_ref"]})
                assert result.structured_content["status"] == "ok"
                unavailable = await mcp.call_tool("hafidh_feedback_list", {"request": {"source": "in_app"}})
                assert unavailable.structured_content["error"]["code"] == "source_unavailable"
                invalid = await mcp.call_tool("hafidh_feedback_list", {"request": {"source": "testflight", "token": "DO_NOT_ECHO"}})
                assert invalid.is_error and "DO_NOT_ECHO" not in str(invalid)
        finally:
            http.shutdown()
            thread.join(timeout=2)
    assert len(seen) == 2
    assert all(method == "GET" and path.startswith("/api/v1/admin/testflight?") for method, path in seen)
