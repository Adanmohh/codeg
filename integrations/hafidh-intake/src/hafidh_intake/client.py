"""GET-only adapter of Hafidh's existing admin contracts. No app imports.

Offset cursors are process-local scan continuations, never durable sync tokens.
All output text is untrusted source material, not tool instructions.
"""
import hashlib
import json
import os
import re
import secrets
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from urllib.parse import urlsplit

import httpx
from pydantic import ValidationError

from .schemas import (ErrorInfo, EvidenceCandidate, IntakeRecordV1, IntakeStatus,
                      ListRequest, Page, Result, SourceRef, TestFlightRow, TriageProjection)
from .triage import classify

MAX_PAGES = 5
MAX_RESPONSE = 2 * 1024 * 1024
MESSAGES = {
    "not_configured": "Intake access is not configured.",
    "source_unavailable": "In-app feedback has no read endpoint.",
    "access_denied": "Intake access was denied.",
    "invalid_cursor": "Restart this bounded scan.",
    "invalid_source": "Source data does not match the read contract.",
    "stale_source": "The selected record could not be revalidated.",
    "upstream_unavailable": "The intake source is unavailable.",
}


class IntakeFailure(Exception):
    def __init__(self, code):
        self.code = code
        super().__init__(MESSAGES[code])


def failure(code):
    return Result(status="unavailable" if code in ("not_configured", "source_unavailable") else "error",
                  error=ErrorInfo(code=code, message=MESSAGES[code]))


def timestamp(value):
    if not isinstance(value, str):
        raise ValueError("invalid timestamp")
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("timezone required")
    return parsed.astimezone(timezone.utc).isoformat()


def now():
    return datetime.now(timezone.utc).isoformat()


def sanitize(value, identities=()):
    if value is None:
        return None
    for identity in identities:
        if isinstance(identity, str) and identity.strip():
            value = re.sub(re.escape(identity), "[private]", value, flags=re.I)
    patterns = (
        r"-----BEGIN [^-]*PRIVATE KEY-----.*?-----END [^-]*PRIVATE KEY-----",
        r"\bBearer\s+\S+", r"\b(?:gh[pousr]_[A-Za-z0-9_]+|github_pat_[A-Za-z0-9_]+)",
        r"\beyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+",
        r"[A-Za-z0-9.!#$%&'*+/=?^_`{|}~-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}",
        r"(?:https?://|s3://|hf://|/Users/|/home/|[A-Za-z]:\\)\S+",
        r"\b(?:password|secret|token|api[_-]?key)\s*[:=]\s*\S+",
    )
    for pattern in patterns:
        value = re.sub(pattern, "[private]", value, flags=re.I | re.S)
    return "".join(c for c in value if c in "\n\t" or ord(c) >= 32)


@dataclass(frozen=True, repr=False)
class Settings:
    origin: str = ""
    product_id: str = ""
    bearer: str = field(default="", repr=False)

    @classmethod
    def from_environment(cls):
        return cls(*(os.environ.get(name, "") for name in (
            "HAFIDH_INTAKE_ORIGIN", "HAFIDH_INTAKE_PRODUCT_ID", "HAFIDH_INTAKE_BEARER")))

    @property
    def configured(self):
        try:
            p = urlsplit(self.origin)
            return bool(self.bearer.strip() and re.fullmatch(r"[A-Za-z0-9_-]{1,100}", self.product_id)
                        and p.hostname and not p.username and not p.password
                        and p.path in ("", "/") and not p.query and not p.fragment
                        and (p.scheme == "https" or (p.scheme == "http" and p.hostname in ("127.0.0.1", "::1"))))
        except ValueError:
            return False


