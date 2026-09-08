// Local acceptance helper, not product code. Use only the PID of the isolated
// native app launched by this review. Apple SDK AXUIElement.h is the API source.
// Editable values are excluded from dumps; set values arrive on stdin only.
import Foundation
import ApplicationServices
import CoreGraphics

let args = CommandLine.arguments
guard args.count >= 3, let pid = Int32(args[1]), pid > 0,
      AXIsProcessTrusted() else { exit(2) }
let app = AXUIElementCreateApplication(pid)
func attr(_ node: AXUIElement, _ key: String) -> CFTypeRef? {
    var value: CFTypeRef?
    guard AXUIElementCopyAttributeValue(node, key as CFString, &value) == .success else { return nil }
    return value
}
func children(_ node: AXUIElement) -> [AXUIElement] {
    (attr(node, kAXChildrenAttribute) as? [AXUIElement]) ?? []
}
func safe(_ value: String) -> String {
    value.replacingOccurrences(of: "bdm_[A-Za-z0-9_-]+", with: "[redacted]", options: .regularExpression)
}
func emit(_ value: Any) {
    if let data = try? JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]),
       let text = String(data: data, encoding: .utf8) { print(text) }
}
if args[2] == "windows" {
    let all = CGWindowListCopyWindowInfo(.optionAll, kCGNullWindowID) as? [[String: Any]] ?? []
    emit(all.filter { ($0[kCGWindowOwnerPID as String] as? Int32) == pid }.map {
        ["id": $0[kCGWindowNumber as String] ?? 0,
         "name": $0[kCGWindowName as String] ?? "",
         "bounds": $0[kCGWindowBounds as String] ?? [:]]
    })
} else if args[2] == "restore" {
    guard let window = children(app).first(where: { attr($0, kAXRoleAttribute) as? String == "AXWindow" }) else { exit(2) }
    let result = AXUIElementSetAttributeValue(window, kAXMinimizedAttribute as CFString, kCFBooleanFalse)
    emit(["action": "restore", "result": result.rawValue])
    if result != .success { exit(1) }
} else if args[2] == "resize" {
    guard args.count == 5, let w = Double(args[3]), let h = Double(args[4]),
          w >= 400, h >= 600, w <= 1400, h <= 950,
          let window = children(app).first(where: { attr($0, kAXRoleAttribute) as? String == "AXWindow" }) else { exit(2) }
    var size = CGSize(width: w, height: h)
    guard let value = AXValueCreate(.cgSize, &size) else { exit(3) }
    let result = AXUIElementSetAttributeValue(window, kAXSizeAttribute as CFString, value)
    emit(["action": "resize", "result": result.rawValue])
    if result != .success { exit(1) }
} else if args[2] == "dump" {
    var rows: [[String: Any]] = []
    func walk(_ node: AXUIElement, _ path: [Int], _ depth: Int) {
        guard depth <= 30, rows.count < 1000 else { return }
        let role = attr(node, kAXRoleAttribute) as? String ?? ""
        guard role != "AXMenuBar" else { return }
        var row: [String: Any] = ["path": path.map(String.init).joined(separator: "."), "role": role]
        if let v = attr(node, kAXPositionAttribute), CFGetTypeID(v) == AXValueGetTypeID() {
            var point = CGPoint.zero
            if AXValueGetValue(v as! AXValue, .cgPoint, &point) { row["x"] = point.x; row["y"] = point.y }
        }
        if let v = attr(node, kAXSizeAttribute), CFGetTypeID(v) == AXValueGetTypeID() {
            var size = CGSize.zero
            if AXValueGetValue(v as! AXValue, .cgSize, &size) { row["width"] = size.width; row["height"] = size.height }
        }
        if let enabled = attr(node, kAXEnabledAttribute) as? Bool { row["enabled"] = enabled }
        if let minimized = attr(node, kAXMinimizedAttribute) as? Bool { row["minimized"] = minimized }
        if let expanded = attr(node, kAXExpandedAttribute) as? Bool { row["expanded"] = expanded }
        for key in [kAXTitleAttribute, kAXDescriptionAttribute] {
            if let text = attr(node, key) as? String, !text.isEmpty { row[key] = safe(text) }
        }
        if role == "AXStaticText", let text = attr(node, kAXValueAttribute) as? String { row["text"] = safe(text) }
        if role == "AXWebArea", let url = attr(node, kAXURLAttribute) as? URL {
            // Record only route identity, never URL credentials/query/fragment.
            row["route"] = url.path
            row["scheme"] = url.scheme ?? ""
        }
        rows.append(row)
        for (index, child) in children(node).enumerated() { walk(child, path + [index], depth + 1) }
    }
    walk(app, [], 0)
    emit(rows)
} else {
    guard args.count == 4 else { exit(2) }
    var node = app
    for part in args[3].split(separator: ".") {
        guard let index = Int(part) else { exit(2) }
        let nodes = children(node)
        guard nodes.indices.contains(index) else { exit(3) }
        node = nodes[index]
    }
    let result: AXError
    switch args[2] {
    case "press": result = AXUIElementPerformAction(node, kAXPressAction as CFString)
    case "set":
        guard let value = readLine() else { exit(2) }
        result = AXUIElementSetAttributeValue(node, kAXValueAttribute as CFString, value as CFString)
    default: exit(2)
    }
    emit(["action": args[2], "result": result.rawValue])
    if result != .success { exit(1) }
}
