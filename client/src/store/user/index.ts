import { Action, action } from 'easy-peasy'

export enum UserStatus {
  CONNECTING,
  LOGGED_OUT,
  LOGGED_IN,
}

export interface IUserModel {
  status: UserStatus
  username?: String
  lastError?: String

  setError: Action<IUserModel, String>
  login: Action<IUserModel, String>
  logout: Action<IUserModel>
  connecting: Action<IUserModel>
}

const user: IUserModel = {
  status: UserStatus.LOGGED_OUT,
  username: undefined,
  lastError: undefined,
  login: action((state, username) => {
    state.username = username
    state.status = UserStatus.LOGGED_IN
    state.lastError = undefined
  }),
  logout: action((state) => {
    state.username = undefined
    state.status = UserStatus.LOGGED_OUT
    state.lastError = undefined
  }),
  connecting: action((state) => {
    state.username = undefined
    state.status = UserStatus.CONNECTING
    state.lastError = undefined
  }),
  setError: action((state, error) => {
    state.username = undefined
    state.status = UserStatus.LOGGED_OUT
    state.lastError = error
  }),
}

export default user
