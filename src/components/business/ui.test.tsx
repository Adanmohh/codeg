import { useState } from "react"
import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { describe, expect, it } from "vitest"
import { Modal } from "./ui"

function FocusExample({ removeOpener }: { removeOpener: boolean }) {
  const [open, setOpen] = useState(false)
  const [removed, setRemoved] = useState(false)
  return (
    <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
      <main id="business-main" tabIndex={-1}>
        {!removed && <button onClick={() => setOpen(true)}>Open task</button>}
      </main>
      {open && (
        <Modal
          title="Synthetic task"
          onClose={() => {
            setOpen(false)
            setRemoved(removeOpener)
          }}
        >
          <p>Task details</p>
        </Modal>
      )}
    </NextIntlClientProvider>
  )
}

describe("controlled business dialog keyboard recovery", () => {
  it("returns to the work area after a programmatic detail handoff with no focused opener", async () => {
    render(<FocusExample removeOpener={false} />)
    expect(document.activeElement).toBe(document.body)
    fireEvent.click(screen.getByRole("button", { name: "Open task" }))
    fireEvent.click(screen.getByRole("button", { name: "Close" }))
    await waitFor(() => expect(screen.getByRole("main")).toHaveFocus())
  })
  it.each([false, true])(
    "restores a useful focus target when the opener disappears: %s",
    async (removeOpener) => {
      render(<FocusExample removeOpener={removeOpener} />)
      const opener = screen.getByRole("button", { name: "Open task" })
      opener.focus()
      fireEvent.click(opener)
      expect(screen.getByRole("dialog")).toBeVisible()
      fireEvent.click(screen.getByRole("button", { name: "Close" }))
      await waitFor(() =>
        expect(removeOpener ? screen.getByRole("main") : opener).toHaveFocus()
      )
      expect(screen.queryByRole("dialog")).toBeNull()
    }
  )
})
