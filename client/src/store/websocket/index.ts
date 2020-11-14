import { Action, action, thunk, Thunk } from 'easy-peasy';
import { removeItem, setItem, USER_SESSION_UUID_KEY } from '../../utils/storage';
import { storeModel } from '../index';
import { Error, ErrorType, GlobalChat, LobbyChat, LobbyInfo, LobbyList, MessageTypes, UserConnected } from './messages';

export enum WebSocketStatus {
  CONNECTED,
  CONNECTING,
  CLOSED
}

export interface IWebSocketModel {
  status: WebSocketStatus
  setStatus: Action<IWebSocketModel, WebSocketStatus>
  onMessage: Thunk<IWebSocketModel, string, void, typeof storeModel>
  onError: Thunk<IWebSocketModel, Error, void, typeof storeModel>
  onUserConnected: Thunk<IWebSocketModel, UserConnected, void, typeof storeModel>
  onGlobalChat: Thunk<IWebSocketModel, GlobalChat, void, typeof storeModel>
  onLobbyChat: Thunk<IWebSocketModel, LobbyChat, void, typeof storeModel>
  onLobbyList: Thunk<IWebSocketModel, LobbyList, void, typeof storeModel>
  onLobbyInfo: Thunk<IWebSocketModel, LobbyInfo, void, typeof storeModel>
}

const websocket: IWebSocketModel = {
  status: WebSocketStatus.CONNECTING,
  setStatus: action((state, value) => {
    state.status = value;
  }),
  onMessage: thunk((actions, payload) => {
    const message = JSON.parse(payload);
    switch (message.type) {
      case MessageTypes.Error:
        actions.onError(message);
        break;
      case MessageTypes.UserConnected:
        actions.onUserConnected(message);
        break;
      case MessageTypes.GlobalChat:
        actions.onGlobalChat(message);
        break;
      case MessageTypes.LobbyChat:
        actions.onLobbyChat(message);
        break;
      case MessageTypes.LobbyInfo:
        actions.onLobbyInfo(message);
        break;
      case MessageTypes.LobbyList:
        actions.onLobbyList(message);
        break;
      default:
        console.warn('Unknown message', message);
    }
  }),
  onError: thunk((actions, payload, { getStoreActions }) => {
    const { lobby, user } = getStoreActions();
    switch (payload.error_type) {
      case ErrorType.Disconnected:
        user.logout();
        break;
      case ErrorType.LobbyLeft:
        lobby.failed(payload.text);
        break;
      case ErrorType.ReconnectFailed:
        user.setError(payload.text);
        break;
      case ErrorType.SessionNotFound:
        removeItem(USER_SESSION_UUID_KEY);
        user.logout();
        break;
      default:
        console.warn('Unknown error', payload);
    }
  }),
  onUserConnected: thunk((actions, payload, { getStoreActions }) => {
    const { user } = getStoreActions();
    setItem(USER_SESSION_UUID_KEY, payload.session_id);
    user.login(payload.username);
  }),
  onGlobalChat: thunk((actions, payload, { getStoreActions }) => {
    const { chat } = getStoreActions();
    chat.addGlobalMessage({
      username: payload.username,
      timestamp: payload.timestamp,
      content: payload.message,
      systemMsg: payload.system_msg,
    });
  }),
  onLobbyChat: thunk((actions, payload, { getStoreActions }) => {
    const { chat } = getStoreActions();
    chat.addLobbyMessage({
      username: payload.username,
      timestamp: payload.timestamp,
      content: payload.message,
      systemMsg: payload.system_msg,
    });
  }),
  onLobbyList: thunk((actions, payload, { getStoreActions }) => {
    const { lobby } = getStoreActions();
    lobby.setList(payload.lobbies);
  }),
  onLobbyInfo: thunk((actions, payload, { getStoreActions }) => {
    const { lobby } = getStoreActions();
    lobby.setDetail({
      lobby_id: payload.lobby_id,
      name: payload.name,
      players: payload.players,
    });
  }),
};

export default websocket;
