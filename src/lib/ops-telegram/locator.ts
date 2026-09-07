// A locator can survive login; it carries no credential or decision authority.
// Accept one canonical UUID only, never a caller-selected redirect destination.
export function reviewNotice(search: string, key = "notice"): string | null {
  const params = new URLSearchParams(search)
  const values = params.getAll(key)
  const value = values[0] ?? ""
  return values.length === 1 &&
    /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(
      value
    )
    ? value
    : null
}
export function reviewLoginPath(pathname: string, search: string): string {
  const notice =
    pathname === "/ops-review" || pathname === "/ops-review/"
      ? reviewNotice(search)
      : null
  return notice ? `/login?opsNotice=${notice}` : "/login"
}
export function afterLoginPath(search: string): string {
  const notice = reviewNotice(search, "opsNotice")
  return notice ? `/ops-review?notice=${notice}` : "/workspace"
}
