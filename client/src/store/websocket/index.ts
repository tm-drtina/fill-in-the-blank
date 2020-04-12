import { Action, action, thunk, Thunk } from 'easy-peasy';
import { setItem, USER_SESSION_UUID_KEY } from '../../utils/storage';
import { storeModel } from '../index';
import { ConnectionFailed, GlobalChat, MessageTypes, UserConnected } from './messages';

export enum WebSocketStatus {
  CONNECTED,
  CONNECTING,
  CLOSED
}

export interface IWebSocketModel {
  status: WebSocketStatus
  setStatus: Action<IWebSocketModel, WebSocketStatus>
  onMessage: Thunk<IWebSocketModel, string, void, typeof storeModel>
  onConnectionFailed: Thunk<IWebSocketModel, ConnectionFailed, void, typeof storeModel>
  onUserConnected: Thunk<IWebSocketModel, UserConnected, void, typeof storeModel>
  onGlobalChat: Thunk<IWebSocketModel, GlobalChat, void, typeof storeModel>
}

const websocket: IWebSocketModel = {
  status: WebSocketStatus.CONNECTING,
  setStatus: action((state, value) => {
    state.status = value;
  }),
  onMessage: thunk((actions, payload) => {
    const message = JSON.parse(payload);
    switch (message.type) {
      case MessageTypes.ConnectionFailed:
        actions.onConnectionFailed(message);
        break;
      case MessageTypes.UserConnected:
        actions.onUserConnected(message);
        break;
      case MessageTypes.GlobalChat:
        actions.onGlobalChat(message);
        break;
      default:
        console.warn("Unknown message", message);
    }
  }),
  onConnectionFailed: thunk((actions, payload) => {
    // TODO
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
    })
  }),
};

export default websocket;
