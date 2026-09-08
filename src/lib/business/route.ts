// Business members have a separate authenticated surface. This routing hint
// only isolates inherited operator UI; authorization remains on the server.
export function isBusinessPath(pathname: string | null): boolean {
  return (
    pathname === "/" ||
    pathname === "/index.html" ||
    pathname === "/business" ||
    pathname === "/business.html" ||
    !!pathname?.startsWith("/business/")
  )
}
