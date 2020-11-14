import { Action, action } from 'easy-peasy';
import { LobbyDetail, LobbyOverview } from '../websocket/messages';

export enum LobbyStatus {
  NO_LOBBY,
  JOINING,
  JOINED,
  FAILED,
}

export interface ILobbyModel {
  status: LobbyStatus
  list: LobbyOverview[] | undefined
  detail: LobbyDetail | undefined
  failureReason: string | undefined

  reset: Action<ILobbyModel>
  failed: Action<ILobbyModel, string>
  setStatus: Action<ILobbyModel, LobbyStatus>
  setList: Action<ILobbyModel, LobbyOverview[] | undefined>
  setDetail: Action<ILobbyModel, LobbyDetail>
}

const lobby: ILobbyModel = {
  status: LobbyStatus.NO_LOBBY,
  detail: undefined,
  list: undefined,
  failureReason: undefined,

  setStatus: action((state, value) => {
    state.status = value;
  }),
  setList: action((state, value) => {
    state.list = value;
  }),
  setDetail: action((state, value) => {
    state.detail = value;
    state.status = LobbyStatus.JOINED;
  }),
  reset: action(((state) => {
    state.status = LobbyStatus.NO_LOBBY;
    state.list = undefined;
    state.detail = undefined;
    state.failureReason = undefined;
  })),
  failed: action(((state, payload) => {
    state.status = LobbyStatus.FAILED;
    state.failureReason = payload;
  })),
};

export default lobby;
