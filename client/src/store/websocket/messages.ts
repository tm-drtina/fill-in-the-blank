export enum MessageTypes {
  ConnectionFailed = 'ConnectionFailed',
  UserConnected = 'UserConnected',
  GlobalChat = 'GlobalChat',
}

export interface ConnectionFailed {
  type: MessageTypes.ConnectionFailed
  reason: string,
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
}
