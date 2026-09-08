// Acceptance-only native keyboard input. Apple SDK CGEvent.h is the source.
// Read synthetic text from stdin, never command arguments/clipboard/log output.
import AppKit
import CoreGraphics
let a = CommandLine.arguments
guard a.count >= 3, let pid = Int32(a[1]),
      let app = NSRunningApplication(processIdentifier: pid), app.isActive else { exit(2) }
func key(_ code: CGKeyCode, _ flags: CGEventFlags = []) {
    for down in [true, false] {
        guard app.isActive, let e = CGEvent(keyboardEventSource:nil, virtualKey:code,keyDown:down) else { exit(3) }
        e.flags = flags; e.post(tap:.cghidEventTap)
        Thread.sleep(forTimeInterval:0.02)
    }
}
if a[2] == "text" {
    guard let text = readLine() else { exit(2) }
    for char in text {
        let units = Array(String(char).utf16)
        for down in [true,false] {
            guard app.isActive, let e = CGEvent(keyboardEventSource:nil,virtualKey:0,keyDown:down) else { exit(3) }
            e.keyboardSetUnicodeString(stringLength:units.count,unicodeString:units)
            e.post(tap:.cghidEventTap)
        }
        Thread.sleep(forTimeInterval:0.005)
    }
} else if a[2] == "select-all" { key(0,.maskCommand)
} else if a.count == 4, let code = UInt16(a[3]) { key(code)
} else { exit(2) }
print("Native keyboard action posted")
