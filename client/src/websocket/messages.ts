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

interface CreateLobbyMessage {
  action: string
  name: string
}

interface JoinLobbyMessage {
  action: string
  lobby_id: string
}

interface LeaveLobbyMessage {
  action: string
}

interface ListLobbiesMessage {
  action: string
}

interface LobbyChatMessage {
  action: string
  message: string
}

export type WebSocketMessageType = ReconnectMessage | ConnectMessage | LogoutMessage | GlobalChatMessage | CreateLobbyMessage | JoinLobbyMessage | LeaveLobbyMessage | ListLobbiesMessage | LobbyChatMessage;

export const WebSocketMessage = {
  reconnect: (sessionId: string): ReconnectMessage => ({
    action: 'PlayerReconnect',
    session_id: sessionId,
  }),
  connect: (username: string): ConnectMessage => ({
    action: 'PlayerConnect',
    username,
  }),
  logout: (): LogoutMessage => ({
    action: 'PlayerLogout',
  }),
  globalChat: (message: string): GlobalChatMessage => ({
    action: 'GlobalChat',
    message,
  }),
  createLobby: (name: string): CreateLobbyMessage => ({
    action: 'LobbyCreate',
    name,
  }),
  joinLobby: (lobbyId: string): JoinLobbyMessage => ({
    action: 'LobbyJoin',
    lobby_id: lobbyId,
  }),
  leaveLobby: (): LeaveLobbyMessage => ({
    action: 'LobbyLeave',
  }),
  listLobbies: (): ListLobbiesMessage => ({
    action: 'LobbyList',
  }),
  lobbyChat: (message: string): LobbyChatMessage => ({
    action: 'LobbyChat',
    message,
  }),
};
