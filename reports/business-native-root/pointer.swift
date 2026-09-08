// Acceptance-only native pointer input; explicitly supplied owned PID and points.
// Apple SDK NSRunningApplication.h and CGEvent.h read before use.
import AppKit
import CoreGraphics
let a = CommandLine.arguments
guard a.count == 4 || a.count == 6, let pid = Int32(a[1]), pid > 0,
      let x = Double(a[2]), let y = Double(a[3]),
      let app = NSRunningApplication(processIdentifier: pid) else { exit(2) }
let activated = app.activate(options: [])
print("activationRequested=\(activated)")
Thread.sleep(forTimeInterval: 0.5)
print("active=\(app.isActive)")
guard app.isActive else { exit(4) }
func send(_ type: CGEventType, _ point: CGPoint) {
    guard let event = CGEvent(mouseEventSource: nil, mouseType: type,
                              mouseCursorPosition: point, mouseButton: .left) else { exit(3) }
    guard app.isActive else { exit(4) }
    event.post(tap: .cghidEventTap)
}
let start = CGPoint(x: x, y: y)
send(.leftMouseDown, start)
Thread.sleep(forTimeInterval: 0.1)
var end = start
if a.count == 6 {
    guard let ex = Double(a[4]), let ey = Double(a[5]) else { exit(2) }
    end = CGPoint(x: ex, y: ey)
    for n in 1...10 {
        send(.leftMouseDragged, CGPoint(x: x + (ex-x)*Double(n)/10, y: y + (ey-y)*Double(n)/10))
        Thread.sleep(forTimeInterval: 0.03)
    }
}
send(.leftMouseUp, end)
print("Owned-PID pointer events posted")
