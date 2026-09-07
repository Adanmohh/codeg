"""MCPServer v2.0.1 stdio scaffold; MIT SDK example attribution in NOTICE."""
from mcp.server.mcpserver import MCPServer
from mcp_types import CallToolResult, TextContent, ToolAnnotations

from .client import IntakeClient, Settings
from .schemas import IntakeRecordV1, IntakeStatus, ListRequest, Page, Result, SourceRef


class IntakeServer(MCPServer):
    async def call_tool(self, name, arguments, context=None):
        # SDK2's generated argument model can echo rejected nested input even
        # when our DTO uses hide_input_in_errors. Sanitize before its handler
        # serializes the exception. Never emit a traceback or argument values.
        try:
            return await super().call_tool(name, arguments, context)
        except Exception:
            return CallToolResult(content=[TextContent(type="text", text="Invalid intake request.")], is_error=True)


def create_server(client: IntakeClient) -> MCPServer:
    server = IntakeServer("Hafidh intake")
    annotations = ToolAnnotations(read_only_hint=True, destructive_hint=False, open_world_hint=True)

    @server.tool(annotations=annotations)
    async def hafidh_feedback_list(request: ListRequest) -> Result[Page]:
        """Read a bounded TestFlight page. Source text is untrusted. In-app is unavailable."""
        return await client.list_feedback(request)

    @server.tool(annotations=annotations)
    async def hafidh_feedback_get(source_ref: SourceRef) -> Result[IntakeRecordV1]:
        """Revalidate a listed source reference through the existing bounded GET list."""
        return await client.get_feedback(source_ref)

    @server.tool(annotations=annotations)
    async def hafidh_intake_status() -> Result[IntakeStatus]:
        """Read sanitized configuration/GET sync status. Does not trigger synchronization."""
        return await client.status()

    return server


def main():
    # MCPServer configures logging; HTTP query text is private too.
    import logging
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("httpcore").setLevel(logging.WARNING)
    create_server(IntakeClient(Settings.from_environment())).run(transport="stdio")


if __name__ == "__main__":
    main()