class IntakeClient:
    def __init__(self, settings, *, transport=None):
        self.settings = settings
        self.transport = transport
        self._cursors = {}
        self._listed = {}
        self.last_scan = None

    async def _get(self, path, params=None):
        if not self.settings.configured:
            raise IntakeFailure("not_configured")
        # Neither tools nor callers choose a method or path.
        if path not in ("/api/v1/admin/testflight", "/api/v1/admin/testflight/sync/status"):
            raise IntakeFailure("access_denied")
        try:
            async with httpx.AsyncClient(transport=self.transport, timeout=30,
                                         follow_redirects=False, trust_env=False) as c:
                async with c.stream("GET", self.settings.origin.rstrip("/") + path,
                                    params=params, headers={"Authorization": "Bearer " + self.settings.bearer}) as r:
                    if r.status_code in (401, 403):
                        raise IntakeFailure("access_denied")
                    if r.status_code != 200:
                        raise IntakeFailure("upstream_unavailable")
                    data = bytearray()
                    async for chunk in r.aiter_bytes():
                        data.extend(chunk)
                        if len(data) > MAX_RESPONSE:
                            raise IntakeFailure("invalid_source")
            envelope = json.loads(data)
            if envelope.get("succeeded") is not True or envelope.get("hasErrors") is not False:
                raise IntakeFailure("upstream_unavailable")
            if not isinstance(envelope.get("value"), dict):
                raise IntakeFailure("invalid_source")
            return envelope["value"]
        except httpx.HTTPError:
            raise IntakeFailure("upstream_unavailable") from None
        except (ValueError, TypeError, AttributeError):
            raise IntakeFailure("invalid_source") from None

    def _normalize(self, raw, fetched):
        projected = {k: v for k, v in raw.items() if k in TestFlightRow.model_fields}
        projected["screenshots"] = [dict(ulid=s["ulid"], content_type=s["contentType"],
                                         width=s.get("width"), height=s.get("height"))
                                    for s in raw.get("screenshots", [])]
        row = TestFlightRow.model_validate(projected)
        identities = [raw.get(k) for k in ("testerEmail", "testerName", "lastModifiedByName")]
        identities.append(self.settings.bearer)
        clean = lambda s: sanitize(s, identities)
        comment = clean(row.comment) or ""
        triage = classify(comment)
        build = clean(row.buildVersion)
        # ASC version is the build number. Never parse a marketing version.
        if not build or not re.fullmatch(r"[0-9]+(?:\.[0-9]+){0,2}", build):
            build = None
        fields = dict(source_ref=SourceRef(product_id=self.settings.product_id, source="testflight",
                                           ulid=row.ulid, external_id=row.ascSubmissionId),
                      title="TestFlight draft: " + (comment.splitlines()[0][:170] if comment else row.ulid),
                      description=comment, source_status=row.status,
                      submitted_at=timestamp(row.submittedAt) if row.submittedAt else None,
                      source_updated_at=timestamp(row.lastModifiedAt), device=clean(row.deviceModel),
                      os_version=clean(row.osVersion), build_number=build, platform=clean(row.appPlatform),
                      locale=clean(row.locale), screenshots=row.screenshots,
                      triage=TriageProjection(seeded_tags=triage.tags, seeded_severity=triage.severity.value,
                                              source_tags=[clean(t) for t in row.tags], source_severity=row.severity),
                      evidence_candidates=[EvidenceCandidate(field="build", value=build)] if build else [],
                      missing_required=(["build"] if not build else []) + ["screen", "reciter", "log"])
        record = IntakeRecordV1(**fields, fetched_at=fetched, source_revision="0" * 64)
        snapshot = record.model_dump(exclude={"fetched_at", "source_revision"})
        record.source_revision = hashlib.sha256(json.dumps(snapshot, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        return record

    async def list_feedback(self, request: ListRequest) -> Result[Page]:
        if request.source == "in_app":
            return failure("source_unavailable")
        try:
            signature = request.model_dump_json(exclude={"cursor"})
            offset, seen, pages = 0, set(), 0
            if request.cursor:
                saved = self._cursors.get(request.cursor)
                if not saved or saved[0] != signature or saved[4] < time.monotonic():
                    return failure("invalid_cursor")
                _, offset, seen, pages, _ = saved
                seen = set(seen)
            params = {"limit": request.limit, "offset": offset}
            for field_name, upstream in (("status", "status"), ("severity", "severity"), ("tag", "tag"), ("query", "q")):
                value = getattr(request, field_name)
                if value is not None:
                    params[upstream] = value
            data = await self._get("/api/v1/admin/testflight", params)
            raw = data["items"]
            if not isinstance(raw, list) or len(raw) > request.limit:
                raise IntakeFailure("invalid_source")
            fetched = now()
            items = []
            for value in raw:
                record = self._normalize(value, fetched)
                if record.source_ref.ulid not in seen:
                    items.append(record)
                    seen.add(record.source_ref.ulid)
                self._listed[record.source_ref.ulid] = (record.source_ref, time.monotonic() + 300)
            # Limit caches and cursor lifetime. A capped scan must restart with overlap.
            self._listed = dict(list(self._listed.items())[-1000:])
            complete = len(raw) < request.limit
            next_cursor = None
            if not complete and pages + 1 < MAX_PAGES:
                next_cursor = secrets.token_urlsafe(24)
                self._cursors[next_cursor] = (signature, offset + len(raw), seen, pages + 1, time.monotonic() + 300)
                self._cursors = dict(list(self._cursors.items())[-128:])
            self.last_scan = fetched
            return Result[Page](status="ok", value=Page(items=items, next_cursor=next_cursor,
                                                       scan_complete=complete, fetched_at=fetched))
        except IntakeFailure as e:
            return failure(e.code)
        except (ValidationError, ValueError, TypeError, KeyError, AttributeError):
            return failure("invalid_source")

    async def get_feedback(self, source_ref: SourceRef) -> Result[IntakeRecordV1]:
        if source_ref.source == "in_app":
            return failure("source_unavailable")
        known = self._listed.get(source_ref.ulid)
        if (source_ref.product_id != self.settings.product_id or not known
                or known[0] != source_ref or known[1] < time.monotonic()):
            return failure("access_denied")
        cursor = None
        for _ in range(MAX_PAGES):
            page = await self.list_feedback(ListRequest(source="testflight", cursor=cursor))
            if page.status != "ok":
                return Result(status=page.status, error=page.error)
            for record in page.value.items:
                if record.source_ref == source_ref:
                    return Result[IntakeRecordV1](status="ok", value=record)
            cursor = page.value.next_cursor
            if cursor is None:
                break
        return failure("stale_source")

    async def status(self) -> Result[IntakeStatus]:
        try:
            data = await self._get("/api/v1/admin/testflight/sync/status")
            run = data.get("lastRun")
            result = IntakeStatus(configured=True, testflight_available=True, last_successful_scan=self.last_scan,
                                  sync_enabled=data["syncEnabled"], sync_state=run["status"] if run else "never",
                                  last_sync_finished_at=timestamp(run["finishedAt"]) if run and run.get("finishedAt") else None)
            return Result[IntakeStatus](status="ok", value=result)
        except IntakeFailure as e:
            return failure(e.code)
        except (ValidationError, KeyError, ValueError, TypeError):
            return failure("invalid_source")
