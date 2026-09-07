# Port: Hafidh a83794708193a18611c8e6eb8e88637b8136e471; see NOTICE.
"""
Keyword triage for TestFlight feedback.

Pure functions: no database, no network, no FastAPI. Results are written once at
ingest time and are editable afterwards, because these are guesses. A tester
writing "the audio has to be fixed" is reporting a defect even though the comment
contains none of the obvious severity words, and an operator must be able to
correct the label rather than argue with a regex.
"""

import re
from dataclasses import dataclass

from .schemas import Severity

# Ordered longest-to-shortest only for readability; matching is independent.
TAG_PATTERNS: dict[str, re.Pattern[str]] = {
    "audio": re.compile(r"audio|playback|recit(ation|er)|sound|volume|play sign|player", re.I),
    "recognizer": re.compile(r"recogni[sz]|voice|listen|asr|speech|\bmic\b", re.I),
    "memorization": re.compile(r"memori[sz]ation mode|recall|hidden|unhid|hide", re.I),
    "mushaf": re.compile(r"mushaf|page|layout|font|line break|justif", re.I),
    "counter": re.compile(r"bead|counter|repetition|\d+\s*/\s*\d+", re.I),
    "navigation": re.compile(r"navigat|rewind|forward|scroll|swipe|back button|jump", re.I),
    "ui": re.compile(r"button|screen|text|colou?r|icon|size|align|dark mode|design", re.I),
    "crash": re.compile(r"crash|freeze|frozen|hang|stuck|not respond", re.I),
    "translation": re.compile(r"translat|tafsir|arabic text|typo|spelling", re.I),
}

# First match wins, so the most serious pattern is listed first.
SEVERITY_PATTERNS: list[tuple[re.Pattern[str], Severity]] = [
    (
        re.compile(
            r"crash|freeze|frozen|hang|stuck|not respond|data loss|lost my|wrong verse|incorrect|corrupt",
            re.I,
        ),
        Severity.HIGH,
    ),
    (
        re.compile(
            # "has to be" and the \bfix\w*\b form are what catch reports such as
            # "the audio has to be fixed", which an earlier version rated LOW.
            r"should|\bfix\w*\b|has to be|does ?n[o']?t|do ?n[o']?t|is not|not possible"
            r"|not work|can[' ]?t\b|cannot|bug|wrong|error|fail|missing|instead of",
            re.I,
        ),
        Severity.MEDIUM,
    ),
]

FALLBACK_TAG = "other"


@dataclass(frozen=True)
class Triage:
    """The result of classifying a single comment."""

    tags: list[str]
    severity: Severity


def classify(comment: str | None) -> Triage:
    """
    Derive tags and a severity from a tester's comment.

    An empty or missing comment is tagged `other` at LOW severity rather than
    being left untagged, so it still appears under a filter instead of vanishing.
    """
    text = (comment or "").strip()
    if not text:
        return Triage(tags=[FALLBACK_TAG], severity=Severity.LOW)

    tags = [tag for tag, pattern in TAG_PATTERNS.items() if pattern.search(text)]
    if not tags:
        tags = [FALLBACK_TAG]

    severity = next((sev for pattern, sev in SEVERITY_PATTERNS if pattern.search(text)), Severity.LOW)

    return Triage(tags=tags, severity=severity)
