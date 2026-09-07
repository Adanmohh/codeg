// Original Hafidh Ops Desk vector mark; keep in sync with public/icon.svg.
export function AppIcon({ className }: { className?: string }) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 512 512"
      className={className}
      aria-hidden="true"
    >
      <rect width="512" height="512" rx="90" fill="#123c3a" />
      <path
        d="M152 136V376M360 136V376M152 256H360"
        fill="none"
        stroke="#f5f1e7"
        strokeWidth="48"
        strokeLinecap="round"
      />
      <circle cx="256" cy="256" r="32" fill="#dbb66b" />
    </svg>
  )
}
