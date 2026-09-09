import { useState } from "react"
import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { describe, expect, it } from "vitest"
import { Modal } from "./ui"
import { OverlayHostHiddenProvider } from "@/components/ui/overlay-host-hidden"

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
  it("hides a background pane's portal without discarding the parent's private draft", () => {
    const closed = () => {
      throw new Error("Hiding a pane must not close its editor")
    }
    function Pane({ hidden }: { hidden: boolean }) {
      const [note, setNote] = useState("")
      return (
        <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
          <OverlayHostHiddenProvider hidden={hidden}>
            <Modal title="Synthetic pane dialog" onClose={closed}>
              <input
                aria-label="Synthetic draft"
                value={note}
                onChange={(event) => setNote(event.target.value)}
              />
            </Modal>
          </OverlayHostHiddenProvider>
        </NextIntlClientProvider>
      )
    }
    const view = render(<Pane hidden={false} />)
    fireEvent.change(screen.getByRole("textbox"), {
      target: { value: "Synthetic private note" },
    })
    view.rerender(<Pane hidden />)
    expect(screen.queryByRole("dialog")).toBeNull()
    view.rerender(<Pane hidden={false} />)
    expect(screen.getByRole("textbox")).toHaveValue("Synthetic private note")
  })
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
