import { describe, expect, it } from "vitest"
import {
  retained,
  outputCandidate,
  fixtureTaskId,
} from "@/components/business-execution/asset-test-fixtures"
import {
  parseAssetPage,
  parseAssetResult,
  parseOutputPage,
  parseVersionPage,
} from "./assets"
import { MAX_ASSET_BYTES } from "./content"

describe("exact private asset projection", () => {
  function parse(value: unknown) {
    return parseAssetResult(value, {
      taskId: fixtureTaskId,
      assetId: "synthetic-asset",
      versionId: "synthetic-version",
    })
  }
  it("keeps the selected immutable version when the asset's latest pointer advances", () => {
    const value = retained()
    value.asset.latestVersionId = "synthetic-newer-version"
    expect(parse(value).version.id).toBe("synthetic-version")
    expect(parse(value).asset.latestVersionId).toBe("synthetic-newer-version")
  })
  it.each(["task", "asset", "version", "reference"])(
    "withholds a foreign %s binding",
    (kind) => {
      const value = retained()
      if (kind === "task") value.asset.taskId = "foreign-task"
      if (kind === "asset") value.version.assetId = "foreign-asset"
      if (kind === "version") value.version.id = "foreign-version"
      if (kind === "reference")
        value.version.reviewReferences.push({
          taskId: "foreign-task",
          deliverableId: "foreign-deliverable",
          taskRevision: 1,
        })
      expect(() => parse(value)).toThrow()
    }
  )
  it.each(["hash", "too_large", "negative"])(
    "refuses invalid retained-byte metadata: %s",
    (kind) => {
      const value = retained()
      if (kind === "hash") value.version.sha256 = "not-a-digest"
      if (kind === "too_large") value.version.byteSize = MAX_ASSET_BYTES + 1
      if (kind === "negative") value.version.byteSize = -1
      expect(() => parse(value)).toThrow()
    }
  )
  it("refuses extra host fields even in otherwise valid private metadata", () => {
    const value = retained()
    expect(() =>
      parse({ ...value, hostPath: "/synthetic/not-a-real-file" })
    ).toThrow()
    expect(() =>
      parse({
        ...value,
        version: {
          ...value.version,
          producer: {
            ...value.version.producer,
            credential: "synthetic-hidden-field",
          },
        },
      })
    ).toThrow()
  })
  it("checks every task asset and every version in a bounded opaque page", () => {
    const value = retained()
    expect(
      parseAssetPage(
        { items: [value.asset], nextCursor: "opaque-next" },
        fixtureTaskId
      ).nextCursor
    ).toBe("opaque-next")
    expect(() =>
      parseAssetPage(
        { items: [{ ...value.asset, taskId: "foreign" }], nextCursor: null },
        fixtureTaskId
      )
    ).toThrow()
    expect(() =>
      parseVersionPage(
        { items: [{ ...value.version, assetId: "foreign" }], nextCursor: null },
        { taskId: fixtureTaskId, assetId: value.asset.id }
      )
    ).toThrow()
    expect(() =>
      parseVersionPage(
        {
          items: Array.from({ length: 51 }, () => value.version),
          nextCursor: null,
        },
        { taskId: fixtureTaskId, assetId: value.asset.id }
      )
    ).toThrow()
  })
  it("uses output handles and known availability only, never accepts source paths", () => {
    const output = outputCandidate()
    expect(
      parseOutputPage({
        items: [{ ...output, status: "unsupported" }],
        nextCursor: null,
      }).items[0].status
    ).toBe("unsupported")
    expect(() =>
      parseOutputPage({
        items: [{ ...output, path: "/synthetic/forbidden" }],
        nextCursor: null,
      })
    ).toThrow()
    expect(() =>
      parseOutputPage({
        items: [{ ...output, status: "completed" }],
        nextCursor: null,
      })
    ).toThrow()
  })
})
