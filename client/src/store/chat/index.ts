import { Action, action } from 'easy-peasy'

export interface Message {
  username: string,
  timestamp: string,
  content: string,
}

export interface IUserModel {
  global: Message[]
  addGlobalMessage: Action<IUserModel, Message>
}

const chat: IUserModel = {
  global: [],
  addGlobalMessage: action((state, value) => {
    state.global.push(value)
  }),
}

export default chat
