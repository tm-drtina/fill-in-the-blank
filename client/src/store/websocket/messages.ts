export enum MessageTypes {
  Error = 'Error',
  UserConnected = 'UserConnected',
  GlobalChat = 'GlobalChat',
  LobbyChat = 'LobbyChat',
  LobbyList = 'LobbyList',
  LobbyInfo = 'LobbyInfo',
}

export enum ErrorType {
  Disconnected = 'Disconnected',
  SessionNotFound = 'SessionNotFound',
  ReconnectFailed = 'ReconnectFailed',
  LobbyLeft = 'LobbyLeft',
}

export interface Error {
  type: MessageTypes.Error,
  timestamp: string
  error_type: ErrorType
  text: string
}

export interface UserConnected {
  type: MessageTypes.UserConnected,
  session_id: string,
  username: string,
}

export interface GlobalChat {
  type: MessageTypes.GlobalChat,
  timestamp: string,
  username: string,
  message: string,
  system_msg: boolean,
}

export interface LobbyChat {
  type: MessageTypes.LobbyChat,
  timestamp: string,
  username: string,
  message: string,
  system_msg: boolean,
}

export interface LobbyOverview {
  lobby_id: string,
  name: string,
  player_count: number,
}

export interface LobbyList {
  type: MessageTypes.LobbyList,
  lobbies: LobbyOverview[],
}

export interface LobbyDetail {
  lobby_id: string,
  name: string,
  players: string[],
}

export interface LobbyInfo {
  type: MessageTypes.LobbyInfo,
  lobby_id: string,
  name: string,
  players: string[],
}
