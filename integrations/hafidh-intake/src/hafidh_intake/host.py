"""Closed NDJSON host bridge over the accepted read-only adapter.

This is an owned child process, not an HTTP endpoint or a general tool runner.
Credentials enter through the trusted parent's environment, never the protocol.
"""
import asyncio
import json
import logging
import sys
from typing import Annotated, Literal

from pydantic import Field

from .client import IntakeClient, Settings
from .schemas import ListRequest, SourceRef, StrictModel

MAX_REQUEST = 16 * 1024
MAX_RESPONSE = 2 * 1024 * 1024


class Request(StrictModel):
    request_id: Annotated[int, Field(ge=1)]
    operation: Literal["list", "get", "status"]
    input: dict


def invalid(request_id=None, code="invalid_request"):
    return {"request_id": request_id, "result": {
        "status": "error", "value": None,
        "error": {"code": code, "message": "Intake request could not be completed."},
    }}


async def handle(client: IntakeClient, payload: bytes) -> dict:
    request_id = None
    try:
        if len(payload) > MAX_REQUEST:
            return invalid()
        request = Request.model_validate_json(payload)
        request_id = request.request_id
        if request.operation == "list":
            result = await client.list_feedback(ListRequest.model_validate(request.input))
        elif request.operation == "get":
            result = await client.get_feedback(SourceRef.model_validate(request.input))
        else:
            StrictModel.model_validate(request.input)
            result = await client.status()
        return {"request_id": request_id, "result": result.model_dump(mode="json")}
    except Exception:
        # Validation exceptions contain rejected input; never echo or log them.
        return invalid(request_id)


async def serve(client: IntakeClient, reader, writer):
    while payload := reader.readline(MAX_REQUEST + 1):
        oversized = len(payload) > MAX_REQUEST
        result = invalid() if oversized else await handle(client, payload)
        output = json.dumps(result, separators=(",", ":")).encode()
        if len(output) > MAX_RESPONSE:
            output = json.dumps(invalid(result["request_id"], "response_too_large")).encode()
        writer.write(output + b"\n")
        writer.flush()
        if oversized:
            break


def main():
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("httpcore").setLevel(logging.WARNING)
    asyncio.run(serve(IntakeClient(Settings.from_environment()), sys.stdin.buffer, sys.stdout.buffer))


if __name__ == "__main__":
    main()
