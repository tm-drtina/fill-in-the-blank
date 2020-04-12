import { Action, action } from 'easy-peasy'

export enum UserStatus {
  RECONNECTING,
  LOGGED_OUT,
  LOGGED_IN,
}

export interface IUserModel {
  status: UserStatus
  username?: String
  setStatus: Action<IUserModel, UserStatus>
  login: Action<IUserModel, string>
}

const user: IUserModel = {
  status: UserStatus.LOGGED_OUT,
  username: undefined,
  setStatus: action((state, value) => {
    state.status = value
  }),
  login: action((state, username) => {
    state.username = username
    state.status = UserStatus.LOGGED_IN
  }),

}

export default user
