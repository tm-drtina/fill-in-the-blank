interface ReconnectMessage {
  action: string
  session_id: string
}

interface ConnectMessage {
  action: string
  username: string
}

interface LogoutMessage {
  action: string
}

interface GlobalChatMessage {
  action: string
  message: string
}

export type WebSocketMessageType = ReconnectMessage | ConnectMessage | LogoutMessage | GlobalChatMessage;

export const WebSocketMessage = {
  reconnect: (sessionId: string): ReconnectMessage => ({
    action: 'Reconnect',
    session_id: sessionId,
  }),
  connect: (username: string): ConnectMessage => ({
    action: 'Connect',
    username,
  }),
  logout: (): LogoutMessage => ({
    action: 'Logout',
  }),
  globalChat: (message: string): GlobalChatMessage => ({
    action: 'GlobalChat',
    message
  }),
};
