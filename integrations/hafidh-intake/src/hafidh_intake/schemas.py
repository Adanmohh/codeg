"""Strict projections of Hafidh's TestFlight schemas; see NOTICE for source SHA."""
from enum import Enum
from typing import Annotated, Generic, Literal, TypeVar

from pydantic import BaseModel, ConfigDict, Field

Source = Literal["in_app", "testflight"]
Status = Literal["pending", "in_progress", "resolved", "closed"]
SeverityValue = Literal["high", "medium", "low"]
Ulid = Annotated[str, Field(pattern=r"^[0-7][0-9A-HJKMNP-TV-Z]{25}$")]
Identifier = Annotated[str, Field(pattern=r"^[A-Za-z0-9_-]{1,100}$")]
Text = Annotated[str, Field(max_length=20000)]


class Severity(str, Enum):
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"


class StrictModel(BaseModel):
    model_config = ConfigDict(extra="forbid", strict=True, hide_input_in_errors=True)


class SourceRef(StrictModel):
    product_id: Identifier
    source: Source
    ulid: Ulid
    external_id: Identifier | None = None


class ListRequest(StrictModel):
    source: Source
    status: Status | None = None
    severity: SeverityValue | None = None
    tag: Annotated[str, Field(max_length=50)] | None = None
    query: Annotated[str, Field(max_length=200)] | None = None
    limit: Annotated[int, Field(ge=1, le=100)] = 100
    cursor: Annotated[str, Field(max_length=100)] | None = None


class Screenshot(StrictModel):
    ulid: Ulid
    content_type: Annotated[str, Field(pattern=r"^image/[a-zA-Z0-9.+-]+$")]
    width: Annotated[int, Field(gt=0)] | None = None
    height: Annotated[int, Field(gt=0)] | None = None


class TriageProjection(StrictModel):
    seeded_tags: list[str]
    seeded_severity: SeverityValue
    source_tags: list[str]
    source_severity: SeverityValue
    confirmed_tags: list[str] | None = None
    confirmed_severity: SeverityValue | None = None


class EvidenceCandidate(StrictModel):
    field: Literal["build"]
    value: str
    provenance: Literal["testflight.buildVersion"] = "testflight.buildVersion"


class IntakeRecordV1(StrictModel):
    schema_version: Literal[1] = 1
    source_ref: SourceRef
    source_revision: Annotated[str, Field(pattern=r"^[0-9a-f]{64}$")]
    fetched_at: str
    title: str
    title_is_draft: Literal[True] = True
    description: str
    feedback_type: None = None
    source_status: Status
    submitted_at: str | None
    source_updated_at: str
    device: str | None
    os_version: str | None
    app_version: None = None
    build_number: str | None
    platform: str | None
    locale: str | None
    screenshots: list[Screenshot]
    triage: TriageProjection
    evidence_candidates: list[EvidenceCandidate]
    missing_required: list[Literal["build", "screen", "reciter", "log"]]


class Page(StrictModel):
    items: list[IntakeRecordV1]
    next_cursor: str | None
    scan_complete: bool
    fetched_at: str
    source_health: Literal["ok"] = "ok"


class IntakeStatus(StrictModel):
    configured: bool
    testflight_available: bool
    in_app_available: Literal[False] = False
    last_successful_scan: str | None
    sync_enabled: bool
    sync_state: Literal["never", "success", "failed", "skipped_locked"]
    last_sync_finished_at: str | None


class ErrorInfo(StrictModel):
    code: str
    message: str


T = TypeVar("T")


class Result(StrictModel, Generic[T]):
    status: Literal["ok", "unavailable", "error"]
    value: T | None = None
    error: ErrorInfo | None = None


# Upstream may add fields. Project its allowlist BEFORE parsing, never retain
# contact fields, operator notes or signed URLs on the normalized model.
class TestFlightRow(StrictModel):
    ulid: Ulid
    ascSubmissionId: Identifier
    comment: Text | None = None
    deviceModel: Text | None = None
    osVersion: Text | None = None
    locale: Text | None = None
    buildVersion: Text | None = None
    appPlatform: Text | None = None
    submittedAt: str | None = None
    status: Status
    severity: SeverityValue
    tags: list[Annotated[str, Field(max_length=100)]] = Field(default_factory=list, max_length=20)
    screenshots: list[Screenshot] = Field(default_factory=list, max_length=50)
    lastModifiedAt: str
